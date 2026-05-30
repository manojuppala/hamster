use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::error::Result;
use crate::utils::get_hamster_dir;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Remote {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub remotes: BTreeMap<String, Remote>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn load() -> Result<Self> {
        let config_path = get_hamster_dir()?.join("config");
        
        if !config_path.exists() {
            return Ok(Self::new());
        }
        
        let data = std::fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&data)?;
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = get_hamster_dir()?.join("config");
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, data)?;
        Ok(())
    }
    
    pub fn add_remote(&mut self, name: String, url: String) {
        self.remotes.insert(name.clone(), Remote { name, url });
    }
    
    pub fn remove_remote(&mut self, name: &str) {
        self.remotes.remove(name);
    }
    
    pub fn get_remote(&self, name: &str) -> Option<&Remote> {
        self.remotes.get(name)
    }
}
