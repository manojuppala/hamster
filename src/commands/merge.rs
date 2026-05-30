use anyhow::Result;
use std::collections::HashMap;
use crate::error::HamsterError;
use crate::repository::Repository;
use crate::objects::{Commit, Tree};
use crate::utils::find_repo_root;
use crate::core::{get_author_name, get_author_email};

pub fn execute(branch: String) -> Result<()> {
    let repo_root = find_repo_root()?;
    let repo = Repository::new(repo_root.clone());
    
    let current_branch = repo.get_current_branch()?;
    
    if current_branch == branch {
        println!("Already on '{}'", branch);
        return Ok(());
    }
    
    // Get branch reference
    let branch_path = repo.refs_dir().join("heads").join(&branch);
    if !branch_path.exists() {
        return Err(HamsterError::BranchNotFound(branch.clone()).into());
    }
    
    let branch_commit_hash = std::fs::read_to_string(&branch_path)?.trim().to_string();
    let current_commit_hash = repo.resolve_ref(&repo.get_head()?)?;
    
    let branch_commit = repo.read_commit(&branch_commit_hash)?;
    let current_commit = repo.read_commit(&current_commit_hash)?;
    
    // Simple merge: just take the other branch's tree
    // (A real implementation would do a 3-way merge)
    let merged_tree = branch_commit.tree.clone();
    
    // Create merge commit
    let author = get_author_name();
    let email = get_author_email();
    let message = format!("Merge branch '{}'", branch);
    
    let merge_commit = Commit::new(
        merged_tree,
        vec![current_commit_hash, branch_commit_hash],
        author,
        email,
        message,
    );
    
    let merge_commit_hash = repo.write_commit(&merge_commit)?;
    
    // Update current branch
    let current_branch_path = repo.refs_dir().join("heads").join(&current_branch);
    std::fs::write(&current_branch_path, &merge_commit_hash)?;
    
    println!("Merged '{}' into '{}'", branch, current_branch);
    println!("Merge commit: {}", &merge_commit_hash[..7]);
    
    Ok(())
}
