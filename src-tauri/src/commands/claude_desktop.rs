use tauri::AppHandle;
use crate::services::claude_desktop_service::ClaudeDesktopService;
use crate::models::ClaudeUsage;

#[tauri::command]
pub async fn check_claude_installed(app: AppHandle) -> Result<bool, String> {
    let service = ClaudeDesktopService::new();
    Ok(service.is_installed().await)
}

#[tauri::command]
pub async fn check_claude_running(app: AppHandle) -> Result<bool, String> {
    let service = ClaudeDesktopService::new();
    Ok(service.is_running().await)
}

#[tauri::command]
pub async fn get_claude_state(app: AppHandle) -> Result<serde_json::Value, String> {
    let service = ClaudeDesktopService::new();
    let state = service.get_state().await;
    
    let result = match state {
        crate::services::claude_desktop_service::ClaudeDesktopState::NotInstalled => {
            serde_json::json!({
                "status": "not_installed",
                "message": "Claude Code is not installed"
            })
        }
        crate::services::claude_desktop_service::ClaudeDesktopState::InstalledButNotRunning => {
            serde_json::json!({
                "status": "not_running",
                "message": "Claude Code is installed but not running"
            })
        }
        crate::services::claude_desktop_service::ClaudeDesktopState::Running { window_info } => {
            serde_json::json!({
                "status": "running",
                "message": "Claude Code is running",
                "window": {
                    "id": window_info.id,
                    "title": window_info.window_title,
                    "process": window_info.process_name
                }
            })
        }
    };
    
    Ok(result)
}

#[tauri::command]
pub async fn launch_claude(app: AppHandle, directory: Option<String>) -> Result<(), String> {
    let service = ClaudeDesktopService::new();
    service.launch(directory.as_deref()).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_command_to_claude(app: AppHandle, command: String) -> Result<(), String> {
    let service = ClaudeDesktopService::new();
    service.send_command(&command).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_claude_usage_info(app: AppHandle) -> Result<Option<ClaudeUsage>, String> {
    let service = ClaudeDesktopService::new();
    service.get_usage().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_claude_hook(app: AppHandle) -> Result<(), String> {
    let service = ClaudeDesktopService::new();
    service.install_hook().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn uninstall_claude_hook(app: AppHandle) -> Result<(), String> {
    let service = ClaudeDesktopService::new();
    service.uninstall_hook().await.map_err(|e| e.to_string())
}
