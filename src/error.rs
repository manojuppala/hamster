use thiserror::Error;

#[derive(Error, Debug)]
pub enum HamsterError {
    #[error("Not a hamster repository (or any of the parent directories): .hamster")]
    NotARepository,
    
    #[error("Repository already exists")]
    RepositoryExists,
    
    #[error("Object not found: {0}")]
    ObjectNotFound(String),
    
    #[error("Invalid object type: {0}")]
    InvalidObjectType(String),
    
    #[error("Invalid hash: {0}")]
    InvalidHash(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Branch not found: {0}")]
    BranchNotFound(String),
    
    #[error("Branch already exists: {0}")]
    BranchExists(String),
    
    #[error("Cannot delete current branch")]
    CannotDeleteCurrentBranch,
    
    #[error("Merge conflict in {0}")]
    MergeConflict(String),
    
    #[error("Nothing to commit, working tree clean")]
    NothingToCommit,
    
    #[error("No commits yet")]
    NoCommits,
    
    #[error("Remote not found: {0}")]
    RemoteNotFound(String),
    
    #[error("Remote already exists: {0}")]
    RemoteExists(String),
    
    #[error("Invalid reference: {0}")]
    InvalidReference(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, HamsterError>;
