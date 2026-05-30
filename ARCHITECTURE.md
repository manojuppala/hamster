# Hamster VCS Architecture

## Overview

Hamster is a Git-like version control system that uses a content-addressable object database to store file snapshots, directory trees, and commit metadata.

## Core Components

### 1. Object Storage (`src/repository.rs`, `src/objects.rs`)

The heart of Hamster is its object database, which stores three types of objects:

#### Blob Objects
- Store file content
- Identified by SHA-256 hash of their content
- Compressed with zlib before storage

#### Tree Objects
- Represent directory structure
- Contain entries pointing to blobs (files) or other trees (subdirectories)
- Each entry has: mode, name, and hash

#### Commit Objects
- Represent snapshots of the project at a point in time
- Contain:
  - Reference to a tree object (the project state)
  - Parent commit(s) for history
  - Author information
  - Timestamp
  - Commit message

### 2. References (`refs/`)

References are pointers to commits:

- **Branches** (`refs/heads/`): Pointers that move forward with new commits
- **Remote tracking** (`refs/remotes/`): Track remote branch positions
- **HEAD**: Special pointer indicating current branch or commit

### 3. Index (`src/index.rs`)

The staging area that sits between the working directory and the repository:

- Tracks which files will be included in the next commit
- Stores file paths, their hashes, and permissions
- Persisted as JSON in `.hamster/index`

### 4. Configuration (`src/config.rs`)

Repository configuration including:

- Remote repository URLs
- User information
- Repository-specific settings

## Data Flow

### Adding Files

```
Working Directory → Index (Staging) → Object Database
```

1. User runs `ham add file.txt`
2. File content is read and hashed
3. Blob object is created and stored
4. Index is updated with file path and hash

### Creating Commits

```
Index → Tree Object → Commit Object → Branch Reference
```

1. User runs `ham commit -m "message"`
2. Tree object is built from index entries
3. Commit object is created referencing the tree
4. Current branch reference is updated to point to new commit

### Checking Out

```
Commit Object → Tree Object → Working Directory
```

1. User runs `ham checkout branch-name`
2. Branch reference is resolved to commit hash
3. Commit's tree is read
4. Files are written to working directory

## Storage Format

### Object Storage

Objects are stored in `.hamster/objects/` with a two-level directory structure:

```
.hamster/objects/
├── ab/
│   └── cdef1234567890...  (rest of the hash)
├── 12/
│   └── 3456789abcdef...
```

Each object file contains:
1. Type header (blob, tree, or commit)
2. Space
3. Size
4. Null byte
5. Content (compressed with zlib)

### Index Format

The index is stored as JSON:

```json
{
  "entries": {
    "path/to/file.txt": {
      "path": "path/to/file.txt",
      "hash": "abc123...",
      "mode": "100644"
    }
  }
}
```

## Command Implementation

Each command is implemented as a module in `src/commands/`:

- **init**: Creates `.hamster` directory structure
- **add**: Hashes files and updates index
- **commit**: Creates tree and commit objects
- **status**: Compares working directory, index, and HEAD
- **log**: Traverses commit history
- **diff**: Compares file content at different stages
- **branch**: Manages branch references
- **checkout**: Updates HEAD and working directory
- **merge**: Combines branch histories
- **clone/push/pull**: Synchronizes with remote repositories

## Hashing

Hamster uses SHA-256 for content addressing:

```rust
hash = SHA256(type + " " + size + "\0" + content)
```

This ensures:
- Content integrity
- Deduplication (identical content = same hash)
- Efficient lookups

## Compression

Objects are compressed with zlib (Deflate algorithm) to save disk space:

```rust
compressed = zlib_compress(object_data)
```

## Error Handling

Custom error types in `src/error.rs` provide clear error messages:

- `NotARepository`: When `.hamster` directory not found
- `ObjectNotFound`: When referenced object doesn't exist
- `BranchNotFound`: When branch doesn't exist
- etc.

## Future Enhancements

Potential improvements for Hamster:

1. **Packfiles**: Bundle multiple objects for efficient storage/transfer
2. **Delta compression**: Store diffs instead of full content
3. **Network protocols**: HTTP/SSH support for remote operations
4. **3-way merge**: Proper conflict detection and resolution
5. **Reflog**: Track reference updates
6. **Garbage collection**: Clean up unreachable objects
7. **Hooks**: Pre-commit, post-commit scripts
8. **Submodules**: Nested repository support

## Comparison with Git

### Similarities
- Content-addressable storage
- Object types (blob, tree, commit)
- Index/staging area
- Branch model
- Remote repositories

### Differences
- SHA-256 vs SHA-1 hashing
- JSON serialization vs Git's custom format
- No packfiles or delta compression
- Simplified merge algorithm
- Local file system remotes only
- No advanced features (rebase, cherry-pick, etc.)
