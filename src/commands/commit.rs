use anyhow::Result;
use crate::error::HamsterError;
use crate::index::Index;
use crate::repository::Repository;
use crate::objects::{Commit, Tree};
use crate::core::{get_author_name, get_author_email};
use crate::utils::find_repo_root;

pub fn execute(message: String) -> Result<()> {
    let repo_root = find_repo_root()?;
    let repo = Repository::new(repo_root.clone());
    let index = Index::load()?;
    
    if index.entries.is_empty() {
        return Err(HamsterError::NothingToCommit.into());
    }
    
    // Build tree from index (optimized with pre-allocated capacity)
    let mut tree = Tree::new();
    tree.entries.reserve(index.entries.len());

    // Add entries in sorted order for consistent tree hashes
    for (_, entry) in &index.entries {
        tree.add_entry(entry.mode.clone(), entry.path.clone(), entry.hash.clone());
    }

    // Sort entries by path for deterministic tree hashes
    tree.entries.sort_by(|a, b| a.name.cmp(&b.name));

    let tree_hash = repo.write_tree(&tree)?;
    
    // Get parent commit (if exists)
    let mut parents = Vec::new();
    if let Ok(head) = repo.get_head() {
        if let Ok(parent_hash) = repo.resolve_ref(&head) {
            // Only add parent if it's a valid hash (not empty)
            if !parent_hash.is_empty() && parent_hash != head {
                parents.push(parent_hash);
            }
        }
    }
    
    // Create commit object
    let author = get_author_name();
    let email = get_author_email();
    let commit = Commit::new(tree_hash, parents, author, email, message);
    
    let commit_hash = repo.write_commit(&commit)?;
    
    // Update the current branch reference
    let current_branch = repo.get_current_branch()
        .unwrap_or_else(|_| "main".to_string());
    
    let branch_ref_path = repo.refs_dir().join("heads").join(&current_branch);
    std::fs::write(&branch_ref_path, &commit_hash)?;
    
    println!("[{} {}] {}", current_branch, &commit_hash[..7], commit.message);
    
    Ok(())
}
