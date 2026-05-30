use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use walkdir::WalkDir;
use rayon::prelude::*;
use crate::index::Index;
use crate::repository::Repository;
use crate::utils::{find_repo_root, get_relative_path, read_file_bytes, should_ignore};
use crate::core::hash_object;

pub fn execute() -> Result<()> {
    let repo_root = find_repo_root()?;
    let repo = Repository::new(repo_root.clone());
    let index = Index::load()?;
    
    // Get current branch
    let current_branch = repo.get_current_branch()
        .unwrap_or_else(|_| "HEAD (detached)".to_string());
    
    println!("On branch {}", current_branch);
    
    // Collect all file entries first
    let file_entries: Vec<_> = WalkDir::new(&repo_root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().is_file() && !should_ignore(entry.path()))
        .collect();

    // Process files in parallel for better performance on large repositories
    let working_files = Mutex::new(HashMap::new());

    file_entries.par_iter().for_each(|entry| {
        let path = entry.path();
        if let Ok(rel_path) = get_relative_path(path) {
            let rel_path_str = rel_path.to_string_lossy().to_string();
            if let Ok(content) = read_file_bytes(path) {
                let blob_data = create_blob_data(&content);
                let hash = hash_object(&blob_data);
                working_files.lock().unwrap().insert(rel_path_str, hash);
            }
        }
    });

    let working_files = working_files.into_inner().unwrap();
    
    // Track changes
    let mut staged_files = Vec::new();
    let mut modified_files = Vec::new();
    let mut deleted_files = Vec::new();
    let mut untracked_files = Vec::new();
    
    let mut indexed_paths = HashSet::new();
    
    // Check staged files
    for (path, entry) in &index.entries {
        indexed_paths.insert(path.clone());
        
        if let Some(working_hash) = working_files.get(path) {
            if working_hash != &entry.hash {
                modified_files.push(path.clone());
            }
        } else {
            deleted_files.push(path.clone());
        }
        
        staged_files.push(path.clone());
    }
    
    // Check for untracked files
    for path in working_files.keys() {
        if !indexed_paths.contains(path) {
            untracked_files.push(path.clone());
        }
    }
    
    // Display status
    if !staged_files.is_empty() {
        println!("\nChanges to be committed:");
        for file in &staged_files {
            println!("  \x1b[32mnew file:   {}\x1b[0m", file);
        }
    }
    
    if !modified_files.is_empty() {
        println!("\nChanges not staged for commit:");
        for file in &modified_files {
            println!("  \x1b[31mmodified:   {}\x1b[0m", file);
        }
    }
    
    if !deleted_files.is_empty() {
        println!("\nDeleted files:");
        for file in &deleted_files {
            println!("  \x1b[31mdeleted:    {}\x1b[0m", file);
        }
    }
    
    if !untracked_files.is_empty() {
        println!("\nUntracked files:");
        for file in &untracked_files {
            println!("  \x1b[31m{}\x1b[0m", file);
        }
    }
    
    if staged_files.is_empty() && modified_files.is_empty() && 
       deleted_files.is_empty() && untracked_files.is_empty() {
        println!("\nNothing to commit, working tree clean");
    }
    
    Ok(())
}

fn create_blob_data(content: &[u8]) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(b"blob ");
    data.extend_from_slice(content.len().to_string().as_bytes());
    data.push(0);
    data.extend_from_slice(content);
    data
}
