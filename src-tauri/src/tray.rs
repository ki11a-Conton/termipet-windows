use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};

pub fn setup_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    // Create menu items
    let show_pet = MenuItem::with_id(app, "show_pet", "Show Pet", true, None::<&str>)?;
    let show_chat = MenuItem::with_id(app, "show_chat", "Open Chat", true, None::<&str>)?;
    let show_settings = MenuItem::with_id(app, "show_settings", "Settings", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let start_timer = MenuItem::with_id(app, "start_timer", "Start Pomodoro", true, None::<&str>)?;
    let stop_timer = MenuItem::with_id(app, "stop_timer", "Stop Timer", true, None::<&str>)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    // Create menu
    let menu = Menu::with_items(
        app,
        &[
            &show_pet,
            &show_chat,
            &show_settings,
            &separator,
            &start_timer,
            &stop_timer,
            &separator2,
            &quit,
        ],
    )?;

    // Build tray icon
    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("TermiPet")
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "show_pet" => {
                    if let Some(window) = app.get_webview_window("pet") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "show_chat" => {
                    if let Some(window) = app.get_webview_window("chat") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "show_settings" => {
                    if let Some(window) = app.get_webview_window("settings") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "start_timer" => {
                    // Start pomodoro timer
                    let handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = handle.emit("start_pomodoro", 25);
                    });
                }
                "stop_timer" => {
                    let handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = handle.emit("stop_pomodoro", ());
                    });
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                // Toggle pet window on left click
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("pet") {
                    if let Ok(visible) = window.is_visible() {
                        if visible {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
