use tauri::AppHandle;

pub mod pet_service;
pub mod chat_service;
pub mod settings_service;
pub mod terminal_service;
pub mod system_service;
pub mod ai_usage_service;
pub mod timer_service;
pub mod claude_desktop_service;

pub async fn init_services(app: &AppHandle) {
    // Initialize all services here
    pet_service::init(app);
    chat_service::init(app);
    timer_service::init(app);
}
