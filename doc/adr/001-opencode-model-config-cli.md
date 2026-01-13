# ADR-001: OpenCode Model Configuration CLI Tool

## Status
Proposed

## Context
OpenCode uses a JSON-based configuration system that allows users to configure multiple providers, models, and model variants with different thinking/reasoning levels. Currently, users must manually edit `opencode.json` to:
- Add or modify providers
- Configure models within providers
- Create and customize model variants
- Set model-specific options (thinking budget, reasoning effort, verbosity)

This manual editing process is error-prone and lacks discoverability. Users need a better way to:
1. See all configured providers and models
2. Browse existing variants and their parameters
3. Understand what parameters are available for each model
4. Add new models from providers
5. Create custom variants with specific configurations

## Decision Drivers
- **User Experience**: Need a friendly, interactive CLI interface for configuration
- **Discoverability**: Users should easily see what models and parameters are available
- **Validation**: Prevent invalid configurations before writing to files
- **Completeness**: Support all opencode model configuration features
- **Programmatic Discovery**: Leverage external APIs (models.dev) for model information

## Considered Options

### Option 1: Simple TUI with dialoguer/demand/inquire
Use a question-based CLI library that presents sequential menus for navigating the provider/model/variant hierarchy.

**Pros:**
- Simple to implement
- Familiar pattern for CLI users
- Easy to validate input

**Cons:**
- Lost context between menu levels
- Cannot see the full tree at once
- Tedious to navigate deep hierarchies

### Option 2: Tree-based TUI with ratatui
Use a terminal UI framework that displays the full provider/model/variant tree with persistent state and split-pane editing.

**Pros:**
- Full tree visibility at all times
- Navigate up/down the hierarchy freely
- Split-pane editing preserves context
- More professional appearance

**Cons:**
- More complex implementation
- Higher learning curve for users
- Requires more terminal features

### Option 3: Inline TUI within existing CLI
Build a lightweight tree UI that appears inline with the normal CLI flow, using a simplified rendering approach.

**Pros:**
- Less disruptive than full-screen TUI
- Can be integrated with existing opencode commands
- Simpler than full ratatui app

**Cons:**
- Limited screen real estate
- May be difficult to maintain tree state
- Potential compatibility issues

## Decision
**Choose Option 2: Tree-based TUI with ratatui**

The tree-based approach provides the best user experience for exploring and editing complex hierarchical configurations. The ability to see the entire tree and navigate freely between levels is critical for understanding how providers, models, and variants relate to each other.

## Proposed Architecture

### Data Model

```rust
pub struct ModelConfig {
    pub providers: HashMap<String, ProviderConfig>,
}

pub struct ProviderConfig {
    pub npm: Option<String>,
    pub name: Option<String>,
    pub options: ProviderOptions,
    pub models: HashMap<String, ModelEntry>,
}

pub struct ModelEntry {
    pub name: Option<String>,
    pub limit: Option<ContextLimits>,
    pub variants: HashMap<String, VariantConfig>,
}

pub struct VariantConfig {
    pub options: ModelOptions,
}

pub struct ModelOptions {
    // Anthropic-specific
    pub thinking: Option<ThinkingConfig>,
    
    // OpenAI-specific
    pub reasoning_effort: Option<String>, // "low", "medium", "high"
    pub text_verbosity: Option<String>,   // "low", "high"
    pub reasoning_summary: Option<String>, // "auto", "none"
    
    // Generic
    pub temperature: Option<f64>,
    pub max_tokens: Option<u32>,
}

pub struct ThinkingConfig {
    pub type_: String, // "enabled", "disabled"
    pub budget_tokens: u32,
}

pub struct ContextLimits {
    pub context: u32,
    pub output: u32,
}
```

### Tree Structure

The UI will display a hierarchical tree that is **fully expanded by default**. The tree has **no borders** to maximize screen space and reduce visual clutter.

```
OpenCode Model Configuration

  anthropic
    claude-sonnet-4-5-20250929
      thinking-high (variant)
      thinking-low (variant)
    claude-opus-4-20250514
      [new variant]
    [new model]
  openai
    gpt-5
      high-reasoning (variant)
    [new model]
  [new provider]

────────────────────────────────────────────────────────────
Configuration Panel

Selected: anthropic/claude-sonnet-4-5-20250929/thinking-high

Thinking Config:
  [X] Enabled
  Budget Tokens: [16000      ]

[Save] [Cancel]
```

### User Interaction Flow

1. **Navigation**: Use arrow keys or vim-style navigation to move through the tree
2. **Expansion/Collapse**:
   - Tree starts fully expanded
   - Press `[` to expand the selected subtree
   - Press `]` to collapse the selected subtree
   - Press Enter or → to toggle expansion state
3. **Selection**: Press Space to select a node and show in configuration panel
4. **New Items**: Select "[new ...]" entries to create new providers/models/variants
5. **Editing**: Modify parameters in the configuration panel
6. **Persistence**: Press "Save" to write changes to `opencode.json`

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| ↑ / ↓ | Navigate up/down in tree |
| → / Enter | Toggle expand/collapse on selected node |
| `[` | Expand the selected subtree |
| `]` | Collapse the selected subtree |
| Space | Select node and show in configuration panel |
| q | Quit application |
| s | Save configuration (when valid) |
| ? | Show help |

### Integration with models.dev

The tool will use the `models.dev` crate to:
1. Fetch available models for supported providers
2. Get metadata about model capabilities
3. Discover available parameters and their ranges

```rust
// Pseudo-code for models.dev integration
async fn fetch_provider_models(provider_id: &str) -> Vec<ModelMetadata> {
    let client = ModelsDevClient::new();
    client.list_models(provider_id).await
}

struct ModelMetadata {
    pub id: String,
    pub name: String,
    pub context_window: u32,
    pub supports_thinking: bool,
    pub max_thinking_tokens: Option<u32>,
    pub supports_reasoning_effort: bool,
}
```

### Known Model Parameters

Based on opencode documentation, we understand these parameters:

#### Anthropic Models
- `thinking.type`: "enabled" or "disabled"
- `thinking.budget_tokens`: integer (typically up to 200000)
- `temperature`: float (0.0 - 1.0)
- `max_tokens`: integer

#### OpenAI Models
- `reasoning_effort`: "low", "medium", or "high"
- `text_verbosity`: "low" or "high"
- `reasoning_summary`: "auto" or "none"
- `temperature`: float (0.0 - 2.0)
- `max_tokens`: integer

#### Generic Parameters
- `temperature`: Controls randomness (0.0 - 2.0 depending on model)
- `max_tokens`: Maximum output tokens

#### Provider Options
- `baseURL`: Custom API endpoint
- `apiKey`: Authentication key
- `headers`: Custom HTTP headers

### Missing Information

We need to research the following to build a complete solution:

1. **Parameter Discovery**: Does `models.dev` API provide schema information for model parameters?
   - Available parameter names
   - Valid value ranges
   - Default values
   - Required vs optional parameters

2. **Model Capabilities**: Can we determine from `models.dev`:
   - Which models support extended thinking?
   - Maximum thinking budget per model?
   - Which models support reasoning effort?
   - Context window limits?

3. **Provider-Specific Validation**: Need to understand:
   - What parameters are valid for each provider type?
   - Are there mutually exclusive parameters?
   - How do parameters interact?

4. **Opencode Schema**: Need to verify:
   - Full list of supported parameters
   - Schema validation rules
   - Version-specific differences

### Implementation Plan

#### Phase 1: Core Data Structures
1. Define Rust types for opencode configuration
2. Implement parsing and serialization of `opencode.json`
3. Create in-memory tree representation

#### Phase 2: models.dev Integration
1. Investigate `models.dev` crate API
2. Implement provider/model fetching
3. Test parameter discovery capabilities

#### Phase 3: Basic UI Framework
1. Set up ratatui application
2. Implement tree view rendering
3. Add basic navigation

#### Phase 4: Configuration Panel
1. Create split-pane layout
2. Implement form fields for parameters
3. Add validation

#### Phase 5: CRUD Operations
1. Implement create (new provider/model/variant)
2. Implement read (parse and display)
3. Implement update (edit and save)
4. Implement delete (remove items)

#### Phase 6: Advanced Features
1. Search/filter functionality
2. Import/export configurations
3. Configuration templates
4. Validation and error messages

### Technology Stack

- **Language**: Rust
- **TUI Framework**: ratatui
- **Input Handling**: crossterm
- **Configuration Parsing**: serde_json
- **Model Discovery**: models.dev crate
- **Forms/Validation**: custom implementation (or inquire integration)

### Dependencies

```toml
[dependencies]
ratatui = "0.26"
crossterm = "0.27"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
models-dev = "0.1"  # placeholder - verify actual crate name
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Alternatives Considered

**dialoguer** - Good for simple prompts, but lacks persistent tree views  
**demand** - Focused on input validation, not suitable for complex UIs  
**inquire** - Modern and ergonomic, but designed for sequential workflows  

### Success Criteria

1. Users can view all configured providers, models, and variants
2. Users can navigate the tree without losing context
3. Users can edit existing configurations with real-time validation
4. Users can add new models using `models.dev` discovery
5. Users can create custom variants with specific parameter combinations
6. Changes are validated before writing to `opencode.json`
7. Tool handles edge cases (invalid configs, missing files, etc.)

### Open Questions

1. What is the exact API for the `models.dev` crate?
2. Does `models.dev` provide parameter schemas?
3. How should we handle providers not supported by `models.dev`?
4. Should we support local model providers (Ollama, LM Studio)?
5. How do we handle configuration migrations when opencode schema changes?

### References

- [OpenCode Configuration Documentation](https://opencode.ai/docs/config)
- [ratatui Documentation](https://ratatui.rs/)
- [models.dev API](https://models.dev/api) (investigate actual URL)
- OpenCode configuration examples in `provider/opencode-model-config.json`

---

**Document Version**: 1.0  
**Created**: January 13, 2026  
**Last Updated**: January 13, 2026
