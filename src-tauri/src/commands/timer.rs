use tauri::AppHandle;
use crate::models::{TimerState, TimerMode};
use crate::services::timer_service::TimerService;

#[tauri::command]
pub async fn start_pomodoro(
    app: AppHandle,
    duration_minutes: u32,
) -> Result<TimerState, String> {
    let service = app.state::<TimerService>();
    service
        .start(TimerMode::Pomodoro, duration_minutes * 60)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_pomodoro(app: AppHandle) -> Result<(), String> {
    let service = app.state::<TimerService>();
    service.stop().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_timer_state(app: AppHandle) -> Result<TimerState, String> {
    let service = app.state::<TimerService>();
    Ok(service.get_state().await)
}
