use anyhow::Result;
use std::fs;
use crate::error::HamsterError;
use crate::repository::Repository;
use crate::utils::find_repo_root;

pub fn execute(name: Option<String>, delete: bool) -> Result<()> {
    let repo_root = find_repo_root()?;
    let repo = Repository::new(repo_root);
    
    if delete {
        // Delete branch
        if let Some(branch_name) = name {
            delete_branch(&repo, &branch_name)?;
        } else {
            return Err(HamsterError::Unknown("Branch name required for deletion".to_string()).into());
        }
    } else if let Some(branch_name) = name {
        // Create new branch
        create_branch(&repo, &branch_name)?;
    } else {
        // List branches
        list_branches(&repo)?;
    }
    
    Ok(())
}

fn create_branch(repo: &Repository, name: &str) -> Result<()> {
    let branch_path = repo.refs_dir().join("heads").join(name);
    
    if branch_path.exists() {
        return Err(HamsterError::BranchExists(name.to_string()).into());
    }
    
    // Get current HEAD commit
    let head = repo.get_head()?;
    let commit_hash = repo.resolve_ref(&head)?;
    
    if commit_hash.is_empty() || commit_hash == head {
        return Err(HamsterError::NoCommits.into());
    }
    
    fs::write(&branch_path, &commit_hash)?;
    println!("Created branch '{}'", name);
    
    Ok(())
}

fn delete_branch(repo: &Repository, name: &str) -> Result<()> {
    let current_branch = repo.get_current_branch()?;
    
    if current_branch == name {
        return Err(HamsterError::CannotDeleteCurrentBranch.into());
    }
    
    let branch_path = repo.refs_dir().join("heads").join(name);
    
    if !branch_path.exists() {
        return Err(HamsterError::BranchNotFound(name.to_string()).into());
    }
    
    fs::remove_file(&branch_path)?;
    println!("Deleted branch '{}'", name);
    
    Ok(())
}

fn list_branches(repo: &Repository) -> Result<()> {
    let heads_dir = repo.refs_dir().join("heads");
    
    if !heads_dir.exists() {
        println!("No branches yet");
        return Ok(());
    }
    
    let current_branch = repo.get_current_branch().ok();
    
    for entry in fs::read_dir(&heads_dir)? {
        let entry = entry?;
        let branch_name = entry.file_name().to_string_lossy().to_string();
        
        if Some(&branch_name) == current_branch.as_ref() {
            println!("* {}", branch_name);
        } else {
            println!("  {}", branch_name);
        }
    }
    
    Ok(())
}
