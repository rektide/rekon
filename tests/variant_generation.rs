use oc_variance::cache::{CacheManager, ModelInfo};
use oc_variance::models::{generate_variants, ModelCapabilities};

#[test]
fn test_variants_for_anthropic() {
    let model = ModelInfo {
        id: "claude-sonnet-4-5-20250929".to_string(),
        name: "Claude Sonnet 4.5".to_string(),
        provider_id: "anthropic".to_string(),
        family: None,
        capabilities: oc_variance::cache::ModelCapabilities {
            temperature: true,
            reasoning: true,
            attachment: true,
            tool_call: true,
            input: Default::default(),
            output: Default::default(),
            interleaved: oc_variance::cache::InterleavedConfig::Enabled(false),
        },
        context_limit: 200000,
        output_limit: 8192,
        cost_input: 0.003,
        cost_output: 0.015,
        cost_cache_read: Some(0.0003),
        cost_cache_write: Some(0.00375),
        cost_over_200k_input: None,
        cost_over_200k_output: None,
        release_date: "2024-09-29".to_string(),
        status: oc_variance::cache::ModelStatus::Active,
        options: None,
        headers: None,
    };

    let variants = generate_variants(&model, "@ai-sdk/anthropic");

    assert!(!variants.is_empty(), "Anthropic reasoning models should have variants");
    assert!(variants.contains_key("high"), "Anthropic should have 'high' variant");
    assert!(variants.contains_key("max"), "Anthropic should have 'max' variant");

    let high_config = variants.get("high").unwrap();
    assert_eq!(
        high_config["thinking"]["type"],
        "enabled",
        "Anthropic 'high' variant should enable thinking"
    );
    assert_eq!(
        high_config["thinking"]["budgetTokens"],
        16000,
        "Anthropic 'high' variant should have 16000 budget tokens"
    );

    let max_config = variants.get("max").unwrap();
    assert_eq!(
        max_config["thinking"]["budgetTokens"],
        31999,
        "Anthropic 'max' variant should have 31999 budget tokens"
    );
}

#[test]
fn test_variants_for_openai() {
    let model = ModelInfo {
        id: "gpt-4o".to_string(),
        name: "GPT-4o".to_string(),
        provider_id: "openai".to_string(),
        family: None,
        capabilities: oc_variance::cache::ModelCapabilities {
            temperature: true,
            reasoning: true,
            attachment: true,
            tool_call: true,
            input: Default::default(),
            output: Default::default(),
            interleaved: oc_variance::cache::InterleavedConfig::Enabled(false),
        },
        context_limit: 128000,
        output_limit: 4096,
        cost_input: 0.005,
        cost_output: 0.015,
        cost_cache_read: None,
        cost_cache_write: None,
        cost_over_200k_input: None,
        cost_over_200k_output: None,
        release_date: "2024-01-01".to_string(),
        status: oc_variance::cache::ModelStatus::Active,
        options: None,
        headers: None,
    };

    let variants = generate_variants(&model, "@ai-sdk/openai");

    assert!(!variants.is_empty(), "OpenAI reasoning models should have variants");
    assert!(variants.contains_key("low"), "OpenAI should have 'low' variant");
    assert!(variants.contains_key("medium"), "OpenAI should have 'medium' variant");
    assert!(variants.contains_key("high"), "OpenAI should have 'high' variant");

    let low_config = variants.get("low").unwrap();
    assert_eq!(
        low_config["reasoningEffort"],
        "low",
        "OpenAI 'low' variant should have reasoningEffort='low'"
    );
}

#[test]
fn test_variants_for_non_reasoning_model() {
    let model = ModelInfo {
        id: "gpt-3.5-turbo".to_string(),
        name: "GPT-3.5 Turbo".to_string(),
        provider_id: "openai".to_string(),
        family: None,
        capabilities: oc_variance::cache::ModelCapabilities {
            temperature: true,
            reasoning: false,
            attachment: false,
            tool_call: true,
            input: Default::default(),
            output: Default::default(),
            interleaved: oc_variance::cache::InterleavedConfig::Enabled(false),
        },
        context_limit: 16385,
        output_limit: 4096,
        cost_input: 0.0005,
        cost_output: 0.0015,
        cost_cache_read: None,
        cost_cache_write: None,
        cost_over_200k_input: None,
        cost_over_200k_output: None,
        release_date: "2023-01-01".to_string(),
        status: oc_variance::cache::ModelStatus::Active,
        options: None,
        headers: None,
    };

    let variants = generate_variants(&model, "@ai-sdk/openai");

    assert!(
        variants.is_empty(),
        "Non-reasoning models should not have variants"
    );
}

#[test]
fn test_variants_for_google() {
    let model = ModelInfo {
        id: "gemini-2.5-pro".to_string(),
        name: "Gemini 2.5 Pro".to_string(),
        provider_id: "google".to_string(),
        family: None,
        capabilities: oc_variance::cache::ModelCapabilities {
            temperature: true,
            reasoning: true,
            attachment: true,
            tool_call: true,
            input: Default::default(),
            output: Default::default(),
            interleaved: oc_variance::cache::InterleavedConfig::Enabled(false),
        },
        context_limit: 1000000,
        output_limit: 8192,
        cost_input: 0.00125,
        cost_output: 0.005,
        cost_cache_read: None,
        cost_cache_write: None,
        cost_over_200k_input: None,
        cost_over_200k_output: None,
        release_date: "2024-01-01".to_string(),
        status: oc_variance::cache::ModelStatus::Active,
        options: None,
        headers: None,
    };

    let variants = generate_variants(&model, "@ai-sdk/google");

    assert!(!variants.is_empty(), "Google reasoning models should have variants");
    assert!(
        variants.contains_key("high"),
        "Google 2.5 should have 'high' variant"
    );
    assert!(variants.contains_key("max"), "Google 2.5 should have 'max' variant");

    let high_config = variants.get("high").unwrap();
    assert_eq!(
        high_config["thinkingConfig"]["includeThoughts"],
        true,
        "Google 'high' variant should include thoughts"
    );
    assert_eq!(
        high_config["thinkingConfig"]["thinkingBudget"],
        16000,
        "Google 'high' variant should have 16000 thinking budget"
    );
}

#[test]
fn test_variants_for_mistral() {
    let model = ModelInfo {
        id: "mistral-large".to_string(),
        name: "Mistral Large".to_string(),
        provider_id: "mistral".to_string(),
        family: None,
        capabilities: oc_variance::cache::ModelCapabilities {
            temperature: true,
            reasoning: true,
            attachment: false,
            tool_call: true,
            input: Default::default(),
            output: Default::default(),
            interleaved: oc_variance::cache::InterleavedConfig::Enabled(false),
        },
        context_limit: 128000,
        output_limit: 4096,
        cost_input: 0.004,
        cost_output: 0.012,
        cost_cache_read: None,
        cost_cache_write: None,
        cost_over_200k_input: None,
        cost_over_200k_output: None,
        release_date: "2024-01-01".to_string(),
        status: oc_variance::cache::ModelStatus::Active,
        options: None,
        headers: None,
    };

    let variants = generate_variants(&model, "@ai-sdk/mistral");

    assert!(
        variants.is_empty(),
        "Mistral models should not have variants"
    );
}

#[test]
fn test_variants_for_groq() {
    let model = ModelInfo {
        id: "llama-3.3-70b-instruct".to_string(),
        name: "Llama 3.3 70B Instruct".to_string(),
        provider_id: "groq".to_string(),
        family: None,
        capabilities: oc_variance::cache::ModelCapabilities {
            temperature: true,
            reasoning: true,
            attachment: false,
            tool_call: true,
            input: Default::default(),
            output: Default::default(),
            interleaved: oc_variance::cache::InterleavedConfig::Enabled(false),
        },
        context_limit: 131072,
        output_limit: 8192,
        cost_input: 0.00059,
        cost_output: 0.00079,
        cost_cache_read: None,
        cost_cache_write: None,
        cost_over_200k_input: None,
        cost_over_200k_output: None,
        release_date: "2024-01-01".to_string(),
        status: oc_variance::cache::ModelStatus::Active,
        options: None,
        headers: None,
    };

    let variants = generate_variants(&model, "@ai-sdk/groq");

    assert!(!variants.is_empty(), "Groq reasoning models should have variants");
    assert!(
        variants.contains_key("none"),
        "Groq should have 'none' variant"
    );
    assert!(
        variants.contains_key("low"),
        "Groq should have 'low' variant"
    );
    assert!(
        variants.contains_key("medium"),
        "Groq should have 'medium' variant"
    );
    assert!(
        variants.contains_key("high"),
        "Groq should have 'high' variant"
    );
}
