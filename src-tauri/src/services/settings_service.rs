use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreBuilder;
use crate::models::AppSettings;

pub struct SettingsService {
    app: AppHandle,
}

impl SettingsService {
    pub fn new(app: &AppHandle) -> Self {
        Self { app: app.clone() }
    }
    
    fn get_store_path(&self) -> PathBuf {
        self.app
            .path()
            .app_data_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("settings.json")
    }
    
    pub async fn get_settings(&self) -> anyhow::Result<AppSettings> {
        let store_path = self.get_store_path();
        
        if !store_path.exists() {
            return Ok(AppSettings::default());
        }
        
        let store = StoreBuilder::new(&self.app, store_path).build()?;
        
        match store.get("settings") {
            Some(value) => {
                let settings: AppSettings = serde_json::from_value(value)?;
                Ok(settings)
            }
            None => Ok(AppSettings::default()),
        }
    }
    
    pub async fn save_settings(&self, settings: &AppSettings) -> anyhow::Result<()> {
        let store_path = self.get_store_path();
        let store = StoreBuilder::new(&self.app, store_path).build()?;
        
        let value = serde_json::to_value(settings)?;
        store.set("settings", value);
        store.save()?;
        
        Ok(())
    }
    
    pub async fn get_api_key(&self, provider: &str) -> anyhow::Result<Option<String>> {
        // Use Windows Credential Manager
        #[cfg(windows)]
        {
            use windows::Win32::Security::Credentials::{
                CredReadW, CRED_TYPE_GENERIC, CREDENTIALW, CRED_TYPE,
            };
            use windows::core::PCWSTR;
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;
            
            let target_name = format!("TermiPet/{}", provider);
            let target_wide: Vec<u16> = OsStr::new(&target_name)
                .encode_wide()
                .chain(Some(0))
                .collect();
            
            let mut cred: *mut CREDENTIALW = std::ptr::null_mut();
            
            unsafe {
                let result = CredReadW(
                    PCWSTR(target_wide.as_ptr()),
                    CRED_TYPE_GENERIC,
                    0,
                    &mut cred,
                );
                
                if result.is_ok() && !cred.is_null() {
                    let credential = &*cred;
                    let password_slice = std::slice::from_raw_parts(
                        credential.CredentialBlob,
                        credential.CredentialBlobSize as usize,
                    );
                    let password = String::from_utf8_lossy(password_slice);
                    return Ok(Some(password.to_string()));
                }
            }
        }
        
        Ok(None)
    }
    
    pub async fn set_api_key(
        &self,
        provider: &str,
        api_key: Option<&str>,
    ) -> anyhow::Result<()> {
        #[cfg(windows)]
        {
            use windows::Win32::Security::Credentials::{
                CredWriteW, CredDeleteW, CRED_TYPE_GENERIC, CREDENTIALW,
            };
            use windows::core::PCWSTR;
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;
            
            let target_name = format!("TermiPet/{}", provider);
            let target_wide: Vec<u16> = OsStr::new(&target_name)
                .encode_wide()
                .chain(Some(0))
                .collect();
            
            if let Some(key) = api_key {
                // Write credential
                let key_bytes = key.as_bytes();
                let mut credential = CREDENTIALW {
                    Flags: 0,
                    Type: CRED_TYPE_GENERIC,
                    TargetName: PCWSTR(target_wide.as_ptr()),
                    Comment: PCWSTR::null(),
                    LastWritten: windows::Win32::Foundation::FILETIME::default(),
                    CredentialBlobSize: key_bytes.len() as u32,
                    CredentialBlob: key_bytes.as_ptr() as *mut u8,
                    Persist: 2, // CRED_PERSIST_LOCAL_MACHINE
                    AttributeCount: 0,
                    Attributes: std::ptr::null_mut(),
                    TargetAlias: PCWSTR::null(),
                    UserName: PCWSTR::null(),
                };
                
                unsafe {
                    CredWriteW(&mut credential, 0)?;
                }
            } else {
                // Delete credential
                unsafe {
                    let _ = CredDeleteW(
                        PCWSTR(target_wide.as_ptr()),
                        CRED_TYPE_GENERIC,
                        0,
                    );
                }
            }
        }
        
        Ok(())
    }
}
