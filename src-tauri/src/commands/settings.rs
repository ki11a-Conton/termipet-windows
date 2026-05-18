use tauri::AppHandle;
use crate::models::AppSettings;
use crate::services::settings_service::SettingsService;

#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    let service = SettingsService::new(&app);
    service.get_settings().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    let service = SettingsService::new(&app);
    service.save_settings(&settings).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_api_key(app: AppHandle, provider: String) -> Result<Option<String>, String> {
    let service = SettingsService::new(&app);
    service.get_api_key(&provider).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_api_key(
    app: AppHandle,
    provider: String,
    api_key: Option<String>,
) -> Result<(), String> {
    let service = SettingsService::new(&app);
    service.set_api_key(&provider, api_key.as_deref()).await.map_err(|e| e.to_string())
}
