use std::collections::VecDeque;
use std::sync::Arc;
use parking_lot::RwLock;
use tauri::AppHandle;
use crate::models::{ChatMessage, LocalModel, ModelConfig, ModelProvider, MessageRole};

pub struct ChatService {
    history: Arc<RwLock<VecDeque<ChatMessage>>>,
    max_history: usize,
}

impl ChatService {
    pub fn new() -> Self {
        Self {
            history: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
            max_history: 100,
        }
    }
    
    pub async fn send_message(
        &self,
        message: &ChatMessage,
        config: &ModelConfig,
    ) -> anyhow::Result<ChatMessage> {
        match config.provider {
            ModelProvider::Ollama => self.send_ollama_message(message, config).await,
            ModelProvider::OpenAI => self.send_openai_message(message, config).await,
            ModelProvider::Gemini => self.send_gemini_message(message, config).await,
            ModelProvider::Custom => self.send_custom_message(message, config).await,
        }
    }
    
    async fn send_ollama_message(
        &self,
        message: &ChatMessage,
        config: &ModelConfig,
    ) -> anyhow::Result<ChatMessage> {
        let base_url = config.base_url.as_deref().unwrap_or("http://localhost:11434");
        let url = format!("{}/api/chat", base_url);
        
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&serde_json::json!({
                "model": config.model_name,
                "messages": [{
                    "role": "user",
                    "content": message.content
                }],
                "stream": false
            }))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Ollama request failed: {}", response.status()));
        }
        
        let result: serde_json::Value = response.json().await?;
        let content = result["message"]["content"]
            .as_str()
            .unwrap_or("No response")
            .to_string();
        
        Ok(ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            role: MessageRole::Assistant,
            content,
            timestamp: chrono::Local::now(),
        })
    }
    
    async fn send_openai_message(
        &self,
        message: &ChatMessage,
        config: &ModelConfig,
    ) -> anyhow::Result<ChatMessage> {
        let base_url = config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/chat/completions", base_url);
        
        let api_key = config.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("API key not configured"))?;
        
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&serde_json::json!({
                "model": config.model_name,
                "messages": [{
                    "role": "user",
                    "content": message.content
                }]
            }))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenAI request failed: {}", response.status()));
        }
        
        let result: serde_json::Value = response.json().await?;
        let content = result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response")
            .to_string();
        
        Ok(ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            role: MessageRole::Assistant,
            content,
            timestamp: chrono::Local::now(),
        })
    }
    
    async fn send_gemini_message(
        &self,
        message: &ChatMessage,
        config: &ModelConfig,
    ) -> anyhow::Result<ChatMessage> {
        // Gemini API implementation
        Err(anyhow::anyhow!("Gemini API is not yet supported"))
    }
    
    async fn send_custom_message(
        &self,
        message: &ChatMessage,
        config: &ModelConfig,
    ) -> anyhow::Result<ChatMessage> {
        // Custom OpenAI-compatible API
        self.send_openai_message(message, config).await
    }
    
    pub async fn get_available_models(&self) -> anyhow::Result<Vec<LocalModel>> {
        // Check if Ollama is running
        let client = reqwest::Client::new();
        match client.get("http://localhost:11434/api/tags").send().await {
            Ok(response) if response.status().is_success() => {
                let result: serde_json::Value = response.json().await?;
                let models: Vec<LocalModel> = result["models"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|m| LocalModel {
                        name: m["name"].as_str().unwrap_or("unknown").to_string(),
                        size: m["size"].as_u64().map(|s| format!("{:.1}GB", s as f64 / 1e9)).unwrap_or_default(),
                        description: String::new(),
                        downloaded: true,
                    })
                    .collect();
                Ok(models)
            }
            _ => Ok(vec![]),
        }
    }
    
    pub async fn test_connection(&self, config: &ModelConfig) -> anyhow::Result<bool> {
        match config.provider {
            ModelProvider::Ollama => {
                let base_url = config.base_url.as_deref().unwrap_or("http://localhost:11434");
                let client = reqwest::Client::new();
                let response = client.get(format!("{}/api/tags", base_url)).send().await?;
                Ok(response.status().is_success())
            }
            ModelProvider::OpenAI | ModelProvider::Custom => {
                // Test with a simple request
                let test_message = ChatMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    role: MessageRole::User,
                    content: "Hello".to_string(),
                    timestamp: chrono::Local::now(),
                };
                match self.send_message(&test_message, config).await {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
            _ => Ok(false),
        }
    }
    
    pub async fn add_message(&self, message: ChatMessage) {
        let mut history = self.history.write();
        if history.len() >= self.max_history {
            history.pop_front();
        }
        history.push_back(message);
    }
    
    pub async fn get_history(&self) -> Vec<ChatMessage> {
        self.history.read().iter().cloned().collect()
    }
    
    pub async fn clear_history(&self) {
        self.history.write().clear();
    }
}

pub fn init(app: &AppHandle) {
    let service = ChatService::new();
    app.manage(service);
}
