use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use crate::error::Result;
use crate::utils::get_hamster_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub path: String,
    pub hash: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Index {
    pub entries: BTreeMap<String, IndexEntry>,
}

impl Index {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn load() -> Result<Self> {
        let index_path = get_hamster_dir()?.join("index");
        
        if !index_path.exists() {
            return Ok(Self::new());
        }
        
        let data = std::fs::read(&index_path)?;
        let index: Index = serde_json::from_slice(&data)?;
        Ok(index)
    }
    
    pub fn save(&self) -> Result<()> {
        let index_path = get_hamster_dir()?.join("index");
        let data = serde_json::to_vec_pretty(self)?;
        std::fs::write(&index_path, data)?;
        Ok(())
    }
    
    pub fn add(&mut self, path: String, hash: String, mode: String) {
        self.entries.insert(
            path.clone(),
            IndexEntry { path, hash, mode },
        );
    }
    
    pub fn remove(&mut self, path: &str) {
        self.entries.remove(path);
    }
    
    pub fn get(&self, path: &str) -> Option<&IndexEntry> {
        self.entries.get(path)
    }
    
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
