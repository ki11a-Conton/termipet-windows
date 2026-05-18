use tauri::AppHandle;
use crate::models::{ClaudeUsage, CopilotUsage, OllamaStatus};
use crate::services::ai_usage_service::AIUsageService;

#[tauri::command]
pub async fn get_claude_usage(app: AppHandle) -> Result<Option<ClaudeUsage>, String> {
    let service = AIUsageService::new();
    service.get_claude_usage().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_copilot_usage(app: AppHandle) -> Result<Option<CopilotUsage>, String> {
    let service = AIUsageService::new();
    service.get_copilot_usage().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ollama_status(app: AppHandle) -> Result<OllamaStatus, String> {
    let service = AIUsageService::new();
    service.get_ollama_status().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_tools_summary(app: AppHandle) -> Result<serde_json::Value, String> {
    let service = AIUsageService::new();
    Ok(service.get_ai_tools_summary().await)
}
