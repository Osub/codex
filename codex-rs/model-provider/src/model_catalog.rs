use codex_protocol::openai_models::InputModality;
use codex_protocol::openai_models::ReasoningEffort;

/// Provider-owned catalog metadata used to synthesize Codex model listings.
///
/// Implementations should use the model identifier that the provider expects
/// at request time as `slug`. Keep `model_id` alongside it so future providers
/// can add user-facing aliases without losing the runtime identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderModelCatalog {
    pub models: Vec<ProviderModelMetadata>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderModelMetadata {
    pub slug: String,
    pub model_id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub context_window: i64,
    pub priority: i32,
    pub default_reasoning_effort: ReasoningEffort,
    pub supported_reasoning_efforts: Vec<ReasoningEffort>,
    pub input_modalities: Vec<InputModality>,
    pub hidden: bool,
}
