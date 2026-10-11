use std::fs;
use std::path::Path;

use super::{Format, Result};

pub fn run(file: &Path, scope: Option<&str>, format: Format) -> Result {
    let source = fs::read_to_string(file)
        .map_err(|error| format!("{}: {error}", file.display()))?;
    let mut records = rekon_markup::extract(&source)
        .map_err(|error| format!("{}: {error}", file.display()))?;
    if let Some(scope) = scope {
        let id = scope.strip_prefix('#').unwrap_or(scope);
        records.retain(|record| record.owner.id.as_deref() == Some(id));
        if records.is_empty() {
            return Err(format!("{}: no metadata owned by scope {scope:?}", file.display()).into());
        }
    }
    match format {
        Format::Json => println!("{}", serde_json::to_string_pretty(&records)?),
        Format::Yaml => print!("{}", serde_yaml_ng::to_string(&records)?),
    }
    Ok(true)
}
