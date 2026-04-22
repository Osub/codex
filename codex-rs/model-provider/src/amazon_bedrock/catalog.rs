use codex_protocol::openai_models::InputModality;
use codex_protocol::openai_models::ReasoningEffort;

use crate::model_catalog::ProviderModelCatalog;
use crate::model_catalog::ProviderModelMetadata;

const GPT_OSS_CONTEXT_WINDOW: i64 = 128_000;

pub(super) fn static_model_catalog() -> ProviderModelCatalog {
    ProviderModelCatalog {
        models: vec![
            bedrock_model(
                "openai.gpt-oss-120b-1:0",
                "GPT OSS 120B on Bedrock",
                /*priority*/ 0,
            ),
            bedrock_model(
                "openai.gpt-oss-20b-1:0",
                "GPT OSS 20B on Bedrock",
                /*priority*/ 1,
            ),
        ],
    }
}

fn bedrock_model(slug: &str, display_name: &str, priority: i32) -> ProviderModelMetadata {
    ProviderModelMetadata {
        slug: slug.to_string(),
        model_id: slug.to_string(),
        display_name: display_name.to_string(),
        description: Some(display_name.to_string()),
        context_window: GPT_OSS_CONTEXT_WINDOW,
        priority,
        default_reasoning_effort: ReasoningEffort::Medium,
        supported_reasoning_efforts: vec![
            ReasoningEffort::Low,
            ReasoningEffort::Medium,
            ReasoningEffort::High,
        ],
        input_modalities: vec![InputModality::Text],
        hidden: false,
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn catalog_uses_bedrock_model_ids_as_slugs() {
        let catalog = static_model_catalog();

        assert_eq!(catalog.models.len(), 2);
        assert_eq!(catalog.models[0].slug, "openai.gpt-oss-120b-1:0");
        assert_eq!(catalog.models[0].model_id, "openai.gpt-oss-120b-1:0");
        assert_eq!(catalog.models[1].slug, "openai.gpt-oss-20b-1:0");
        assert_eq!(catalog.models[1].model_id, "openai.gpt-oss-20b-1:0");
    }
}
