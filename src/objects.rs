use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::error::{HamsterError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl ObjectType {
    pub fn as_str(&self) -> &str {
        match self {
            ObjectType::Blob => "blob",
            ObjectType::Tree => "tree",
            ObjectType::Commit => "commit",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "blob" => Ok(ObjectType::Blob),
            "tree" => Ok(ObjectType::Tree),
            "commit" => Ok(ObjectType::Commit),
            _ => Err(HamsterError::InvalidObjectType(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub obj_type: ObjectType,
    pub data: Vec<u8>,
}

impl Object {
    pub fn new(obj_type: ObjectType, data: Vec<u8>) -> Self {
        Self { obj_type, data }
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(self.obj_type.as_str().as_bytes());
        result.push(b' ');
        result.extend_from_slice(&self.data.len().to_string().as_bytes());
        result.push(0); // null byte
        result.extend_from_slice(&self.data);
        result
    }
    
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        let null_pos = data.iter().position(|&b| b == 0)
            .ok_or_else(|| HamsterError::Unknown("Invalid object format".to_string()))?;
        
        let header = std::str::from_utf8(&data[..null_pos])
            .map_err(|_| HamsterError::Unknown("Invalid UTF-8 in object header".to_string()))?;
        
        let parts: Vec<&str> = header.split(' ').collect();
        if parts.len() != 2 {
            return Err(HamsterError::Unknown("Invalid object header".to_string()));
        }
        
        let obj_type = ObjectType::from_str(parts[0])?;
        let obj_data = data[null_pos + 1..].to_vec();
        
        Ok(Object::new(obj_type, obj_data))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeEntry {
    pub mode: String,
    pub name: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }
    
    pub fn add_entry(&mut self, mode: String, name: String, hash: String) {
        self.entries.push(TreeEntry { mode, name, hash });
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
    
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        serde_json::from_slice(data)
            .map_err(|e| HamsterError::Serialization(e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub tree: String,
    pub parents: Vec<String>,
    pub author: String,
    pub email: String,
    pub timestamp: i64,
    pub message: String,
}

impl Commit {
    pub fn new(tree: String, parents: Vec<String>, author: String, email: String, message: String) -> Self {
        Self {
            tree,
            parents,
            author,
            email,
            timestamp: crate::core::current_timestamp(),
            message,
        }
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
    
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        serde_json::from_slice(data)
            .map_err(|e| HamsterError::Serialization(e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob {
    pub content: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }
}
