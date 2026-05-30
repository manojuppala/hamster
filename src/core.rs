use sha2::{Sha256, Digest};

/// Compute SHA-256 hash of data
pub fn hash_object(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Get the current timestamp in Unix format
pub fn current_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

/// Get the current user's name from git config or environment
pub fn get_author_name() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Get the current user's email
pub fn get_author_email() -> String {
    format!("{}@localhost", get_author_name())
}
