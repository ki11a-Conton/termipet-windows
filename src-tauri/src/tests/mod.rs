#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::services::*;

    #[test]
    fn test_pet_state_serialization() {
        let state = PetState::Happy;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"happy\"");
        
        let deserialized: PetState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, PetState::Happy);
    }

    #[test]
    fn test_model_provider_serialization() {
        let provider = ModelProvider::Ollama;
        let json = serde_json::to_string(&provider).unwrap();
        assert_eq!(json, "\"ollama\"");
    }

    #[test]
    fn test_app_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.language, "zh-CN");
        assert_eq!(settings.skin, AppSkin::Glass);
        assert_eq!(settings.pet_id, "terminal-cat");
    }

    #[tokio::test]
    async fn test_terminal_service_creation() {
        let service = terminal_service::TerminalService::new();
        // Just verify it can be created without panicking
    }

    #[tokio::test]
    async fn test_ai_usage_service_creation() {
        let service = ai_usage_service::AIUsageService::new();
        // Just verify it can be created without panicking
    }
}
