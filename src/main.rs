mod commands;
mod core;
mod error;
mod repository;
mod objects;
mod index;
mod remote;
mod config;
mod utils;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "ham")]
#[command(about = "Hamster - A Git-like version control system", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init,
    
    /// Add files to the staging area
    Add {
        /// Files to add
        files: Vec<String>,
    },
    
    /// Record changes to the repository
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,
    },
    
    /// Show the working tree status
    Status,
    
    /// Show commit logs
    Log {
        /// Number of commits to show
        #[arg(short, long)]
        n: Option<usize>,
    },
    
    /// Show changes between commits, commit and working tree, etc
    Diff {
        /// Commit to compare against
        commit: Option<String>,
    },
    
    /// List, create, or delete branches
    Branch {
        /// Branch name
        name: Option<String>,
        
        /// Delete branch
        #[arg(short, long)]
        delete: bool,
    },
    
    /// Switch branches or restore working tree files
    Checkout {
        /// Branch or commit to checkout
        target: String,
    },
    
    /// Join two or more development histories together
    Merge {
        /// Branch to merge into current branch
        branch: String,
    },
    
    /// Clone a repository into a new directory
    Clone {
        /// Repository URL or path
        url: String,
        
        /// Directory to clone into
        directory: Option<String>,
    },
    
    /// Manage remote repositories
    Remote {
        #[command(subcommand)]
        command: Option<RemoteCommands>,
    },
    
    /// Fetch from and integrate with another repository
    Pull {
        /// Remote name
        remote: Option<String>,
        
        /// Branch name
        branch: Option<String>,
    },
    
    /// Download objects and refs from another repository
    Fetch {
        /// Remote name
        remote: Option<String>,
    },
    
    /// Update remote refs along with associated objects
    Push {
        /// Remote name
        remote: Option<String>,
        
        /// Branch name
        branch: Option<String>,
    },
}

#[derive(Subcommand)]
enum RemoteCommands {
    /// Add a remote
    Add {
        /// Remote name
        name: String,
        
        /// Remote URL
        url: String,
    },
    
    /// Remove a remote
    Remove {
        /// Remote name
        name: String,
    },
    
    /// List remotes
    List {
        /// Show URLs
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Init => commands::init::execute()?,
        Commands::Add { files } => commands::add::execute(files)?,
        Commands::Commit { message } => commands::commit::execute(message)?,
        Commands::Status => commands::status::execute()?,
        Commands::Log { n } => commands::log::execute(n)?,
        Commands::Diff { commit } => commands::diff::execute(commit)?,
        Commands::Branch { name, delete } => commands::branch::execute(name, delete)?,
        Commands::Checkout { target } => commands::checkout::execute(target)?,
        Commands::Merge { branch } => commands::merge::execute(branch)?,
        Commands::Clone { url, directory } => commands::clone::execute(url, directory)?,
        Commands::Remote { command } => commands::remote::execute(command)?,
        Commands::Pull { remote, branch } => commands::pull::execute(remote, branch)?,
        Commands::Fetch { remote } => commands::fetch::execute(remote)?,
        Commands::Push { remote, branch } => commands::push::execute(remote, branch)?,
    }
    
    Ok(())
}
