use tauri::{Manager, Window};

/// Make window click-through except for specific regions
pub fn set_click_through(window: &Window, enabled: bool) -> Result<(), String> {
    #[cfg(windows)]
    {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetWindowLongW, SetWindowLongW, GWL_EXSTYLE,
            WS_EX_LAYERED, WS_EX_TRANSPARENT,
        };
        
        let hwnd = HWND(window.hwnd().map_err(|e| e.to_string())?.0);
        
        unsafe {
            let style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            
            let new_style = if enabled {
                style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32
            } else {
                style & !(WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32)
            };
            
            SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);
        }
    }
    
    Ok(())
}

/// Set window to always be on top
pub fn set_always_on_top(window: &Window, always_on_top: bool) -> Result<(), String> {
    window
        .set_always_on_top(always_on_top)
        .map_err(|e| e.to_string())
}

/// Make window ignore mouse events (for pet window)
pub fn set_ignore_cursor_events(window: &Window, ignore: bool) -> Result<(), String> {
    window
        .set_ignore_cursor_events(ignore)
        .map_err(|e| e.to_string())
}
