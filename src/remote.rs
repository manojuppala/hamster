// Remote repository management
// This module provides functionality for working with remote repositories

use crate::error::Result;
use std::path::PathBuf;

pub struct Remote {
    pub name: String,
    pub url: String,
}

impl Remote {
    pub fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
    
    pub fn is_local(&self) -> bool {
        !self.url.starts_with("http://") && !self.url.starts_with("https://")
    }
    
    pub fn get_path(&self) -> Option<PathBuf> {
        if self.is_local() {
            Some(PathBuf::from(&self.url))
        } else {
            None
        }
    }
}
