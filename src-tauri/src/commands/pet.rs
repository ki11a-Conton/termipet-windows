use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use crate::models::{Pet, PetState};
use crate::services::pet_service::PetService;

#[tauri::command]
pub async fn get_pet_state(app: AppHandle) -> Result<PetState, String> {
    let service = app.state::<PetService>();
    Ok(service.get_state().await)
}

#[tauri::command]
pub async fn set_pet_state(app: AppHandle, state: PetState) -> Result<(), String> {
    let service = app.state::<PetService>();
    service.set_state(state).await;
    Ok(())
}

#[tauri::command]
pub async fn get_available_pets(app: AppHandle) -> Result<Vec<Pet>, String> {
    let service = app.state::<PetService>();
    service.get_available_pets().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn load_pet(app: AppHandle, pet_id: String) -> Result<Pet, String> {
    let service = app.state::<PetService>();
    service.load_pet(&pet_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_pet(app: AppHandle, path: String) -> Result<Pet, String> {
    let service = app.state::<PetService>();
    let path = PathBuf::from(path);
    service.import_pet(&path).await.map_err(|e| e.to_string())
}
