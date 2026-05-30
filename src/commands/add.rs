use anyhow::Result;
use std::path::Path;
use std::sync::Mutex;
use walkdir::WalkDir;
use rayon::prelude::*;
use crate::error::HamsterError;
use crate::index::Index;
use crate::repository::Repository;
use crate::utils::{find_repo_root, get_relative_path, read_file_bytes, should_ignore};

pub fn execute(files: Vec<String>) -> Result<()> {
    let repo_root = find_repo_root()?;
    let repo = Repository::new(repo_root.clone());
    let mut index = Index::load()?;
    
    for file_pattern in files {
        let path = Path::new(&file_pattern);
        
        // Handle absolute and relative paths
        let full_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };
        
        if !full_path.exists() {
            return Err(HamsterError::FileNotFound(file_pattern).into());
        }
        
        if full_path.is_file() {
            add_file(&repo, &mut index, &full_path)?;
        } else if full_path.is_dir() {
            // Collect all files first
            let files: Vec<_> = WalkDir::new(&full_path)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry| entry.path().is_file() && !should_ignore(entry.path()))
                .map(|entry| entry.path().to_path_buf())
                .collect();

            // Process files in parallel for large directories
            let results: Vec<_> = files.par_iter()
                .map(|path| add_file_parallel(&repo, path))
                .collect();

            // Collect results and update index
            for result in results {
                if let Ok((path_str, hash, mode)) = result {
                    index.add(path_str, hash, mode);
                }
            }
        }
    }
    
    index.save()?;
    println!("Files added to staging area");
    
    Ok(())
}

fn add_file(repo: &Repository, index: &mut Index, path: &Path) -> Result<()> {
    if should_ignore(path) {
        return Ok(());
    }

    let content = read_file_bytes(path)?;
    let hash = repo.write_blob(content)?;

    let relative_path = get_relative_path(path)?;
    let path_str = relative_path.to_string_lossy().to_string();

    // Determine file mode (simplified version)
    let mode = if path.metadata()?.permissions().readonly() {
        "100644".to_string()
    } else {
        "100755".to_string()
    };

    index.add(path_str.clone(), hash, mode);

    Ok(())
}

/// Parallel version of add_file that returns data instead of modifying index
fn add_file_parallel(repo: &Repository, path: &Path) -> Result<(String, String, String)> {
    if should_ignore(path) {
        return Err(anyhow::anyhow!("Ignored file"));
    }

    let content = read_file_bytes(path)?;
    let hash = repo.write_blob(content)?;

    let relative_path = get_relative_path(path)?;
    let path_str = relative_path.to_string_lossy().to_string();

    // Determine file mode (simplified version)
    let mode = if path.metadata()?.permissions().readonly() {
        "100644".to_string()
    } else {
        "100755".to_string()
    };

    Ok((path_str, hash, mode))
}
