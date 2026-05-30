use anyhow::Result;
use std::fs;
use std::path::Path;
use rayon::prelude::*;
use crate::error::HamsterError;
use crate::repository::Repository;
use crate::utils::find_repo_root;

pub fn execute(target: String) -> Result<()> {
    let repo_root = find_repo_root()?;
    let repo = Repository::new(repo_root.clone());
    
    // Check if target is a branch
    let branch_path = repo.refs_dir().join("heads").join(&target);
    
    if branch_path.exists() {
        // Switch to branch
        checkout_branch(&repo, &target)?;
    } else {
        // Try to checkout as commit hash
        checkout_commit(&repo, &target)?;
    }
    
    Ok(())
}

fn checkout_branch(repo: &Repository, branch: &str) -> Result<()> {
    let branch_ref = format!("ref: refs/heads/{}", branch);
    repo.set_head(&branch_ref)?;
    
    // Update working directory
    update_working_directory(repo, branch)?;
    
    println!("Switched to branch '{}'", branch);
    
    Ok(())
}

fn checkout_commit(repo: &Repository, commit_hash: &str) -> Result<()> {
    // Verify commit exists
    let _ = repo.read_commit(commit_hash)?;
    
    repo.set_head(commit_hash)?;
    println!("HEAD is now at {} (detached)", &commit_hash[..7]);
    
    Ok(())
}

fn update_working_directory(repo: &Repository, branch: &str) -> Result<()> {
    // Read the branch's commit
    let branch_path = repo.refs_dir().join("heads").join(branch);
    let commit_hash = fs::read_to_string(&branch_path)?;
    let commit = repo.read_commit(commit_hash.trim())?;

    // Read the tree
    let tree = repo.read_tree(&commit.tree)?;

    // Process files in parallel for faster checkout on large repositories
    let results: Vec<_> = tree.entries.par_iter()
        .map(|entry| {
            let file_path = repo.root.join(&entry.name);

            // Read blob content
            let object = repo.read_object(&entry.hash)?;

            // Create parent directories if needed
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Write file
            fs::write(&file_path, &object.data)?;

            Ok(())
        })
        .collect();

    // Check for errors
    for result in results {
        result?;
    }

    Ok(())
}
