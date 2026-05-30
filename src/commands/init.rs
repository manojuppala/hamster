use std::fs;
use anyhow::Result;
use crate::error::HamsterError;

pub fn execute() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let hamster_dir = current_dir.join(".hamster");
    
    if hamster_dir.exists() {
        return Err(HamsterError::RepositoryExists.into());
    }
    
    // Create .hamster directory structure
    fs::create_dir(&hamster_dir)?;
    fs::create_dir(hamster_dir.join("objects"))?;
    fs::create_dir(hamster_dir.join("refs"))?;
    fs::create_dir(hamster_dir.join("refs").join("heads"))?;
    fs::create_dir(hamster_dir.join("refs").join("tags"))?;
    fs::create_dir(hamster_dir.join("refs").join("remotes"))?;
    
    // Create HEAD file pointing to main branch
    fs::write(hamster_dir.join("HEAD"), "ref: refs/heads/main\n")?;
    
    // Create empty config file
    fs::write(hamster_dir.join("config"), "{}\n")?;
    
    // Create empty index file
    fs::write(hamster_dir.join("index"), "{\"entries\":{}}\n")?;
    
    println!("Initialized empty Hamster repository in {}", hamster_dir.display());
    
    Ok(())
}
