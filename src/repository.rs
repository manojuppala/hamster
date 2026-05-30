use std::path::{Path, PathBuf};
use std::io::{Read, Write, BufReader, BufWriter};
use std::collections::HashMap;
use std::cell::RefCell;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use crate::error::{HamsterError, Result};
use crate::objects::{Object, ObjectType, Commit, Tree, Blob};
use crate::core::hash_object;
use crate::utils::get_hamster_dir;

/// Object cache to reduce disk I/O for frequently accessed objects
type ObjectCache = RefCell<HashMap<String, Object>>;

pub struct Repository {
    pub root: PathBuf,
    /// Cache for frequently accessed objects (commits, trees)
    cache: ObjectCache,
}

impl Repository {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            cache: RefCell::new(HashMap::new()),
        }
    }

    /// Clear the object cache (useful for testing or memory management)
    pub fn clear_cache(&self) {
        self.cache.borrow_mut().clear();
    }

    /// Get cache size for monitoring
    pub fn cache_size(&self) -> usize {
        self.cache.borrow().len()
    }
    
    /// Get the objects directory
    pub fn objects_dir(&self) -> PathBuf {
        self.root.join(".hamster").join("objects")
    }
    
    /// Get the refs directory
    pub fn refs_dir(&self) -> PathBuf {
        self.root.join(".hamster").join("refs")
    }
    
    /// Get the HEAD file path
    pub fn head_path(&self) -> PathBuf {
        self.root.join(".hamster").join("HEAD")
    }
    
    /// Write an object to the object database
    pub fn write_object(&self, object: &Object) -> Result<String> {
        let serialized = object.serialize();
        let hash = hash_object(&serialized);

        // Check if object already exists (content-addressable deduplication)
        let object_dir = self.objects_dir().join(&hash[..2]);
        let object_path = object_dir.join(&hash[2..]);

        if object_path.exists() {
            // Object already exists, no need to write
            return Ok(hash);
        }

        std::fs::create_dir_all(&object_dir)?;

        // Use buffered writer for better I/O performance
        let file = std::fs::File::create(&object_path)?;
        let mut buf_writer = BufWriter::new(file);

        // Compress the object data
        let mut encoder = ZlibEncoder::new(&mut buf_writer, Compression::default());
        encoder.write_all(&serialized)?;
        encoder.finish()?;

        // Cache the object for potential re-reads
        self.cache.borrow_mut().insert(hash.clone(), object.clone());

        Ok(hash)
    }
    
    /// Read an object from the object database (with caching)
    pub fn read_object(&self, hash: &str) -> Result<Object> {
        if hash.len() < 3 {
            return Err(HamsterError::InvalidHash(hash.to_string()));
        }

        // Check cache first
        if let Some(obj) = self.cache.borrow().get(hash) {
            return Ok(obj.clone());
        }

        let object_path = self.objects_dir()
            .join(&hash[..2])
            .join(&hash[2..]);

        if !object_path.exists() {
            return Err(HamsterError::ObjectNotFound(hash.to_string()));
        }

        // Use buffered reader for better I/O performance
        let file = std::fs::File::open(&object_path)?;
        let buf_reader = BufReader::new(file);

        // Decompress the object data
        let mut decoder = ZlibDecoder::new(buf_reader);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        let object = Object::deserialize(&decompressed)?;

        // Cache the object for future reads
        self.cache.borrow_mut().insert(hash.to_string(), object.clone());

        Ok(object)
    }
    
    /// Write a blob object
    pub fn write_blob(&self, content: Vec<u8>) -> Result<String> {
        let object = Object::new(ObjectType::Blob, content);
        self.write_object(&object)
    }
    
    /// Write a tree object
    pub fn write_tree(&self, tree: &Tree) -> Result<String> {
        let object = Object::new(ObjectType::Tree, tree.serialize());
        self.write_object(&object)
    }
    
    /// Write a commit object
    pub fn write_commit(&self, commit: &Commit) -> Result<String> {
        let object = Object::new(ObjectType::Commit, commit.serialize());
        self.write_object(&object)
    }
    
    /// Read a commit object
    pub fn read_commit(&self, hash: &str) -> Result<Commit> {
        let object = self.read_object(hash)?;
        match object.obj_type {
            ObjectType::Commit => Commit::deserialize(&object.data),
            _ => Err(HamsterError::InvalidObjectType("Expected commit".to_string())),
        }
    }
    
    /// Read a tree object
    pub fn read_tree(&self, hash: &str) -> Result<Tree> {
        let object = self.read_object(hash)?;
        match object.obj_type {
            ObjectType::Tree => Tree::deserialize(&object.data),
            _ => Err(HamsterError::InvalidObjectType("Expected tree".to_string())),
        }
    }
    
    /// Get the current HEAD reference
    pub fn get_head(&self) -> Result<String> {
        let head_path = self.head_path();
        let content = std::fs::read_to_string(&head_path)?;
        Ok(content.trim().to_string())
    }
    
    /// Set the HEAD reference
    pub fn set_head(&self, reference: &str) -> Result<()> {
        let head_path = self.head_path();
        std::fs::write(&head_path, reference)?;
        Ok(())
    }
    
    /// Get the current branch name
    pub fn get_current_branch(&self) -> Result<String> {
        let head = self.get_head()?;
        if head.starts_with("ref: refs/heads/") {
            Ok(head.trim_start_matches("ref: refs/heads/").to_string())
        } else {
            Err(HamsterError::Unknown("Detached HEAD state".to_string()))
        }
    }
    
    /// Get the hash that a reference points to
    pub fn resolve_ref(&self, reference: &str) -> Result<String> {
        if reference.starts_with("ref: ") {
            let ref_path = self.root.join(".hamster").join(reference.trim_start_matches("ref: "));
            let content = std::fs::read_to_string(&ref_path)?;
            Ok(content.trim().to_string())
        } else {
            Ok(reference.to_string())
        }
    }
}
