use crate::models::{TerminalInfo, TerminalStatus};
use std::process::Command;

pub struct TerminalService;

#[derive(Debug, Clone)]
pub enum TerminalType {
    WindowsTerminal,
    PowerShell,
    Cmd,
    GitBash,
    Wsl,
    Unknown,
}

impl TerminalService {
    pub fn new() -> Self {
        Self
    }

    /// Detect terminal type from window title and process name
    fn detect_terminal_type(title: &str, process_name: &str) -> TerminalType {
        let title_lower = title.to_lowercase();
        let process_lower = process_name.to_lowercase();

        if title_lower.contains("windows terminal") || process_lower.contains("windowsterminal") {
            TerminalType::WindowsTerminal
        } else if title_lower.contains("powershell") || title_lower.contains("pwsh") || process_lower.contains("powershell") || process_lower.contains("pwsh") {
            TerminalType::PowerShell
        } else if title_lower.contains("command prompt") || title_lower.contains("cmd") || process_lower.contains("cmd") {
            TerminalType::Cmd
        } else if title_lower.contains("git bash") || title_lower.contains("bash") || process_lower.contains("bash") {
            TerminalType::GitBash
        } else if title_lower.contains("wsl") || title_lower.contains("ubuntu") || process_lower.contains("wsl") {
            TerminalType::Wsl
        } else {
            TerminalType::Unknown
        }
    }

    /// Get process name from window handle
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

    pub async fn get_active_terminal(&self) -> anyhow::Result<Option<TerminalInfo>> {
        #[cfg(windows)]
        {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{
                GetForegroundWindow, GetWindowTextW,
            };
            use std::ffi::OsString;
            use std::os::windows::ffi::OsStringExt;

            unsafe {
                let foreground = GetForegroundWindow();
                if foreground.0 == 0 {
                    return Ok(None);
                }

                // Get window title
                let mut title = [0u16; 512];
                let len = GetWindowTextW(foreground, &mut title);
                let title = OsString::from_wide(&title[..len as usize])
                    .to_string_lossy()
                    .to_string();

                // Get process name
                let process_name = Self::get_process_name_from_hwnd(foreground);

                // Check if it's a terminal
                let terminal_type = Self::detect_terminal_type(&title, &process_name);

                match terminal_type {
                    TerminalType::Unknown => Ok(None),
                    _ => {
                        let terminal_name = match terminal_type {
                            TerminalType::WindowsTerminal => "Windows Terminal",
                            TerminalType::PowerShell => "PowerShell",
                            TerminalType::Cmd => "Command Prompt",
                            TerminalType::GitBash => "Git Bash",
                            TerminalType::Wsl => "WSL",
                            TerminalType::Unknown => "Terminal",
                        };

                        Ok(Some(TerminalInfo {
                            id: format!("{}", foreground.0),
                            name: terminal_name.to_string(),
                            process_name,
                            window_title: title,
                            current_directory: self.get_terminal_cwd(&terminal_type).await.ok(),
                            is_active: true,
                        }))
                    }
                }
            }
        }

        #[cfg(not(windows))]
        Ok(None)
    }

    /// Try to get current working directory from terminal
    async fn get_terminal_cwd(&self, terminal_type: &TerminalType) -> anyhow::Result<String> {
        match terminal_type {
            TerminalType::PowerShell => {
                // Use PowerShell to get current location
                let output = Command::new("powershell.exe")
                    .args(["-Command", "Get-Location | Select-Object -ExpandProperty Path"])
                    .output()?;
                
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    Err(anyhow::anyhow!("Failed to get PowerShell CWD"))
                }
            }
            TerminalType::Cmd => {
                // Use cmd to get current directory
                let output = Command::new("cmd.exe")
                    .args(["/c", "cd"])
                    .output()?;
                
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    Err(anyhow::anyhow!("Failed to get CMD CWD"))
                }
            }
            _ => Err(anyhow::anyhow!("CWD detection not supported for this terminal")),
        }
    }

    pub async fn get_terminal_list(&self) -> anyhow::Result<Vec<TerminalInfo>> {
        let mut terminals = Vec::new();

        #[cfg(windows)]
        {
            use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
            use windows::Win32::UI::WindowsAndMessaging::{
                EnumWindows, GetWindowTextW, IsWindowVisible,
            };
            use std::ffi::OsString;
            use std::os::windows::ffi::OsStringExt;

            struct EnumData {
                terminals: Vec<TerminalInfo>,
                service: *const TerminalService,
            }

            unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
                let data = &mut *(lparam.0 as *mut EnumData);
                let service = &*data.service;

                if IsWindowVisible(hwnd).as_bool() {
                    let mut title = [0u16; 512];
                    let len = GetWindowTextW(hwnd, &mut title);
                    let title = OsString::from_wide(&title[..len as usize])
                        .to_string_lossy()
                        .to_string();

                    if !title.is_empty() {
                        let process_name = TerminalService::get_process_name_from_hwnd(hwnd);
                        let terminal_type = TerminalService::detect_terminal_type(&title, &process_name);

                        if !matches!(terminal_type, TerminalType::Unknown) {
                            let terminal_name = match terminal_type {
                                TerminalType::WindowsTerminal => "Windows Terminal",
                                TerminalType::PowerShell => "PowerShell",
                                TerminalType::Cmd => "Command Prompt",
                                TerminalType::GitBash => "Git Bash",
                                TerminalType::Wsl => "WSL",
                                TerminalType::Unknown => "Terminal",
                            };

                            data.terminals.push(TerminalInfo {
                                id: format!("{}", hwnd.0),
                                name: terminal_name.to_string(),
                                process_name,
                                window_title: title,
                                current_directory: None,
                                is_active: false,
                            });
                        }
                    }
                }

                BOOL(1)
            }

            let mut data = EnumData {
                terminals: Vec::new(),
                service: self,
            };

            unsafe {
                EnumWindows(
                    Some(enum_callback),
                    LPARAM(&mut data as *mut _ as isize),
                );
            }

            terminals = data.terminals;
        }

        // Mark the active terminal
        if let Ok(Some(active)) = self.get_active_terminal().await {
            for terminal in &mut terminals {
                if terminal.id == active.id {
                    terminal.is_active = true;
                }
            }
        }

        Ok(terminals)
    }

    pub async fn send_command(&self, terminal_id: &str, command: &str) -> anyhow::Result<()> {
        #[cfg(windows)]
        {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{
                SetForegroundWindow, SendMessageW, WM_CHAR,
            };
            use windows::Win32::UI::Input::KeyboardAndMouse::{
                keybd_event, KEYEVENTF_KEYUP, VK_RETURN,
            };

            let hwnd = HWND(terminal_id.parse::<isize>()?);

            unsafe {
                // Bring terminal to foreground
                SetForegroundWindow(hwnd);

                // Wait for focus
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

                // Send each character
                for ch in command.chars() {
                    // Handle special characters
                    match ch {
                        '\n' => {
                            // Send Enter key
                            keybd_event(VK_RETURN.0 as u8, 0, Default::default(), 0);
                            keybd_event(VK_RETURN.0 as u8, 0, KEYEVENTF_KEYUP, 0);
                        }
                        _ => {
                            SendMessageW(hwnd, WM_CHAR, ch as usize, 0);
                        }
                    }
                    
                    // Small delay between characters to ensure proper input
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }

                // Send Enter key at the end
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                keybd_event(VK_RETURN.0 as u8, 0, Default::default(), 0);
                keybd_event(VK_RETURN.0 as u8, 0, KEYEVENTF_KEYUP, 0);
            }
        }

        Ok(())
    }

    /// Send a "cd" command to change directory
    pub async fn send_cd_command(&self, terminal_id: &str, path: &str) -> anyhow::Result<()> {
        // Escape the path for the terminal
        let escaped_path = path.replace("\\", "/");
        let command = format!("cd \"{}\"", escaped_path);
        self.send_command(terminal_id, &command).await
    }

    /// Check if Windows Terminal is installed
    pub async fn is_windows_terminal_installed(&self) -> bool {
        #[cfg(windows)]
        {
            // Check if wt.exe exists in PATH
            Command::new("where")
                .arg("wt.exe")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
        #[cfg(not(windows))]
        false
    }

    /// Launch Windows Terminal with a specific profile
    pub async fn launch_windows_terminal(&self, profile: Option<&str>, directory: Option<&str>) -> anyhow::Result<()> {
        let mut cmd = Command::new("wt.exe");
        
        if let Some(profile) = profile {
            cmd.arg("-p").arg(profile);
        }
        
        if let Some(dir) = directory {
            cmd.arg("-d").arg(dir);
        }
        
        cmd.spawn()?;
        Ok(())
    }
}
