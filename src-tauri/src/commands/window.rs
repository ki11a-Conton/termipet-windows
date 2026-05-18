use tauri::{Manager, Window};
use crate::models::WindowPosition;

#[tauri::command]
pub async fn show_window(window: Window, label: String) -> Result<(), String> {
    let target = window
        .get_webview_window(&label)
        .ok_or_else(|| format!("Window '{}' not found", label))?;
    
    target.show().map_err(|e| e.to_string())?;
    target.set_focus().map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn hide_window(window: Window, label: String) -> Result<(), String> {
    let target = window
        .get_webview_window(&label)
        .ok_or_else(|| format!("Window '{}' not found", label))?;
    
    target.hide().map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn set_window_position(
    window: Window,
    label: String,
    position: WindowPosition,
) -> Result<(), String> {
    let target = window
        .get_webview_window(&label)
        .ok_or_else(|| format!("Window '{}' not found", label))?;
    
    target
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x: position.x,
            y: position.y,
        }))
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_window_position(window: Window, label: String) -> Result<WindowPosition, String> {
    let target = window
        .get_webview_window(&label)
        .ok_or_else(|| format!("Window '{}' not found", label))?;
    
    let pos = target.outer_position().map_err(|e| e.to_string())?;
    
    Ok(WindowPosition {
        x: pos.x,
        y: pos.y,
    })
}
