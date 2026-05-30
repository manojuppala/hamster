# Hamster VCS Performance Guide

## Overview

Hamster is designed to be **blazing fast** and capable of handling **large repositories** with thousands of files and deep commit histories. This document outlines the performance optimizations implemented and how to measure and improve performance.

## Performance Features

### 1. Object Caching

**Location**: `src/repository.rs`

The Repository struct maintains an in-memory cache of frequently accessed objects (commits, trees, blobs):

```rust
// Objects are cached on read/write
let object = repo.read_object(hash)?; // First read: disk I/O
let same_object = repo.read_object(hash)?; // Subsequent reads: cache hit
```

**Benefits**:
- Reduces disk I/O for frequently accessed commits and trees
- Particularly effective during `log`, `diff`, and `merge` operations
- Automatically populated during write operations

**Management**:
```rust
repo.clear_cache();  // Clear cache to free memory
let size = repo.cache_size();  // Monitor cache usage
```

### 2. Buffered I/O

**Location**: `src/repository.rs`, `src/utils.rs`

All file operations use buffered readers/writers:

- BufReader with 128KB buffer for reading objects and large files
- BufWriter for writing compressed objects
- Reduces system calls and improves throughput

**Large File Threshold**: 1MB (files larger than this use buffered reading)

### 3. Parallel Processing

**Location**: `src/commands/{status,add,checkout}.rs`

Uses Rayon for parallel processing of multiple files:

```rust
// Process files in parallel
files.par_iter()
    .map(|file| process_file(file))
    .collect()
```

**Parallelized Operations**:
- `ham status`: Hash all working directory files in parallel
- `ham add .`: Process and hash multiple files simultaneously
- `ham checkout`: Restore multiple files in parallel

**Performance Gain**: 2-4x faster on multi-core systems with many files

### 4. Ignore Pattern Caching

**Location**: `src/utils.rs`

The `.hamsterignore` file is read once and cached:

```rust
// Patterns loaded once and reused
static IGNORE_PATTERNS: OnceLock<Vec<String>> = OnceLock::new();
```

**Benefits**:
- Eliminates repeated file I/O during status/add operations
- Faster pattern matching with pre-compiled patterns
- Significant improvement for large repositories

### 5. Smart Content Deduplication

**Location**: `src/repository.rs`

Before writing objects, checks if they already exist:

```rust
if object_path.exists() {
    return Ok(hash);  // Skip write, return existing hash
}
```

**Benefits**:
- Reduces disk writes for identical content
- Saves storage space
- Faster operations when dealing with similar files

### 6. Optimized Tree Building

**Location**: `src/commands/commit.rs`

Trees are built with pre-allocated capacity and sorted entries:

```rust
tree.entries.reserve(index.entries.len());  // Pre-allocate
tree.entries.sort_by(|a, b| a.name.cmp(&b.name));  // Deterministic
```

**Benefits**:
- Reduces memory allocations
- Ensures consistent tree hashes
- Faster commit operations

## Benchmarking

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench -- hashing

# Generate HTML reports
cargo bench --bench hamster_benchmarks
```

Benchmark reports are saved to `target/criterion/`.

### Benchmark Suites

1. **Hashing**: Tests SHA-256 performance on various data sizes
2. **Compression**: Measures zlib compression speed
3. **File Operations**: Benchmarks file read/write performance
4. **Parallel vs Sequential**: Compares parallel and sequential processing

### Interpreting Results

```
hashing/1024            time:   [2.5 µs 2.6 µs 2.7 µs]
hashing/1024000         time:   [1.2 ms 1.3 ms 1.4 ms]
```

Lower times are better. Look for:
- Consistent performance across runs (small variance)
- Linear scaling with data size
- Parallel speedup > 2x on multi-core systems

## Performance Tips

### For Users

1. **Use `.hamsterignore`**: Exclude build artifacts, node_modules, etc.
2. **Commit frequently**: Smaller commits are faster to process
3. **Clean working directory**: Fewer untracked files = faster status
4. **Use specific paths**: `ham add src/` is faster than `ham add .`

### For Developers

1. **Profile before optimizing**: Use `cargo flamegraph` or `perf`
2. **Minimize allocations**: Use `&str` over `String`, reuse buffers
3. **Batch operations**: Group I/O operations when possible
4. **Use parallel iterators**: Leverage Rayon for independent operations
5. **Cache intelligently**: Store frequently accessed data
6. **Benchmark changes**: Always measure performance impact

## Performance Testing

### Create Large Test Repository

```bash
# Create test repository with many files
mkdir test-repo && cd test-repo
ham init

# Create 1000 test files
for i in {1..1000}; do
    echo "Content $i" > file_$i.txt
done

# Benchmark add operation
time ham add .

# Benchmark status operation
time ham status
```

### Expected Performance

On modern hardware (4+ cores, SSD):

| Operation | Small Repo (10 files) | Medium Repo (100 files) | Large Repo (1000 files) |
|-----------|----------------------|------------------------|------------------------|
| `ham init` | < 10ms | < 10ms | < 10ms |
| `ham add .` | < 50ms | < 200ms | < 1s |
| `ham commit` | < 30ms | < 100ms | < 500ms |
| `ham status` | < 50ms | < 200ms | < 1s |
| `ham log -n 10` | < 20ms | < 30ms | < 50ms |

## Future Optimizations

Planned performance improvements:

1. **Packfiles**: Bundle objects for efficient storage and transfer
2. **Delta Compression**: Store diffs instead of full content
3. **Memory-mapped I/O**: For very large files (> 100MB)
4. **Incremental Hashing**: Stream processing for huge files
5. **Index Optimization**: Binary format instead of JSON
6. **Sparse Checkout**: Only materialize needed files
7. **Shallow Clone**: Clone with limited history depth

## Monitoring Performance

### Enable Performance Logging

Set environment variable for detailed timing:

```bash
export HAM_PROFILE=1
ham status  # Will print timing information
```

### Memory Profiling

```bash
# Use valgrind for memory profiling
valgrind --tool=massif target/release/ham status

# Analyze results
ms_print massif.out.<pid>
```

### CPU Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Profile a command
sudo cargo flamegraph --bin ham -- status

# Open flamegraph.svg in browser
```

## Contributing Performance Improvements

When submitting performance-related PRs:

1. Include benchmark results (before/after)
2. Profile the code to identify bottlenecks
3. Test on large repositories (1000+ files)
4. Ensure no regressions in other operations
5. Update this document with new optimizations
