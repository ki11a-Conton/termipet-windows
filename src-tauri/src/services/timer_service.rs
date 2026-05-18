use std::sync::Arc;
use parking_lot::RwLock;
use tauri::AppHandle;
use crate::models::{TimerState, TimerMode};

pub struct TimerService {
    state: Arc<RwLock<TimerState>>,
}

impl TimerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(TimerState {
                active: false,
                mode: TimerMode::Pomodoro,
                remaining_seconds: 0,
                total_seconds: 0,
                started_at: None,
            })),
        }
    }
    
    pub async fn start(&self, mode: TimerMode, duration_seconds: u32) -> anyhow::Result<TimerState> {
        let mut state = self.state.write();
        
        state.active = true;
        state.mode = mode;
        state.remaining_seconds = duration_seconds;
        state.total_seconds = duration_seconds;
        state.started_at = Some(chrono::Local::now());
        
        // Start countdown task
        let state_clone = self.state.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                
                let mut state = state_clone.write();
                if !state.active {
                    break;
                }
                
                if state.remaining_seconds > 0 {
                    state.remaining_seconds -= 1;
                } else {
                    // Timer completed
                    state.active = false;
                    // TODO: Send notification
                    break;
                }
            }
        });
        
        Ok(state.clone())
    }
    
    pub async fn stop(&self) -> anyhow::Result<()> {
        let mut state = self.state.write();
        state.active = false;
        Ok(())
    }
    
    pub async fn get_state(&self) -> TimerState {
        self.state.read().clone()
    }
}

pub fn init(app: &AppHandle) {
    let service = TimerService::new();
    app.manage(service);
}
