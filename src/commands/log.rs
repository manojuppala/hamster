use anyhow::Result;
use chrono::{DateTime, Utc, TimeZone};
use crate::error::HamsterError;
use crate::repository::Repository;
use crate::utils::find_repo_root;

pub fn execute(limit: Option<usize>) -> Result<()> {
    let repo_root = find_repo_root()?;
    let repo = Repository::new(repo_root);
    
    // Get HEAD commit
    let head = repo.get_head()?;
    let mut current_hash = repo.resolve_ref(&head)?;
    
    if current_hash.is_empty() || current_hash == head {
        return Err(HamsterError::NoCommits.into());
    }
    
    let max_commits = limit.unwrap_or(usize::MAX);
    let mut count = 0;
    
    loop {
        if count >= max_commits {
            break;
        }
        
        let commit = match repo.read_commit(&current_hash) {
            Ok(c) => c,
            Err(_) => break,
        };
        
        // Display commit
        println!("\x1b[33mcommit {}\x1b[0m", current_hash);
        println!("Author: {} <{}>", commit.author, commit.email);
        
        let dt: DateTime<Utc> = Utc.timestamp_opt(commit.timestamp, 0).unwrap();
        println!("Date:   {}", dt.format("%a %b %d %H:%M:%S %Y %z"));
        println!();
        println!("    {}", commit.message);
        println!();
        
        // Move to parent commit
        if commit.parents.is_empty() {
            break;
        }
        
        current_hash = commit.parents[0].clone();
        count += 1;
    }
    
    Ok(())
}
