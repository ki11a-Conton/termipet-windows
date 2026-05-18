use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;
use tauri::AppHandle;
use crate::models::{Pet, PetState, PetAnimation};

pub struct PetService {
    current_state: Arc<RwLock<PetState>>,
    current_pet: Arc<RwLock<Option<Pet>>>,
    available_pets: Arc<RwLock<HashMap<String, Pet>>>,
}

impl PetService {
    pub fn new() -> Self {
        let mut service = Self {
            current_state: Arc::new(RwLock::new(PetState::Idle)),
            current_pet: Arc::new(RwLock::new(None)),
            available_pets: Arc::new(RwLock::new(HashMap::new())),
        };
        
        // Load built-in pets
        service.load_builtin_pets();
        
        service
    }
    
    fn load_builtin_pets(&mut self) {
        let mut pets = self.available_pets.write();
        
        // Terminal Cat (default)
        pets.insert(
            "terminal-cat".to_string(),
            Pet {
                id: "terminal-cat".to_string(),
                display_name: "Terminal Cat".to_string(),
                description: "A cute cat that accompanies you while coding".to_string(),
                spritesheet_path: "pets/terminal-cat/spritesheet.webp".to_string(),
                animations: vec![
                    PetAnimation { name: "idle".to_string(), row: 0, frames: 4, frame_duration_ms: 250 },
                    PetAnimation { name: "running".to_string(), row: 1, frames: 6, frame_duration_ms: 100 },
                    PetAnimation { name: "moving".to_string(), row: 2, frames: 4, frame_duration_ms: 150 },
                    PetAnimation { name: "happy".to_string(), row: 3, frames: 4, frame_duration_ms: 200 },
                    PetAnimation { name: "alert".to_string(), row: 4, frames: 2, frame_duration_ms: 500 },
                    PetAnimation { name: "error".to_string(), row: 5, frames: 4, frame_duration_ms: 200 },
                    PetAnimation { name: "sleeping".to_string(), row: 6, frames: 4, frame_duration_ms: 500 },
                    PetAnimation { name: "thinking".to_string(), row: 7, frames: 4, frame_duration_ms: 300 },
                    PetAnimation { name: "celebrating".to_string(), row: 8, frames: 6, frame_duration_ms: 150 },
                ],
            },
        );
        
        // Add more built-in pets here
    }
    
    pub async fn get_state(&self) -> PetState {
        *self.current_state.read()
    }
    
    pub async fn set_state(&self, state: PetState) {
        *self.current_state.write() = state;
    }
    
    pub async fn get_available_pets(&self) -> anyhow::Result<Vec<Pet>> {
        let pets = self.available_pets.read();
        Ok(pets.values().cloned().collect())
    }
    
    pub async fn load_pet(&self, pet_id: &str) -> anyhow::Result<Pet> {
        let pets = self.available_pets.read();
        pets.get(pet_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Pet not found: {}", pet_id))
    }
    
    pub async fn import_pet(&self, path: &Path) -> anyhow::Result<Pet> {
        // Read pet.json from the directory
        let pet_json_path = path.join("pet.json");
        let pet_json = tokio::fs::read_to_string(&pet_json_path).await?;
        let pet: Pet = serde_json::from_str(&pet_json)?;
        
        // Copy to imported pets directory
        let app_data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
            .join("TermiPet")
            .join("ImportedPets")
            .join(&pet.id);
        
        tokio::fs::create_dir_all(&app_data_dir).await?;
        
        // Copy files
        let entries = tokio::fs::read_dir(path).await?;
        // ... copy logic
        
        // Add to available pets
        self.available_pets.write().insert(pet.id.clone(), pet.clone());
        
        Ok(pet)
    }
}

pub fn init(app: &AppHandle) {
    let service = PetService::new();
    app.manage(service);
}
