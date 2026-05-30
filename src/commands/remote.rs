use anyhow::Result;
use crate::error::HamsterError;
use crate::config::Config;
use crate::RemoteCommands;

pub fn execute(command: Option<RemoteCommands>) -> Result<()> {
    match command {
        Some(RemoteCommands::Add { name, url }) => add_remote(name, url),
        Some(RemoteCommands::Remove { name }) => remove_remote(name),
        Some(RemoteCommands::List { verbose }) => list_remotes(verbose),
        None => list_remotes(false),
    }
}

fn add_remote(name: String, url: String) -> Result<()> {
    let mut config = Config::load()?;
    
    if config.get_remote(&name).is_some() {
        return Err(HamsterError::RemoteExists(name).into());
    }
    
    config.add_remote(name.clone(), url.clone());
    config.save()?;
    
    println!("Added remote '{}' -> {}", name, url);
    
    Ok(())
}

fn remove_remote(name: String) -> Result<()> {
    let mut config = Config::load()?;
    
    if config.get_remote(&name).is_none() {
        return Err(HamsterError::RemoteNotFound(name).into());
    }
    
    config.remove_remote(&name);
    config.save()?;
    
    println!("Removed remote '{}'", name);
    
    Ok(())
}

fn list_remotes(verbose: bool) -> Result<()> {
    let config = Config::load()?;
    
    if config.remotes.is_empty() {
        println!("No remotes configured");
        return Ok(());
    }
    
    for (name, remote) in &config.remotes {
        if verbose {
            println!("{}\t{} (fetch)", name, remote.url);
            println!("{}\t{} (push)", name, remote.url);
        } else {
            println!("{}", name);
        }
    }
    
    Ok(())
}
