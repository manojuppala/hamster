use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::error::HamsterError;
use crate::config::Config;
use crate::repository::Repository;
use crate::utils::find_repo_root;

pub fn execute(remote: Option<String>, branch: Option<String>) -> Result<()> {
    let repo_root = find_repo_root()?;
    let repo = Repository::new(repo_root.clone());
    let config = Config::load()?;
    
    let remote_name = remote.unwrap_or_else(|| "origin".to_string());
    let branch_name = branch.unwrap_or_else(|| {
        repo.get_current_branch().unwrap_or_else(|_| "main".to_string())
    });
    
    let remote_config = config.get_remote(&remote_name)
        .ok_or_else(|| HamsterError::RemoteNotFound(remote_name.clone()))?;
    
    // For local remotes, copy objects
    let remote_path = PathBuf::from(&remote_config.url);
    if remote_path.exists() {
        push_local(&repo, &remote_path, &branch_name)?;
    } else {
        return Err(HamsterError::Unknown(
            "Remote HTTP push not yet implemented. Use local paths for now.".to_string()
        ).into());
    }
    
    println!("Pushed to {}:{}", remote_name, branch_name);
    
    Ok(())
}

fn push_local(repo: &Repository, remote_path: &PathBuf, branch: &str) -> Result<()> {
    let remote_hamster = remote_path.join(".hamster");
    
    if !remote_hamster.exists() {
        return Err(HamsterError::NotARepository.into());
    }
    
    // Copy objects directory
    let local_objects = repo.objects_dir();
    let remote_objects = remote_hamster.join("objects");
    
    for entry in walkdir::WalkDir::new(&local_objects) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let relative = path.strip_prefix(&local_objects)
                .map_err(|_| HamsterError::Unknown("Path error".to_string()))?;
            let dest = remote_objects.join(relative);
            
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            
            if !dest.exists() {
                fs::copy(path, &dest)?;
            }
        }
    }
    
    // Update remote branch reference
    let branch_ref_path = repo.refs_dir().join("heads").join(branch);
    let commit_hash = fs::read_to_string(&branch_ref_path)?;
    
    let remote_branch_ref = remote_hamster.join("refs").join("heads").join(branch);
    if let Some(parent) = remote_branch_ref.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&remote_branch_ref, commit_hash)?;
    
    Ok(())
}
