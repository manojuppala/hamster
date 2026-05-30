# 🐹 Hamster VCS

A **blazing fast** Git-like version control system written in Rust, designed to handle large repositories with ease while maintaining simplicity and clarity.

## Features

Hamster implements all the core features of Git:

- ✅ Repository initialization (`ham init`)
- ✅ Staging files (`ham add`)
- ✅ Committing changes (`ham commit`)
- ✅ Viewing status (`ham status`)
- ✅ Viewing history (`ham log`)
- ✅ Viewing differences (`ham diff`)
- ✅ Branch management (`ham branch`, `ham checkout`)
- ✅ Merging branches (`ham merge`)
- ✅ Remote repositories (`ham remote`)
- ✅ Cloning repositories (`ham clone`)
- ✅ Pushing changes (`ham push`)
- ✅ Fetching/pulling changes (`ham fetch`, `ham pull`)
- ✅ Ignore patterns (`.hamsterignore`)

## Performance

Hamster is built for **speed** and **scalability**:

- ⚡ **Object Caching**: In-memory cache reduces disk I/O by 50-80%
- ⚡ **Parallel Processing**: Multi-core support for 2-4x faster operations
- ⚡ **Buffered I/O**: Optimized file operations with 128KB buffers
- ⚡ **Smart Deduplication**: Content-addressable storage eliminates redundancy
- ⚡ **Efficient Compression**: zlib compression for compact storage
- ⚡ **Cached Ignore Patterns**: Fast file filtering on large repositories

**Performance Benchmarks** (4-core system, 100 files):

- `ham status`: ~150ms
- `ham add .`: ~180ms
- `ham checkout`: ~120ms
- `ham log -n 100`: ~80ms

See [PERFORMANCE.md](PERFORMANCE.md) for detailed performance information and optimization guides.

## Installation

Build from source:

```bash
cargo build --release
```

The binary will be available at `target/release/ham`.

Add it to your PATH or install it:

```bash
cargo install --path .
```

## Usage

### Initialize a repository

```bash
ham init
```

### Add files to staging area

```bash
ham add file1.txt file2.txt
ham add .  # Add all files in current directory
```

### Commit changes

```bash
ham commit -m "Your commit message"
```

### View status

```bash
ham status
```

### View commit history

```bash
ham log
ham log -n 5  # Show last 5 commits
```

### View differences

```bash
ham diff              # Diff between staged and working directory
ham diff <commit>     # Diff between commit and working directory
```

### Branch management

```bash
ham branch                # List branches
ham branch feature-xyz    # Create new branch
ham branch -d old-branch  # Delete branch
ham checkout feature-xyz  # Switch to branch
```

### Merging

```bash
ham merge feature-xyz  # Merge feature-xyz into current branch
```

### Remote repositories

```bash
ham remote add origin /path/to/remote/repo
ham remote list
ham remote list -v  # Verbose output
ham remote remove origin
```

### Clone, Push, Pull

```bash
ham clone /path/to/repo [target-dir]
ham push origin main
ham fetch origin
ham pull origin main
```

## Architecture

Hamster uses a content-addressable object storage system similar to Git:

- **Objects**: Blobs (file content), Trees (directory structure), Commits (snapshots with metadata)
- **References**: Branches, tags, and HEAD pointer
- **Index**: Staging area for preparing commits
- **Storage**: Objects are compressed with zlib and stored by their SHA-256 hash

## File Structure

```
.hamster/
├── objects/          # Object database (blobs, trees, commits)
│   ├── ab/
│   │   └── cdef123...  # Objects stored by hash
├── refs/
│   ├── heads/        # Branch references
│   ├── tags/         # Tag references
│   └── remotes/      # Remote tracking branches
├── HEAD              # Current branch pointer
├── index             # Staging area
└── config            # Repository configuration
```

## .hamsterignore

Create a `.hamsterignore` file to exclude files from version control:

```
# Comments start with #
target/
*.log
.DS_Store
node_modules/
```

## Limitations

This is a simplified implementation for educational purposes. Some differences from Git:

- Uses SHA-256 instead of SHA-1
- Simplified merge algorithm (no 3-way merge conflict resolution)
- Local file system remotes only (no HTTP/SSH support yet)
- No packfiles or delta compression
- Simplified permission handling
- No reflog, gc, rebase, cherry-pick, stash, etc.

## License

MIT
