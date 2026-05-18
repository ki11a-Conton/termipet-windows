use tauri::AppHandle;
use crate::models::SystemInfo;
use crate::services::system_service::SystemService;

#[tauri::command]
pub async fn get_system_info(app: AppHandle) -> Result<SystemInfo, String> {
    let service = SystemService::new();
    
    Ok(SystemInfo {
        os_version: service.get_os_version(),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        accessibility_enabled: service.check_accessibility_permission(),
    })
}

#[tauri::command]
pub async fn request_accessibility_permission(app: AppHandle) -> Result<(), String> {
    let service = SystemService::new();
    service.request_accessibility_permission().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_accessibility_permission(app: AppHandle) -> Result<bool, String> {
    let service = SystemService::new();
    Ok(service.check_accessibility_permission())
}
