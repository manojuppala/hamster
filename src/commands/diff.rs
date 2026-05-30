use anyhow::Result;
use similar::{ChangeTag, TextDiff};
use crate::error::HamsterError;
use crate::index::Index;
use crate::repository::Repository;
use crate::utils::{find_repo_root, get_relative_path, read_file_bytes};
use std::collections::HashMap;

pub fn execute(commit: Option<String>) -> Result<()> {
    let repo_root = find_repo_root()?;
    let repo = Repository::new(repo_root.clone());
    let index = Index::load()?;
    
    if let Some(commit_hash) = commit {
        // Diff between commit and working directory
        diff_commit_to_working(&repo, &commit_hash)?;
    } else {
        // Diff between staged and working directory
        diff_staged_to_working(&repo, &index)?;
    }
    
    Ok(())
}

fn diff_staged_to_working(repo: &Repository, index: &Index) -> Result<()> {
    if index.entries.is_empty() {
        println!("No files staged");
        return Ok(());
    }
    
    for (path, entry) in &index.entries {
        let file_path = repo.root.join(path);
        
        if !file_path.exists() {
            println!("\x1b[31mdiff --ham a/{} b/{}\x1b[0m", path, path);
            println!("\x1b[31m--- a/{}\x1b[0m", path);
            println!("\x1b[32m+++ /dev/null\x1b[0m");
            println!("File deleted");
            continue;
        }
        
        let staged_content = repo.read_object(&entry.hash)?;
        let working_content = read_file_bytes(&file_path)?;
        
        if staged_content.data != working_content {
            println!("\x1b[31mdiff --ham a/{} b/{}\x1b[0m", path, path);
            display_diff(&staged_content.data, &working_content, path)?;
        }
    }
    
    Ok(())
}

fn diff_commit_to_working(repo: &Repository, commit_hash: &str) -> Result<()> {
    let commit = repo.read_commit(commit_hash)?;
    let tree = repo.read_tree(&commit.tree)?;
    
    for entry in &tree.entries {
        let file_path = repo.root.join(&entry.name);
        
        let committed_content = repo.read_object(&entry.hash)?;
        
        if !file_path.exists() {
            println!("\x1b[31mdiff --ham a/{} b/{}\x1b[0m", entry.name, entry.name);
            println!("File deleted");
            continue;
        }
        
        let working_content = read_file_bytes(&file_path)?;
        
        if committed_content.data != working_content {
            println!("\x1b[31mdiff --ham a/{} b/{}\x1b[0m", entry.name, entry.name);
            display_diff(&committed_content.data, &working_content, &entry.name)?;
        }
    }
    
    Ok(())
}

fn display_diff(old: &[u8], new: &[u8], path: &str) -> Result<()> {
    let old_str = String::from_utf8_lossy(old);
    let new_str = String::from_utf8_lossy(new);
    
    let diff = TextDiff::from_lines(&old_str, &new_str);
    
    println!("\x1b[31m--- a/{}\x1b[0m", path);
    println!("\x1b[32m+++ b/{}\x1b[0m", path);
    
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "\x1b[31m-",
            ChangeTag::Insert => "\x1b[32m+",
            ChangeTag::Equal => " ",
        };
        print!("{}{}\x1b[0m", sign, change);
    }
    
    println!();
    
    Ok(())
}
