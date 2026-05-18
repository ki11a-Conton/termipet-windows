use crate::models::{ClaudeUsage, TerminalInfo};
use std::path::PathBuf;
use std::process::Command;

/// Service for integrating with Claude Code Desktop
pub struct ClaudeDesktopService;

#[derive(Debug, Clone)]
pub enum ClaudeDesktopState {
    NotInstalled,
    InstalledButNotRunning,
    Running { window_info: TerminalInfo },
}

impl ClaudeDesktopService {
    pub fn new() -> Self {
        Self
    }

    /// Check if Claude Code is installed
    pub async fn is_installed(&self) -> bool {
        // Check for claude CLI
        if self.is_cli_installed().await {
            return true;
        }

        // Check for desktop app installation
        self.is_desktop_app_installed().await
    }

    /// Check if Claude CLI is installed
    async fn is_cli_installed(&self) -> bool {
        Command::new("claude")
            .args(["--version"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Check if Claude Desktop app is installed
    async fn is_desktop_app_installed(&self) -> bool {
        #[cfg(windows)]
        {
            // Check common installation paths
            let possible_paths = vec![
                dirs::home_dir().map(|h| h.join("AppData").join("Local").join("Claude")),
                dirs::home_dir().map(|h| h.join("AppData").join("Roaming").join("Claude")),
                Some(PathBuf::from("C:\\Program Files\\Claude")),
                Some(PathBuf::from("C:\\Program Files (x86)\\Claude")),
            ];

            for path in possible_paths.into_iter().flatten() {
                if path.exists() {
                    return true;
                }
            }

            // Check registry or start menu
            self.check_start_menu_for_claude().await
        }
        #[cfg(not(windows))]
        false
    }

    #[cfg(windows)]
    async fn check_start_menu_for_claude(&self) -> bool {
        use std::os::windows::process::CommandExt;
        
        // Check if claude is in PATH via where command
        Command::new("where")
            .arg("claude")
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Check if Claude is currently running
    pub async fn is_running(&self) -> bool {
        self.find_claude_window().await.is_some()
    }

    /// Get current state of Claude Desktop
    pub async fn get_state(&self) -> ClaudeDesktopState {
        if !self.is_installed().await {
            return ClaudeDesktopState::NotInstalled;
        }

        if let Some(window_info) = self.find_claude_window().await {
            ClaudeDesktopState::Running { window_info }
        } else {
            ClaudeDesktopState::InstalledButNotRunning
        }
    }

    /// Find Claude window and return its info
    async fn find_claude_window(&self) -> Option<TerminalInfo> {
        #[cfg(windows)]
        {
            use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
            use windows::Win32::UI::WindowsAndMessaging::{
                EnumWindows, GetWindowTextW, IsWindowVisible,
            };
            use std::ffi::OsString;
            use std::os::windows::ffi::OsStringExt;

            let mut result: Option<TerminalInfo> = None;

            unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
                let result = &mut *(lparam.0 as *mut Option<TerminalInfo>);

                if IsWindowVisible(hwnd).as_bool() {
                    let mut title = [0u16; 512];
                    let len = GetWindowTextW(hwnd, &mut title);
                    let title = OsString::from_wide(&title[..len as usize])
                        .to_string_lossy()
                        .to_string();

                    // Check if this is a Claude window
                    let title_lower = title.to_lowercase();
                    if title_lower.contains("claude") && 
                       (title_lower.contains("code") || title_lower.contains("anthropic")) {
                        
                        // Get process name
                        let process_name = get_process_name_from_hwnd(hwnd);
                        
                        *result = Some(TerminalInfo {
                            id: format!("{}", hwnd.0),
                            name: "Claude Code".to_string(),
                            process_name,
                            window_title: title,
                            current_directory: None,
                            is_active: false,
                        });
                        
                        return BOOL(0); // Stop enumeration
                    }
                }

                BOOL(1)
            }

            unsafe {
                EnumWindows(
                    Some(enum_callback),
                    LPARAM(&mut result as *mut _ as isize),
                );
            }

            result
        }
        #[cfg(not(windows))]
        None
    }

    /// Launch Claude Code
    pub async fn launch(&self, directory: Option<&str>) -> anyhow::Result<()> {
        let mut cmd = Command::new("claude");
        
        if let Some(dir) = directory {
            cmd.current_dir(dir);
        }
        
        cmd.spawn()?;
        Ok(())
    }

    /// Send command to Claude Code
    pub async fn send_command(&self, command: &str) -> anyhow::Result<()> {
        if let Some(window_info) = self.find_claude_window().await {
            // Use terminal service to send command
            let terminal_service = super::terminal_service::TerminalService::new();
            terminal_service.send_command(&window_info.id, command).await
        } else {
            Err(anyhow::anyhow!("Claude Code is not running"))
        }
    }

    /// Get Claude usage information
    pub async fn get_usage(&self) -> anyhow::Result<Option<ClaudeUsage>> {
        // Try to get usage from claude CLI
        match Command::new("claude")
            .args(["status", "--json"])
            .output() 
        {
            Ok(output) if output.status.success() => {
                let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
                
                Ok(Some(ClaudeUsage {
                    tier: json["tier"].as_str().unwrap_or("free").to_string(),
                    requests_used: json["requests_used"].as_u64().unwrap_or(0) as u32,
                    requests_limit: json["requests_limit"].as_u64().map(|v| v as u32),
                    tokens_used: json["tokens_used"].as_u64().unwrap_or(0),
                    reset_time: json["reset_time"]
                        .as_str()
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc)),
                }))
            }
            _ => {
                // Fallback: read from config
                self.get_usage_from_config().await
            }
        }
    }

    /// Get usage from config file
    async fn get_usage_from_config(&self) -> anyhow::Result<Option<ClaudeUsage>> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        
        let settings_path = home.join(".claude").join("settings.json");
        
        if !settings_path.exists() {
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(&settings_path).await?;
        let settings: serde_json::Value = serde_json::from_str(&content)?;

        Ok(Some(ClaudeUsage {
            tier: settings["tier"].as_str().unwrap_or("free").to_string(),
            requests_used: 0,
            requests_limit: None,
            tokens_used: 0,
            reset_time: None,
        }))
    }

    /// Install Claude Code CLI hook
    pub async fn install_hook(&self) -> anyhow::Result<()> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        
        let claude_dir = home.join(".claude");
        let settings_path = claude_dir.join("settings.json");
        
        // Ensure .claude directory exists
        if !claude_dir.exists() {
            tokio::fs::create_dir_all(&claude_dir).await?;
        }

        // Backup existing settings
        if settings_path.exists() {
            let backup_path = settings_path.with_extension("json.termipet.bak");
            tokio::fs::copy(&settings_path, &backup_path).await?;
        }

        // Read or create settings
        let mut settings: serde_json::Value = if settings_path.exists() {
            let content = tokio::fs::read_to_string(&settings_path).await?;
            serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({}))
        } else {
            serde_json::json!({})
        };

        // Add hook configuration
        let hooks = settings.get_mut("hooks").unwrap_or_else(|| {
            settings["hooks"] = serde_json::json!({});
            &mut settings["hooks"]
        });

        hooks["termipet"] = serde_json::json!({
            "url": "http://127.0.0.1:8765/hook",
            "events": ["thinking", "tool_use", "permission_request", "completion"]
        });

        // Save settings
        tokio::fs::write(&settings_path, serde_json::to_string_pretty(&settings)?).await?;

        Ok(())
    }

    /// Uninstall Claude Code CLI hook
    pub async fn uninstall_hook(&self) -> anyhow::Result<()> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        
        let settings_path = home.join(".claude").join("settings.json");
        
        if !settings_path.exists() {
            return Ok(());
        }

        let content = tokio::fs::read_to_string(&settings_path).await?;
        let mut settings: serde_json::Value = serde_json::from_str(&content)?;

        // Remove hook configuration
        if let Some(hooks) = settings.get_mut("hooks") {
            if let Some(obj) = hooks.as_object_mut() {
                obj.remove("termipet");
            }
        }

        // Save settings
        tokio::fs::write(&settings_path, serde_json::to_string_pretty(&settings)?).await?;

        Ok(())
    }
}

#[cfg(windows)]
unsafe fn get_process_name_from_hwnd(hwnd: windows::Win32::Foundation::HWND) -> String {
    use windows::Win32::System::Threading::{GetWindowThreadProcessId, OpenProcess};
    use windows::Win32::System::ProcessStatus::GetModuleBaseNameW;
    use windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION;
    use windows::Win32::System::Threading::PROCESS_VM_READ;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    let mut process_id: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut process_id));

    if process_id == 0 {
        return String::new();
    }

    let process_handle = OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
        false,
        process_id,
    );

    if let Ok(handle) = process_handle {
        let mut name = [0u16; 260];
        let len = GetModuleBaseNameW(handle, None, &mut name);
        let _ = windows::Win32::Foundation::CloseHandle(handle);
        
        if len > 0 {
            return OsString::from_wide(&name[..len as usize])
                .to_string_lossy()
                .to_string();
        }
    }

    String::new()
}
