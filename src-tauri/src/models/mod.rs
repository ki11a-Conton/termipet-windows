use serde::{Deserialize, Serialize};

// Pet models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pet {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub spritesheet_path: String,
    pub animations: Vec<PetAnimation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetAnimation {
    pub name: String,
    pub row: u32,
    pub frames: u32,
    pub frame_duration_ms: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PetState {
    Idle,
    Running,
    Moving,
    Happy,
    Alert,
    Error,
    Sleeping,
    Thinking,
    Celebrating,
}

impl Default for PetState {
    fn default() -> Self {
        PetState::Idle
    }
}

// Chat models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub messages: Vec<ChatMessage>,
    pub model: String,
    pub created_at: chrono::DateTime<chrono::Local>,
}

// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: ModelProvider,
    pub model_name: String,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ModelProvider {
    Ollama,
    OpenAI,
    Gemini,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModel {
    pub name: String,
    pub size: String,
    pub description: String,
    pub downloaded: bool,
}

// Settings models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: String,
    pub skin: AppSkin,
    pub pet_id: String,
    pub pet_name: String,
    pub owner_name: String,
    pub personality: PetPersonality,
    pub model_config: ModelConfig,
    pub shortcuts: Vec<CommandShortcut>,
    pub pomodoro_duration: u32,
    pub break_duration: u32,
    pub auto_start: bool,
    pub show_on_startup: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: "zh-CN".to_string(),
            skin: AppSkin::Glass,
            pet_id: "terminal-cat".to_string(),
            pet_name: "Terminal Cat".to_string(),
            owner_name: "Master".to_string(),
            personality: PetPersonality::default(),
            model_config: ModelConfig {
                provider: ModelProvider::Ollama,
                model_name: "qwen2.5:1.5b".to_string(),
                base_url: Some("http://localhost:11434".to_string()),
                api_key: None,
            },
            shortcuts: vec![],
            pomodoro_duration: 25,
            break_duration: 5,
            auto_start: false,
            show_on_startup: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AppSkin {
    Glass,
    Dark,
    Pixel,
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetPersonality {
    pub preset: String,
    pub custom_prompt: Option<String>,
    pub additional_constraints: Option<String>,
}

impl Default for PetPersonality {
    fn default() -> Self {
        Self {
            preset: "friendly".to_string(),
            custom_prompt: None,
            additional_constraints: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandShortcut {
    pub id: String,
    pub name: String,
    pub command: String,
    pub pinned: bool,
    pub order: u32,
}

// Terminal models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalInfo {
    pub id: String,
    pub name: String,
    pub process_name: String,
    pub window_title: String,
    pub current_directory: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalOutput {
    pub summary: String,
    pub status: TerminalStatus,
    pub last_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalStatus {
    Idle,
    Running,
    Error,
    Waiting,
}

// AI Usage models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeUsage {
    pub tier: String,
    pub requests_used: u32,
    pub requests_limit: Option<u32>,
    pub tokens_used: u64,
    pub reset_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotUsage {
    pub suggestions_accepted: u32,
    pub suggestions_shown: u32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaStatus {
    pub running: bool,
    pub version: Option<String>,
    pub models: Vec<String>,
}

// Timer models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub active: bool,
    pub mode: TimerMode,
    pub remaining_seconds: u32,
    pub total_seconds: u32,
    pub started_at: Option<chrono::DateTime<chrono::Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TimerMode {
    Pomodoro,
    Break,
}

// System models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_version: String,
    pub app_version: String,
    pub accessibility_enabled: bool,
}

// Window position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}
