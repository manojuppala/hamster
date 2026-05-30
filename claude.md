# Hamster VCS - Claude Project Documentation

## Project Overview

**Hamster** is a Git-like version control system written in Rust, designed to be **blazing fast** and capable of **handling large repositories easily**. While maintaining simplicity and clarity in its implementation, Hamster focuses on performance optimization and scalability for real-world use cases. It implements core Git features using a content-addressable object storage system with SHA-256 hashing.

**Binary Name**: `ham` (not `hamster`)

## Performance Goals

Hamster is built with performance as a primary objective:

- **Blazing Fast Operations**: All commands should execute quickly, even on large codebases
- **Large Repository Support**: Designed to handle repositories with thousands of files and deep commit histories
- **Efficient Storage**: Uses zlib compression and content deduplication via hashing
- **Optimized I/O**: Minimizes disk operations and memory usage
- **Scalable Design**: Architecture supports future optimizations like packfiles and delta compression

## Core Architecture

### Object Storage System

Hamster uses three types of objects, all stored in `.hamster/objects/` with zlib compression:

1. **Blob Objects**: Store file content
2. **Tree Objects**: Represent directory structure (JSON serialized)
3. **Commit Objects**: Snapshots with metadata (JSON serialized)

Objects are stored by their SHA-256 hash in a two-level directory structure:

```
.hamster/objects/ab/cdef123... (first 2 chars / rest of hash)
```

### Repository Structure

```
.hamster/
├── objects/          # Compressed objects (blobs, trees, commits)
├── refs/
│   ├── heads/        # Branch pointers
│   ├── tags/         # Tag references
│   └── remotes/      # Remote tracking branches
├── HEAD              # Current branch pointer
├── index             # Staging area (JSON)
└── config            # Repository configuration (JSON)
```

### Key Components

- **Repository** (`src/repository.rs`): Core object database operations (read/write objects with zlib compression)
- **Index** (`src/index.rs`): Staging area implemented as JSON-serialized BTreeMap
- **Objects** (`src/objects.rs`): Data structures for Blob, Tree, Commit (all JSON serialized)
- **Commands** (`src/commands/*.rs`): Individual command implementations
- **Core** (`src/core.rs`): Hashing, timestamps, author info utilities
- **Error** (`src/error.rs`): Custom error types using thiserror
- **Config** (`src/config.rs`): Repository configuration management
- **Remote** (`src/remote.rs`): Remote repository operations
- **Utils** (`src/utils.rs`): Helper functions (path resolution, ignore patterns)

## Technology Stack

```toml
clap = "4.5"           # CLI argument parsing (derive feature)
sha2 = "0.10"          # SHA-256 hashing
hex = "0.4"            # Hex encoding
flate2 = "1.0"         # zlib compression
serde/serde_json       # JSON serialization
chrono = "0.4"         # Timestamps
walkdir = "2.4"        # Directory traversal
ignore = "0.4"         # .hamsterignore support
similar = "2.3"        # Diff generation
```

## Available Commands

| Command                      | Implementation         | Description                              |
| ---------------------------- | ---------------------- | ---------------------------------------- |
| `ham init`                   | `commands/init.rs`     | Initialize repository                    |
| `ham add <files>`            | `commands/add.rs`      | Stage files (supports `.` for all files) |
| `ham commit -m "msg"`        | `commands/commit.rs`   | Create commit from staged files          |
| `ham status`                 | `commands/status.rs`   | Show working tree status                 |
| `ham log [-n N]`             | `commands/log.rs`      | Show commit history                      |
| `ham diff [commit]`          | `commands/diff.rs`     | Show changes                             |
| `ham branch [name] [-d]`     | `commands/branch.rs`   | List/create/delete branches              |
| `ham checkout <target>`      | `commands/checkout.rs` | Switch branches                          |
| `ham merge <branch>`         | `commands/merge.rs`    | Merge branches                           |
| `ham clone <url> [dir]`      | `commands/clone.rs`    | Clone repository                         |
| `ham remote add/remove/list` | `commands/remote.rs`   | Manage remotes                           |
| `ham push [remote] [branch]` | `commands/push.rs`     | Push to remote                           |
| `ham fetch [remote]`         | `commands/fetch.rs`    | Fetch from remote                        |
| `ham pull [remote] [branch]` | `commands/pull.rs`     | Fetch + merge                            |

## Data Flow

### Adding Files

```
Working Directory → hash content → write blob object → update index → save index
```

### Committing

```
Index entries → build tree object → create commit object → update branch ref
```

### Checking Out

```
Resolve branch ref → read commit → read tree → restore files to working directory
```

## Key Implementation Details

### Hashing

```rust
hash = SHA256(type + " " + size + "\0" + content)
```

### Object Serialization

- **Blobs**: Raw bytes
- **Trees**: JSON with array of {mode, name, hash} entries
- **Commits**: JSON with tree, parents[], author, email, timestamp, message

### Index Format (JSON)

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

### File Modes

- `100644`: Regular file (read-only)
- `100755`: Executable file

### Ignore Patterns

`.hamsterignore` file in repository root (similar to `.gitignore`)

## Current Implementation Status

**Performance Features:**

- SHA-256 hashing for content addressing (more secure than SHA-1)
- zlib compression for efficient storage
- Content deduplication (identical content stored once)
- Efficient directory traversal with `walkdir`
- Smart ignore patterns to skip unnecessary files

**Planned Optimizations:**

- Packfiles for bundled object storage and transfer
- Delta compression to store diffs instead of full content
- Parallel object processing for large operations
- Memory-mapped file I/O for large files
- Incremental status and diff algorithms
- Object caching for frequently accessed commits/trees

**Current Limitations:**

1. **JSON serialization**: Uses JSON for trees/commits (human-readable but not most compact)
2. **Local remotes only**: File system paths, no HTTP/SSH support yet
3. **No packfiles**: Each object stored individually (will be optimized)
4. **Simplified merge**: Basic merge algorithm (will be enhanced)
5. **No delta compression**: Full content storage (planned optimization)

## Building & Testing

```bash
# Build
cargo build --release

# Binary location
target/release/ham

# Install
cargo install --path .

# Run tests
cargo test
```

## Common Development Tasks

### Adding a New Command

1. Create `src/commands/yourcommand.rs`
2. Add public `execute()` function
3. Add module to `src/commands/mod.rs`
4. Add enum variant to `Commands` in `src/main.rs`
5. Add match arm in `main()` function
6. **Performance**: Consider impact on large repositories; use iterators, avoid unnecessary allocations

### Modifying Object Types

- Update structs in `src/objects.rs`
- Ensure serde serialization compatibility
- Update serialize/deserialize methods if needed
- **Performance**: Consider serialization overhead; benchmark if making structural changes

### Error Handling

- Add new error variants to `HamsterError` in `src/error.rs`
- Use `?` operator for Result propagation
- Return `Result<()>` from command execute functions

### Performance Optimization Guidelines

- **Profile first**: Use `cargo flamegraph` or similar tools to identify bottlenecks
- **Minimize I/O**: Batch file operations, use buffered readers/writers
- **Avoid allocations**: Reuse buffers, use `&str` over `String` where possible
- **Parallelize**: Consider `rayon` for parallel processing of large object sets
- **Cache intelligently**: Store frequently accessed data (current commit, tree objects)
- **Benchmark**: Add benchmarks for critical paths using `criterion`

## Repository Detection

The `find_repo_root()` utility walks up the directory tree looking for `.hamster/` directory.

## Author Information

Retrieved from environment variables:

- `USER` or `USERNAME` for name
- Email: `{username}@localhost`

## File Organization

- Entry point: `src/main.rs`
- Core logic: `src/core.rs`, `src/repository.rs`, `src/objects.rs`, `src/index.rs`
- Commands: `src/commands/*.rs`
- Support: `src/error.rs`, `src/config.rs`, `src/remote.rs`, `src/utils.rs`

## Additional Documentation

- `README.md`: User-facing documentation
- `ARCHITECTURE.md`: Detailed architecture explanation
- `EXAMPLES.md`: Usage examples
- `QUICK_REFERENCE.md`: Command reference
- `BUILD.md`: Build instructions
