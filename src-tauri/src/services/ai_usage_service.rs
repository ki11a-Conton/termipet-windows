use crate::models::{ClaudeUsage, CopilotUsage, OllamaStatus};
use std::path::PathBuf;
use std::process::Command;

pub struct AIUsageService;

impl AIUsageService {
    pub fn new() -> Self {
        Self
    }

    /// Get Claude Code usage information
    pub async fn get_claude_usage(&self) -> anyhow::Result<Option<ClaudeUsage>> {
        // Try to read Claude Code configuration and auth files
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

        let claude_dir = home.join(".claude");
        let settings_path = claude_dir.join("settings.json");

        if !settings_path.exists() {
            return Ok(None);
        }

        // Read settings
        let settings_content = tokio::fs::read_to_string(&settings_path).await?;
        let settings: serde_json::Value = serde_json::from_str(&settings_content)?;

        // Try to get usage from Claude CLI if available
        let usage = self.query_claude_cli_usage().await;

        // If CLI query fails, return basic info from settings
        if usage.is_none() {
            return Ok(Some(ClaudeUsage {
                tier: settings["tier"].as_str().unwrap_or("free").to_string(),
                requests_used: 0,
                requests_limit: None,
                tokens_used: 0,
                reset_time: None,
            }));
        }

        Ok(usage)
    }

    /// Try to query Claude CLI for usage information
    async fn query_claude_cli_usage(&self) -> Option<ClaudeUsage> {
        // Check if claude CLI is available
        let output = Command::new("claude")
            .args(["--version"])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        // Try to get usage via claude status command
        // Note: This is hypothetical as Claude CLI may not expose this directly
        let output = Command::new("claude")
            .args(["status", "--json"])
            .output()
            .ok()?;

        if output.status.success() {
            let json: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
            
            return Some(ClaudeUsage {
                tier: json["tier"].as_str()?.to_string(),
                requests_used: json["requests_used"].as_u64()? as u32,
                requests_limit: json["requests_limit"].as_u64().map(|v| v as u32),
                tokens_used: json["tokens_used"].as_u64()?,
                reset_time: json["reset_time"]
                    .as_str()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&chrono::Utc)),
            });
        }

        None
    }

    /// Get GitHub Copilot usage information
    pub async fn get_copilot_usage(&self) -> anyhow::Result<Option<CopilotUsage>> {
        // Check VS Code settings for Copilot
        let app_data = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;

        // VS Code settings paths
        let vscode_settings = app_data
            .join("Code")
            .join("User")
            .join("settings.json");

        let copilot_dir = app_data
            .join("Code")
            .join("User")
            .join("globalStorage")
            .join("github.copilot");

        // Check if Copilot is installed
        if !copilot_dir.exists() && !vscode_settings.exists() {
            return Ok(None);
        }

        // Try to read Copilot stats if available
        let stats = self.read_copilot_stats(&copilot_dir).await;

        Ok(Some(CopilotUsage {
            suggestions_accepted: stats.as_ref().map(|s| s.accepted).unwrap_or(0),
            suggestions_shown: stats.as_ref().map(|s| s.shown).unwrap_or(0),
            active: copilot_dir.exists(),
        }))
    }

    /// Read Copilot statistics from VS Code extension data
    async fn read_copilot_stats(&self, copilot_dir: &PathBuf) -> Option<CopilotStats> {
        // Look for telemetry or stats files
        let stats_file = copilot_dir.join("stats.json");
        
        if stats_file.exists() {
            let content = tokio::fs::read_to_string(&stats_file).await.ok()?;
            let json: serde_json::Value = serde_json::from_str(&content).ok()?;
            
            return Some(CopilotStats {
                accepted: json["suggestions_accepted"].as_u64()? as u32,
                shown: json["suggestions_shown"].as_u64()? as u32,
            });
        }

        None
    }

    /// Get Ollama status and available models
    pub async fn get_ollama_status(&self) -> anyhow::Result<OllamaStatus> {
        let client = reqwest::Client::new();

        match client
            .get("http://localhost:11434/api/tags")
            .timeout(std::time::Duration::from_secs(3))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                let result: serde_json::Value = response.json().await?;
                let models: Vec<String> = result["models"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
                    .collect();

                // Try to get version
                let version = self.get_ollama_version().await.ok();

                Ok(OllamaStatus {
                    running: true,
                    version,
                    models,
                })
            }
            _ => Ok(OllamaStatus {
                running: false,
                version: None,
                models: vec![],
            }),
        }
    }

    /// Get Ollama version
    async fn get_ollama_version(&self) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let response = client
            .get("http://localhost:11434/api/version")
            .timeout(std::time::Duration::from_secs(2))
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            if let Some(version) = result["version"].as_str() {
                return Ok(version.to_string());
            }
        }

        Err(anyhow::anyhow!("Could not get Ollama version"))
    }

    /// Check if a specific AI tool is installed
    pub async fn check_tool_availability(&self, tool: &str) -> bool {
        match tool {
            "claude" => {
                Command::new("claude")
                    .args(["--version"])
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            }
            "ollama" => {
                self.get_ollama_status()
                    .await
                    .map(|status| status.running)
                    .unwrap_or(false)
            }
            "copilot" => {
                // Check VS Code extension
                let app_data = dirs::config_dir();
                match app_data {
                    Some(data) => {
                        let copilot_dir = data
                            .join("Code")
                            .join("User")
                            .join("globalStorage")
                            .join("github.copilot");
                        copilot_dir.exists()
                    }
                    None => false,
                }
            }
            _ => false,
        }
    }

    /// Get a summary of all AI tools status
    pub async fn get_ai_tools_summary(&self) -> serde_json::Value {
        let claude_available = self.check_tool_availability("claude").await;
        let ollama_available = self.check_tool_availability("ollama").await;
        let copilot_available = self.check_tool_availability("copilot").await;

        serde_json::json!({
            "claude": {
                "available": claude_available,
                "name": "Claude Code"
            },
            "ollama": {
                "available": ollama_available,
                "name": "Ollama"
            },
            "copilot": {
                "available": copilot_available,
                "name": "GitHub Copilot"
            }
        })
    }
}

struct CopilotStats {
    accepted: u32,
    shown: u32,
}
