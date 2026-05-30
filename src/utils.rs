use std::path::{Path, PathBuf};
use std::io::{Read, BufRead, BufReader};
use std::sync::OnceLock;
use crate::error::{HamsterError, Result};

/// Threshold for using buffered reading (1MB)
const LARGE_FILE_THRESHOLD: u64 = 1024 * 1024;

/// Cached ignore patterns for performance
static IGNORE_PATTERNS: OnceLock<Vec<String>> = OnceLock::new();

/// Find the root of the hamster repository by looking for .hamster directory
pub fn find_repo_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;
    
    loop {
        let hamster_dir = current.join(".hamster");
        if hamster_dir.exists() && hamster_dir.is_dir() {
            return Ok(current);
        }
        
        if !current.pop() {
            return Err(HamsterError::NotARepository);
        }
    }
}

/// Get the .hamster directory path
pub fn get_hamster_dir() -> Result<PathBuf> {
    Ok(find_repo_root()?.join(".hamster"))
}

/// Get relative path from repository root
pub fn get_relative_path(path: &Path) -> Result<PathBuf> {
    let repo_root = find_repo_root()?;
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    
    absolute
        .strip_prefix(&repo_root)
        .map(|p| p.to_path_buf())
        .map_err(|_| HamsterError::FileNotFound(path.display().to_string()))
}

/// Read file content as bytes (optimized for large files)
pub fn read_file_bytes(path: &Path) -> Result<Vec<u8>> {
    let metadata = std::fs::metadata(path)
        .map_err(|_| HamsterError::FileNotFound(path.display().to_string()))?;

    let file_size = metadata.len();

    if file_size > LARGE_FILE_THRESHOLD {
        // For large files, use buffered reading
        let file = std::fs::File::open(path)
            .map_err(|_| HamsterError::FileNotFound(path.display().to_string()))?;
        let mut reader = BufReader::with_capacity(128 * 1024, file); // 128KB buffer
        let mut content = Vec::with_capacity(file_size as usize);
        reader.read_to_end(&mut content)
            .map_err(|_| HamsterError::FileNotFound(path.display().to_string()))?;
        Ok(content)
    } else {
        // For small files, direct read is fine
        std::fs::read(path)
            .map_err(|_| HamsterError::FileNotFound(path.display().to_string()))
    }
}

/// Read file content incrementally for hashing (memory efficient for large files)
pub fn read_file_for_hashing(path: &Path) -> Result<impl Iterator<Item = Result<Vec<u8>>>> {
    let file = std::fs::File::open(path)
        .map_err(|_| HamsterError::FileNotFound(path.display().to_string()))?;

    let reader = BufReader::with_capacity(128 * 1024, file);

    Ok(reader.split(b'\n').map(|line| {
        line.map_err(|_| HamsterError::FileNotFound(path.display().to_string()))
    }))
}

/// Load and cache ignore patterns for performance
fn load_ignore_patterns() -> Vec<String> {
    if let Ok(repo_root) = find_repo_root() {
        let ignore_file = repo_root.join(".hamsterignore");
        if ignore_file.exists() {
            if let Ok(content) = std::fs::read_to_string(ignore_file) {
                return content
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty() && !line.starts_with('#'))
                    .map(String::from)
                    .collect();
            }
        }
    }
    Vec::new()
}

/// Check if path should be ignored based on .hamsterignore (optimized with caching)
pub fn should_ignore(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    // Always ignore .hamster directory
    if path_str.contains(".hamster") {
        return true;
    }

    // Use cached patterns for better performance on large repositories
    let patterns = IGNORE_PATTERNS.get_or_init(load_ignore_patterns);

    // Check against ignore patterns
    for pattern in patterns {
        // Simple pattern matching (can be enhanced with glob patterns)
        if pattern.ends_with('/') {
            // Directory pattern
            if path_str.contains(pattern) {
                return true;
            }
        } else if pattern.starts_with('*') {
            // Extension pattern like *.log
            if path_str.ends_with(pattern.trim_start_matches('*')) {
                return true;
            }
        } else {
            // Simple substring match
            if path_str.contains(pattern.as_str()) {
                return true;
            }
        }
    }

    false
}

/// Clear cached ignore patterns (useful when .hamsterignore is modified)
pub fn clear_ignore_cache() {
    // Note: OnceLock doesn't support clearing, so this is a no-op
    // In production, we'd use a different caching mechanism
}
