use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const CACHE_FILENAME: &str = "models-dev-cache.json";
const CACHE_MAX_AGE_HOURS: i64 = 24;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsCache {
    pub fetched_at: DateTime<Utc>,
    pub providers: Vec<ProviderInfo>,
    pub api_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    pub id: String,
    pub name: String,
    pub npm: String,
    pub api_url: String,
    pub doc: Option<String>,
    pub env: Vec<String>,
    pub models: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider_id: String,
    pub family: Option<String>,

    pub capabilities: ModelCapabilities,

    pub context_limit: u32,
    pub output_limit: u32,

    pub cost_input: f64,
    pub cost_output: f64,
    pub cost_cache_read: Option<f64>,
    pub cost_cache_write: Option<f64>,
    pub cost_over_200k_input: Option<f64>,
    pub cost_over_200k_output: Option<f64>,

    pub release_date: String,
    pub status: ModelStatus,
    pub options: Option<Value>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilities {
    pub temperature: bool,
    pub reasoning: bool,
    pub attachment: bool,
    pub tool_call: bool,
    pub input: ModalityCapabilities,
    pub output: ModalityCapabilities,
    pub interleaved: InterleavedConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModalityCapabilities {
    pub text: bool,
    pub audio: bool,
    pub image: bool,
    pub video: bool,
    pub pdf: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InterleavedConfig {
    Enabled(bool),
    Field { field: String },
}

impl Default for InterleavedConfig {
    fn default() -> Self {
        InterleavedConfig::Enabled(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelStatus {
    Alpha,
    Beta,
    Deprecated,
    Active,
}

impl Default for ModelStatus {
    fn default() -> Self {
        ModelStatus::Active
    }
}

pub struct CacheManager {
    cache_dir: PathBuf,
}

impl CacheManager {
    pub fn new(cache_dir: &Path) -> Self {
        Self {
            cache_dir: cache_dir.to_path_buf(),
        }
    }

    pub fn cache_path(&self) -> PathBuf {
        self.cache_dir.join(CACHE_FILENAME)
    }

    pub fn get_or_fetch(&self) -> Result<ModelsCache> {
        if let Some(cached) = self.load_cached()? {
            if self.is_cache_fresh(&cached) {
                return Ok(cached);
            }
        }

        self.fetch_and_cache()
    }

    pub fn fetch_and_cache(&self) -> Result<ModelsCache> {
        println!("Fetching available models from models.dev...");

        let api_url = "https://models.dev/api.json";
        let response = ureq::get(api_url)
            .call()
            .context("Failed to fetch models from models.dev API")?;

        let text = response
            .into_string()
            .context("Failed to read response body")?;

        let providers = self.parse_models_response(&text)?;

        let cache = ModelsCache {
            fetched_at: Utc::now(),
            providers,
            api_url: api_url.to_string(),
        };

        self.save_cache(&cache)?;

        println!("Fetched {} providers", cache.providers.len());
        Ok(cache)
    }

    pub fn load_cached(&self) -> Result<Option<ModelsCache>> {
        let path = self.cache_path();

        if !path.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read cache file: {}", path.display()))?;

        let cache: ModelsCache = serde_json::from_str(&contents)
            .context("Failed to parse cache file")?;

        Ok(Some(cache))
    }

    fn save_cache(&self, cache: &ModelsCache) -> Result<()> {
        let path = self.cache_path();
        let contents = serde_json::to_string_pretty(cache)
            .context("Failed to serialize cache")?;

        fs::write(&path, contents)
            .with_context(|| format!("Failed to write cache file: {}", path.display()))?;

        Ok(())
    }

    fn is_cache_fresh(&self, cache: &ModelsCache) -> bool {
        let age = Utc::now() - cache.fetched_at;
        age < Duration::hours(CACHE_MAX_AGE_HOURS)
    }

    fn parse_models_response(&self, text: &str) -> Result<Vec<ProviderInfo>> {
        let api_data: Value = serde_json::from_str(text)
            .context("Failed to parse models.dev API response")?;

        let mut providers = Vec::new();

        if let Some(obj) = api_data.as_object() {
            for (provider_id, provider_data) in obj {
                let provider_name = provider_data["name"]
                    .as_str()
                    .unwrap_or(provider_id);

                let npm = provider_data["npm"]
                    .as_str()
                    .unwrap_or("");

                let api_url = provider_data["api"]
                    .as_str()
                    .unwrap_or("");

                let doc = provider_data["doc"].as_str().map(|s| s.to_string());

                let env = provider_data["env"]
                    .as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_default();

                let models = if let Some(models_obj) = provider_data["models"].as_object() {
                    models_obj
                        .values()
                        .filter_map(|m| self.parse_model_info(provider_id, npm, api_url, m))
                        .collect()
                } else {
                    Vec::new()
                };

                providers.push(ProviderInfo {
                    id: provider_id.clone(),
                    name: provider_name.to_string(),
                    npm: npm.to_string(),
                    api_url: api_url.to_string(),
                    doc,
                    env,
                    models,
                });
            }
        }

        Ok(providers)
    }

    fn parse_model_info(
        &self,
        provider_id: &str,
        provider_npm: &str,
        provider_api: &str,
        value: &Value,
    ) -> Option<ModelInfo> {
        let limit = value.get("limit").and_then(|v| v.as_object());
        let cost = value.get("cost").and_then(|v| v.as_object());
        let modalities = value.get("modalities").and_then(|v| v.as_object());

        Some(ModelInfo {
            id: value["id"].as_str()?.to_string(),
            name: value["name"].as_str()?.to_string(),
            provider_id: provider_id.to_string(),
            family: value.get("family").and_then(|v| v.as_str()).map(|s| s.to_string()),

            capabilities: ModelCapabilities {
                temperature: value.get("temperature").and_then(|v| v.as_bool()).unwrap_or(false),
                reasoning: value.get("reasoning").and_then(|v| v.as_bool()).unwrap_or(false),
                attachment: value.get("attachment").and_then(|v| v.as_bool()).unwrap_or(false),
                tool_call: value.get("tool_call").and_then(|v| v.as_bool()).unwrap_or(true),
                input: self.parse_modalities(modalities, "input"),
                output: self.parse_modalities(modalities, "output"),
                interleaved: self.parse_interleaved(value),
            },

            context_limit: limit
                .and_then(|l| l.get("context").and_then(|v| v.as_u64()).map(|v| v as u32))
                .unwrap_or(0),
            output_limit: limit
                .and_then(|l| l.get("output").and_then(|v| v.as_u64()).map(|v| v as u32))
                .unwrap_or(0),

            cost_input: cost
                .and_then(|c| c.get("input").and_then(|v| v.as_f64()))
                .unwrap_or(0.0),
            cost_output: cost
                .and_then(|c| c.get("output").and_then(|v| v.as_f64()))
                .unwrap_or(0.0),
            cost_cache_read: cost
                .and_then(|c| c.get("cache_read").and_then(|v| v.as_f64())),
            cost_cache_write: cost
                .and_then(|c| c.get("cache_write").and_then(|v| v.as_f64())),
            cost_over_200k_input: cost
                .and_then(|c| c.get("context_over_200k"))
                .and_then(|o| o.get("input").and_then(|v| v.as_f64())),
            cost_over_200k_output: cost
                .and_then(|c| c.get("context_over_200k"))
                .and_then(|o| o.get("output").and_then(|v| v.as_f64())),

            release_date: value
                .get("release_date")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),

            status: value
                .get("status")
                .and_then(|s| s.as_str())
                .and_then(|s| match s {
                    "alpha" => Some(ModelStatus::Alpha),
                    "beta" => Some(ModelStatus::Beta),
                    "deprecated" => Some(ModelStatus::Deprecated),
                    "active" => Some(ModelStatus::Active),
                    _ => None,
                })
                .unwrap_or_default(),

            options: value.get("options").cloned(),
            headers: value
                .get("headers")
                .cloned()
                .and_then(|h| serde_json::from_value(h).ok()),
        })
    }

    fn parse_modalities(&self, modalities: Option<&serde_json::Map<String, Value>>, key: &str) -> ModalityCapabilities {
        let obj = modalities
            .and_then(|m| m.get(key))
            .and_then(|v| v.as_object());

        let array = modalities
            .and_then(|m| m.get(key))
            .and_then(|v| v.as_array());

        let mut caps = ModalityCapabilities::default();

        if let Some(arr) = array {
            for item in arr {
                if let Some(s) = item.as_str() {
                    match s {
                        "text" => caps.text = true,
                        "audio" => caps.audio = true,
                        "image" => caps.image = true,
                        "video" => caps.video = true,
                        "pdf" => caps.pdf = true,
                        _ => {}
                    }
                }
            }
        } else if let Some(obj) = obj {
            caps.text = obj.get("text").and_then(|v| v.as_bool()).unwrap_or(false);
            caps.audio = obj.get("audio").and_then(|v| v.as_bool()).unwrap_or(false);
            caps.image = obj.get("image").and_then(|v| v.as_bool()).unwrap_or(false);
            caps.video = obj.get("video").and_then(|v| v.as_bool()).unwrap_or(false);
            caps.pdf = obj.get("pdf").and_then(|v| v.as_bool()).unwrap_or(false);
        }

        caps
    }

    fn parse_interleaved(&self, value: &Value) -> InterleavedConfig {
        if let Some(interleaved) = value.get("interleaved") {
            if let Some(s) = interleaved.as_str() {
                if s == "reasoning_content" || s == "reasoning_details" {
                    return InterleavedConfig::Field {
                        field: s.to_string(),
                    };
                }
            }
            if let Some(b) = interleaved.as_bool() {
                return InterleavedConfig::Enabled(b);
            }
        }
        InterleavedConfig::Enabled(false)
    }

    pub fn refresh(&self) -> Result<ModelsCache> {
        self.fetch_and_cache()
    }
}
