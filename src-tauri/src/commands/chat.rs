use tauri::AppHandle;
use crate::models::{ChatMessage, ChatSession, LocalModel, ModelConfig};
use crate::services::chat_service::ChatService;

#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    message: String,
    model_config: ModelConfig,
) -> Result<ChatMessage, String> {
    let service = app.state::<ChatService>();
    
    let user_message = ChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        role: crate::models::MessageRole::User,
        content: message,
        timestamp: chrono::Local::now(),
    };
    
    service.add_message(user_message.clone()).await;
    
    // Get AI response
    let response = service
        .send_message(&user_message, &model_config)
        .await
        .map_err(|e| e.to_string())?;
    
    service.add_message(response.clone()).await;
    
    Ok(response)
}

#[tauri::command]
pub async fn get_chat_history(app: AppHandle) -> Result<Vec<ChatMessage>, String> {
    let service = app.state::<ChatService>();
    Ok(service.get_history().await)
}

#[tauri::command]
pub async fn clear_chat_history(app: AppHandle) -> Result<(), String> {
    let service = app.state::<ChatService>();
    service.clear_history().await;
    Ok(())
}

#[tauri::command]
pub async fn get_available_models(app: AppHandle) -> Result<Vec<LocalModel>, String> {
    let service = app.state::<ChatService>();
    service.get_available_models().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_model_connection(
    app: AppHandle,
    model_config: ModelConfig,
) -> Result<bool, String> {
    let service = app.state::<ChatService>();
    service
        .test_connection(&model_config)
        .await
        .map_err(|e| e.to_string())
}
