use tauri::AppHandle;
use crate::models::TerminalInfo;
use crate::services::terminal_service::TerminalService;

#[tauri::command]
pub async fn get_active_terminal(app: AppHandle) -> Result<Option<TerminalInfo>, String> {
    let service = TerminalService::new();
    service.get_active_terminal().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_terminal_list(app: AppHandle) -> Result<Vec<TerminalInfo>, String> {
    let service = TerminalService::new();
    service.get_terminal_list().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_command_to_terminal(
    app: AppHandle,
    terminal_id: String,
    command: String,
) -> Result<(), String> {
    let service = TerminalService::new();
    service
        .send_command(&terminal_id, &command)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_cd_to_terminal(
    app: AppHandle,
    terminal_id: String,
    path: String,
) -> Result<(), String> {
    let service = TerminalService::new();
    service
        .send_cd_command(&terminal_id, &path)
        .await
        .map_err(|e| e.to_string())
}
