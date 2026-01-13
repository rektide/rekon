use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration as StdDuration;

const CACHE_FILENAME: &str = "models-dev-cache.json";
const CACHE_MAX_AGE_HOURS: i64 = 24;

const MAX_RETRIES: u32 = 3;
const BASE_RETRY_DELAY_MS: u64 = 1000;
const MAX_RETRY_DELAY_MS: u64 = 10000;

fn retry_with_backoff<F, T>(mut operation: F) -> Result<T>
where
    F: FnMut() -> Result<T>,
{
    let mut last_error = None;

    for attempt in 0..MAX_RETRIES {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();

                let should_retry = error_msg.contains("connect")
                    || error_msg.contains("timeout")
                    || error_msg.contains("network")
                    || error_msg.contains("io");

                if !should_retry {
                    return Err(e);
                }

                last_error = Some(e);

                if attempt < MAX_RETRIES - 1 {
                    let delay_ms = (BASE_RETRY_DELAY_MS * 2u64.pow(attempt))
                        .min(MAX_RETRY_DELAY_MS);
                    let jitter = rand::thread_rng().gen_range(0..=delay_ms / 4);
                    let total_delay = delay_ms + jitter;

                    eprintln!(
                        "Request failed (attempt {}/{}), retrying in {}ms...",
                        attempt + 1,
                        MAX_RETRIES,
                        total_delay
                    );
                    thread::sleep(StdDuration::from_millis(total_delay));
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        anyhow::anyhow!("Operation failed after {} retries", MAX_RETRIES)
    }))
}

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

        let agent = ureq::AgentBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("oc-variance/0.1.0")
            .build();

        let response = retry_with_backoff(|| {
            agent
                .get(api_url)
                .call()
                .map_err(|e| {
                    let error_msg = e.to_string().to_lowercase();
                    if error_msg.contains("connect") || error_msg.contains("connection") {
                        anyhow::anyhow!("Failed to connect to models.dev API. Check your internet connection.")
                    } else if error_msg.contains("dns") {
                        anyhow::anyhow!("DNS resolution failed for models.dev API.")
                    } else if error_msg.contains("timeout") {
                        anyhow::anyhow!("Request to models.dev API timed out after 30 seconds.")
                    } else if error_msg.contains("io") || error_msg.contains("network") {
                        anyhow::anyhow!("Network error while fetching from models.dev API: {}", e)
                    } else {
                        anyhow::anyhow!("Failed to fetch models from models.dev API: {}", e)
                    }
                })
        })?;

        let status_code = response.status();
        if !(200..=299).contains(&status_code) {
            return Err(anyhow::anyhow!(
                "models.dev API returned error status {}: {}",
                status_code,
                response.status_text()
            ));
        }

        let text = response
            .into_string()
            .with_context(|| format!("Failed to read response body from {}", api_url))?;

        if text.is_empty() {
            return Err(anyhow::anyhow!(
                "Received empty response from models.dev API"
            ));
        }

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
            .with_context(|| {
                format!(
                    "Failed to read cache file: {}. Check file permissions.",
                    path.display()
                )
            })?;

        if contents.is_empty() {
            return Err(anyhow::anyhow!(
                "Cache file {} is empty. Consider deleting it to force a fresh fetch.",
                path.display()
            ));
        }

        let cache: ModelsCache = serde_json::from_str(&contents).with_context(|| {
            format!(
                "Cache file {} is corrupted or invalid. Delete it to force a fresh fetch. Error: {}",
                path.display(),
                contents.chars().take(200).collect::<String>()
            )
        })?;

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
        let api_data: Value = serde_json::from_str(text).with_context(|| {
            "Failed to parse models.dev API response. The API format may have changed."
        })?;

        let obj = api_data
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("API response is not a JSON object as expected"))?;

        if obj.is_empty() {
            return Err(anyhow::anyhow!(
                "API response contains no providers. This may be a temporary API issue."
            ));
        }

        let mut providers = Vec::new();

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
                if models_obj.is_empty() {
                    eprintln!("Warning: Provider '{}' has no models defined", provider_id);
                }
                models_obj
                    .values()
                    .filter_map(|m| self.parse_model_info(provider_id, npm, api_url, m))
                    .collect()
            } else {
                eprintln!("Warning: Provider '{}' has no models field", provider_id);
                Vec::new()
            };

            if models.is_empty() && !npm.is_empty() {
                eprintln!("Warning: Provider '{}' ({}) has no valid models", provider_id, npm);
            }

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

        if providers.is_empty() {
            return Err(anyhow::anyhow!(
                "Failed to parse any providers from API response"
            ));
        }

        Ok(providers)
    }

    fn parse_model_info(
        &self,
        provider_id: &str,
        _provider_npm: &str,
        _provider_api: &str,
        value: &Value,
    ) -> Option<ModelInfo> {
        let id = value.get("id")?.as_str()?;

        if id.is_empty() {
            return None;
        }

        let name = value.get("name")?.as_str()?;

        let limit = value.get("limit").and_then(|v| v.as_object());
        let cost = value.get("cost").and_then(|v| v.as_object());
        let modalities = value.get("modalities").and_then(|v| v.as_object());

        let context_limit = limit
            .and_then(|l| l.get("context").and_then(|v| v.as_u64()))
            .map(|v| v as u32)
            .unwrap_or(0);

        let output_limit = limit
            .and_then(|l| l.get("output").and_then(|v| v.as_u64()))
            .map(|v| v as u32)
            .unwrap_or(0);

        if context_limit == 0 && output_limit == 0 {
            return None;
        }

        Some(ModelInfo {
            id: id.to_string(),
            name: name.to_string(),
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

            context_limit,
            output_limit,

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
