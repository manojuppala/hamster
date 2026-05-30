use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::error::HamsterError;
use crate::config::Config;

pub fn execute(url: String, directory: Option<String>) -> Result<()> {
    let target_dir = if let Some(dir) = directory {
        PathBuf::from(dir)
    } else {
        // Extract repository name from URL
        let name = url.split('/').last()
            .unwrap_or("repository")
            .trim_end_matches(".hamster")
            .to_string();
        PathBuf::from(name)
    };
    
    if target_dir.exists() {
        return Err(HamsterError::Unknown(
            format!("Directory '{}' already exists", target_dir.display())
        ).into());
    }
    
    // Create target directory
    fs::create_dir_all(&target_dir)?;
    
    // Check if it's a local path
    let source = PathBuf::from(&url);
    if source.exists() && source.is_dir() {
        clone_local(&source, &target_dir)?;
    } else {
        return Err(HamsterError::Unknown(
            "Remote HTTP cloning not yet implemented. Use local paths for now.".to_string()
        ).into());
    }
    
    // Add origin remote
    std::env::set_current_dir(&target_dir)?;
    let mut config = Config::load()?;
    config.add_remote("origin".to_string(), url.clone());
    config.save()?;
    
    println!("Cloned repository to {}", target_dir.display());
    
    Ok(())
}

fn clone_local(source: &PathBuf, target: &PathBuf) -> Result<()> {
    let source_hamster = source.join(".hamster");
    
    if !source_hamster.exists() {
        return Err(HamsterError::NotARepository.into());
    }
    
    // Copy .hamster directory recursively
    copy_dir_all(&source_hamster, &target.join(".hamster"))?;
    
    // Checkout working directory files
    // This is simplified - a real implementation would checkout HEAD
    
    Ok(())
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        
        if path.is_dir() {
            copy_dir_all(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    
    Ok(())
}
