// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tracing::{info, debug};

mod commands;
mod models;
mod services;
mod utils;
mod tray;

use commands::*;
use tray::setup_tray;

fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    info!("Starting TermiPet for Windows...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            debug!("Setting up TermiPet application...");
            
            // Initialize system tray
            #[cfg(desktop)]
            {
                setup_tray(app.handle())?;
            }
            
            // Initialize services
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                services::init_services(&handle).await;
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Window commands
            window::show_window,
            window::hide_window,
            window::set_window_position,
            window::get_window_position,
            
            // Pet commands
            pet::get_pet_state,
            pet::set_pet_state,
            pet::get_available_pets,
            pet::load_pet,
            pet::import_pet,
            
            // Chat commands
            chat::send_message,
            chat::get_chat_history,
            chat::clear_chat_history,
            chat::get_available_models,
            chat::test_model_connection,
            
            // Settings commands
            settings::get_settings,
            settings::save_settings,
            settings::get_api_key,
            settings::set_api_key,
            
            // Terminal commands
            terminal::get_active_terminal,
            terminal::send_command_to_terminal,
            terminal::get_terminal_list,
            terminal::send_cd_to_terminal,
            
            // System commands
            system::get_system_info,
            system::request_accessibility_permission,
            system::check_accessibility_permission,
            
            // AI Usage commands
            ai_usage::get_claude_usage,
            ai_usage::get_copilot_usage,
            ai_usage::get_ollama_status,
            ai_usage::get_ai_tools_summary,
            
            // Timer commands
            timer::start_pomodoro,
            timer::stop_pomodoro,
            timer::get_timer_state,
            
            // Claude Desktop commands
            claude_desktop::check_claude_installed,
            claude_desktop::check_claude_running,
            claude_desktop::get_claude_state,
            claude_desktop::launch_claude,
            claude_desktop::send_command_to_claude,
            claude_desktop::get_claude_usage_info,
            claude_desktop::install_claude_hook,
            claude_desktop::uninstall_claude_hook,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
