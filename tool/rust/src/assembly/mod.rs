//! Enriched whole-document assembly.
//!
//! [`assemble`] loads the explicit manifest selection, validates every fragment
//! (identity, order, source, status, body shape, relocation safety, anchor IDs),
//! and renders the enriched document with scoped `Okf` metadata and nested
//! semantic sections — all before any filesystem effect. [`write_assembly`]
//! publishes complete bytes atomically through a same-directory staging file;
//! [`check_assembly`] compares published bytes without writing. Validation
//! finishes before any output directory or file is created, and failures never
//! touch the output.
//!
//! The assembled document is caller-owned between assembly and publication:
//! the CLI mutates [`Assembly::document`] with the final ToC pass, then writes
//! or checks those exact bytes. Current design:
//! `/design/toolchain/enriched-assembly0.gpt6a.md`; production behavior and
//! guarantees: `/tool/assembly/README.md`.

pub mod document;
pub mod fragment;
mod transform;

pub use document::{CheckResult, DOCUMENT_TITLE, GENERATED_NOTICE, compare_document, render_document};
pub use fragment::{Fragment, FragmentError, FragmentMeta, FragmentStatus, RESERVED_TOC_ID, parse_fragment};
pub(crate) use transform::parse_options;

use std::collections::HashMap;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::manifest::{DEFAULT_MANIFEST, ManifestError, expand_globs, parse_manifest, plan_assembly};
use crate::nodepath;

/// One ordered fragment's context cost, for human assembly reports.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FragmentReport {
    pub id: String,
    pub order: i64,
    pub path: String,
    /// UTF-8 bytes of the fragment's source prose (not emitted wrappers).
    pub prose_bytes: usize,
}

/// A completed, write-free assembly. `document` stays caller-mutable (the ToC
/// pass) until [`write_assembly`] or [`check_assembly`] consumes it.
#[derive(Debug)]
pub struct Assembly {
    pub document: String,
    pub output: PathBuf,
    /// Absolute manifest and fragment paths the output must never alias.
    pub inputs: Vec<PathBuf>,
    /// Fragments in metadata order.
    pub fragments: Vec<Fragment>,
}

impl Assembly {
    /// UTF-8 bytes of the final document, accurate after caller mutation.
    pub fn total_bytes(&self) -> usize {
        self.document.len()
    }

    /// Ordered per-fragment report rows.
    pub fn fragment_reports(&self) -> Vec<FragmentReport> {
        self.fragments
            .iter()
            .map(|fragment| FragmentReport {
                id: fragment.meta.id.clone(),
                order: fragment.meta.order,
                path: fragment.path.clone(),
                prose_bytes: fragment.prose_bytes,
            })
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum AssemblyError {
    #[error(transparent)]
    Manifest(#[from] ManifestError),
    #[error(transparent)]
    Fragment(#[from] FragmentError),
    #[error("duplicate fragment id \"{id}\" in {path} and {prior}")]
    DuplicateFragmentId { id: String, path: String, prior: String },
    #[error("duplicate fragment order {order} in {path} and {prior}")]
    DuplicateFragmentOrder { order: i64, path: String, prior: String },
    #[error("fragments {path} and {prior} both define explicit id \"{id}\"; ids must be unique across the assembled document")]
    DuplicateAnchorId { id: String, path: String, prior: String },
    #[error("{path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("path is not valid UTF-8: {path}")]
    InvalidPath { path: String },
    #[error("assembly output {output} is a symlink; refusing to publish through it")]
    OutputSymlink { output: PathBuf },
    #[error("assembly output {output} aliases input {input} through the filesystem; refusing to overwrite")]
    OutputProtected { output: PathBuf, input: PathBuf },
    #[error("failed to publish {output}: {source}")]
    Publication {
        output: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

fn path_string(path: &Path) -> Result<&str, AssemblyError> {
    path.to_str().ok_or_else(|| AssemblyError::InvalidPath {
        path: path.display().to_string(),
    })
}

fn read_file(path: &Path) -> Result<String, AssemblyError> {
    fs::read_to_string(path).map_err(|source| AssemblyError::Io {
        path: path.display().to_string(),
        source,
    })
}

/// Read the explicit selection, validate it entirely, and render — no writes.
/// Fragments resolve from the manifest's directory; an output override resolves
/// from `cwd`. All validation, including cross-fragment identity and anchor
/// uniqueness, finishes before anything is returned.
pub fn assemble(manifest_path: &Path, cwd: &Path, output_override: Option<&Path>) -> Result<Assembly, AssemblyError> {
    let manifest_name = path_string(manifest_path)?;
    let working_directory = path_string(cwd)?;
    let override_path = match output_override {
        Some(path) => Some(path_string(path)?),
        None => None,
    };
    let manifest_file = cwd.join(manifest_path);
    let raw = match fs::read_to_string(&manifest_file) {
        Ok(raw) => raw,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && manifest_path == Path::new(DEFAULT_MANIFEST) => "{}".to_string(),
        Err(source) => {
            return Err(AssemblyError::Io {
                path: manifest_file.display().to_string(),
                source,
            });
        }
    };
    let manifest = parse_manifest(&raw, manifest_name)?;
    let expanded = expand_globs(&manifest, &nodepath::dirname(&nodepath::resolve(working_directory, manifest_name)))?;
    let plan = plan_assembly(&expanded, manifest_name, working_directory, override_path)?;

    let mut fragments = Vec::with_capacity(plan.fragments.len());
    let mut inputs = vec![PathBuf::from(&plan.manifest_path)];
    for declared in &plan.fragments {
        let raw = read_file(Path::new(&declared.absolute))?;
        fragments.push(parse_fragment(&raw, &declared.path)?);
        inputs.push(PathBuf::from(&declared.absolute));
    }

    let mut ids: HashMap<&str, &str> = HashMap::new();
    let mut orders: HashMap<i64, &str> = HashMap::new();
    let mut anchors: HashMap<&str, &str> = HashMap::new();
    for fragment in &fragments {
        if let Some(prior) = ids.get(fragment.meta.id.as_str()) {
            return Err(AssemblyError::DuplicateFragmentId {
                id: fragment.meta.id.clone(),
                path: fragment.path.clone(),
                prior: (*prior).to_string(),
            });
        }
        if let Some(prior) = orders.get(&fragment.meta.order) {
            return Err(AssemblyError::DuplicateFragmentOrder {
                order: fragment.meta.order,
                path: fragment.path.clone(),
                prior: (*prior).to_string(),
            });
        }
        for id in &fragment.anchor_ids {
            if let Some(prior) = anchors.get(id.as_str()) {
                return Err(AssemblyError::DuplicateAnchorId {
                    id: id.clone(),
                    path: fragment.path.clone(),
                    prior: (*prior).to_string(),
                });
            }
            anchors.insert(id, &fragment.path);
        }
        ids.insert(&fragment.meta.id, &fragment.path);
        orders.insert(fragment.meta.order, &fragment.path);
    }
    fragments.sort_by_key(|fragment| fragment.meta.order);

    let document = render_document(&fragments)?;
    Ok(Assembly {
        document,
        output: PathBuf::from(plan.output),
        inputs,
        fragments,
    })
}

/// Publish the (possibly caller-mutated) document atomically: stage a complete
/// file in the output's parent, then rename it into place. Symlinked outputs
/// and filesystem aliases onto declared inputs are refused.
pub fn write_assembly(assembly: &Assembly) -> Result<(), AssemblyError> {
    let output = &assembly.output;
    if let Ok(metadata) = fs::symlink_metadata(output)
        && metadata.file_type().is_symlink()
    {
        return Err(AssemblyError::OutputSymlink { output: output.clone() });
    }
    if let Ok(published) = fs::canonicalize(output) {
        for input in &assembly.inputs {
            if fs::canonicalize(input).is_ok_and(|real| real == published) {
                return Err(AssemblyError::OutputProtected {
                    output: output.clone(),
                    input: input.clone(),
                });
            }
        }
    }
    let parent = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|source| AssemblyError::Io {
        path: parent.display().to_string(),
        source,
    })?;
    let mut staging = tempfile::Builder::new()
        .prefix(".rekon-combine-")
        .tempfile_in(parent)
        .map_err(|source| AssemblyError::Publication {
            output: output.clone(),
            source,
        })?;
    staging.write_all(assembly.document.as_bytes()).map_err(|source| AssemblyError::Publication {
        output: output.clone(),
        source,
    })?;
    staging.persist(output).map_err(|error| AssemblyError::Publication {
        output: output.clone(),
        source: error.error,
    })?;
    Ok(())
}

/// Compare the document against the published bytes without writing. A missing
/// output is stale; every other read failure keeps its platform context.
pub fn check_assembly(assembly: &Assembly) -> Result<CheckResult, AssemblyError> {
    match fs::read(&assembly.output) {
        Ok(actual) => Ok(compare_document(&assembly.document, &actual)),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(CheckResult::Missing),
        Err(source) => Err(AssemblyError::Io {
            path: assembly.output.display().to_string(),
            source,
        }),
    }
}
