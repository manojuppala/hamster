use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::error::HamsterError;
use crate::config::Config;
use crate::repository::Repository;
use crate::utils::find_repo_root;

pub fn execute(remote: Option<String>) -> Result<()> {
    let repo_root = find_repo_root()?;
    let repo = Repository::new(repo_root.clone());
    let config = Config::load()?;
    
    let remote_name = remote.unwrap_or_else(|| "origin".to_string());
    
    let remote_config = config.get_remote(&remote_name)
        .ok_or_else(|| HamsterError::RemoteNotFound(remote_name.clone()))?;
    
    // For local remotes, copy objects
    let remote_path = PathBuf::from(&remote_config.url);
    if remote_path.exists() {
        fetch_local(&repo, &remote_path, &remote_name)?;
    } else {
        return Err(HamsterError::Unknown(
            "Remote HTTP fetch not yet implemented. Use local paths for now.".to_string()
        ).into());
    }
    
    println!("Fetched from {}", remote_name);
    
    Ok(())
}

fn fetch_local(repo: &Repository, remote_path: &PathBuf, remote_name: &str) -> Result<()> {
    let remote_hamster = remote_path.join(".hamster");
    
    if !remote_hamster.exists() {
        return Err(HamsterError::NotARepository.into());
    }
    
    // Copy objects from remote
    let remote_objects = remote_hamster.join("objects");
    let local_objects = repo.objects_dir();
    
    for entry in walkdir::WalkDir::new(&remote_objects) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let relative = path.strip_prefix(&remote_objects)
                .map_err(|_| HamsterError::Unknown("Path error".to_string()))?;
            let dest = local_objects.join(relative);
            
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            
            if !dest.exists() {
                fs::copy(path, &dest)?;
            }
        }
    }
    
    // Update remote tracking branches
    let remote_refs = remote_hamster.join("refs").join("heads");
    let local_remote_refs = repo.refs_dir().join("remotes").join(remote_name);
    
    fs::create_dir_all(&local_remote_refs)?;
    
    if remote_refs.exists() {
        for entry in fs::read_dir(&remote_refs)? {
            let entry = entry?;
            let branch_name = entry.file_name();
            let remote_branch_hash = fs::read_to_string(entry.path())?;
            let local_tracking_ref = local_remote_refs.join(&branch_name);
            fs::write(&local_tracking_ref, remote_branch_hash)?;
        }
    }
    
    Ok(())
}
