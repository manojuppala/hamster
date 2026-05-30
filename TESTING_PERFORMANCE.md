# Testing Hamster Performance

This guide helps you test and verify Hamster's performance optimizations.

## Quick Performance Test

### 1. Build Release Version

```bash
cargo build --release
```

**Important**: Always use release builds for performance testing. Debug builds are 10-100x slower.

### 2. Create Test Repository

```bash
# Create a test directory
mkdir hamster-perf-test
cd hamster-perf-test

# Initialize repository
../target/release/ham init

# Create test script to generate files
cat > generate_files.sh << 'EOF'
#!/bin/bash
for i in {1..500}; do
    echo "This is test file number $i with some content" > file_$i.txt
done
EOF

chmod +x generate_files.sh
./generate_files.sh
```

### 3. Test Performance

```bash
# Test status command
time ../target/release/ham status

# Test add command
time ../target/release/ham add .

# Test commit command  
time ../target/release/ham commit -m "Initial commit with 500 files"

# Test status again (should be faster due to caching)
time ../target/release/ham status

# Test log command
time ../target/release/ham log -n 10
```

### 4. Test Parallel Performance

```bash
# Create more files for parallel processing test
for i in {501..1000}; do
    echo "Additional test file $i" > file_$i.txt
done

# Add files - should show parallel speedup
time ../target/release/ham add .
```

## Benchmark Suite

### Run Full Benchmarks

```bash
# Run all benchmarks
cargo bench

# View HTML reports
open target/criterion/report/index.html
```

### Run Specific Benchmarks

```bash
# Benchmark hashing only
cargo bench -- hashing

# Benchmark compression only
cargo bench -- compression

# Benchmark parallel vs sequential
cargo bench -- parallel_comparison
```

### Interpret Results

Example output:
```
hashing/1024            time:   [2.5 µs 2.6 µs 2.7 µs]
                        change: [-5.2% -3.1% -1.0%]
```

- **time**: Current performance (2.6 µs average)
- **change**: Performance difference from last run (-3.1% = 3.1% faster)
- Lower times are better

## Performance Comparison

### Before vs After Optimizations

Create two test scenarios to compare:

```bash
# Test with parallel processing (optimized)
time ham add .

# To simulate non-parallel (for comparison), you'd need to
# comment out rayon code and rebuild, but you can see the
# difference in the benchmark suite's parallel_comparison test
```

### Cache Performance Test

```bash
# First run (cold cache)
time ham log -n 50

# Second run (warm cache - should be much faster)
time ham log -n 50

# Third run (still warm cache)
time ham log -n 50
```

## Large Repository Test

### Create Large Test Repository

```bash
#!/bin/bash
mkdir large-repo-test
cd large-repo-test
ham init

# Create directory structure
for dir in {1..10}; do
    mkdir -p dir_$dir
    for file in {1..100}; do
        echo "Content for dir $dir file $file" > dir_$dir/file_$file.txt
    done
done

echo "Created 1000 files in 10 directories"

# Test performance
echo "Testing ham status..."
time ham status

echo "Testing ham add..."
time ham add .

echo "Testing ham commit..."
time ham commit -m "Large commit with 1000 files"

echo "Testing ham log..."
time ham log -n 20
```

Expected results on modern hardware (4 cores, SSD):
- `ham status`: < 1 second
- `ham add .`: < 2 seconds
- `ham commit`: < 1 second
- `ham log -n 20`: < 100ms

## Profiling

### CPU Profiling with Flamegraph

```bash
# Install flamegraph
cargo install flamegraph

# Profile the status command
cd hamster-perf-test
sudo cargo flamegraph --bin ham -- status

# Open flamegraph.svg to see CPU usage
open flamegraph.svg
```

### Memory Profiling

```bash
# Using Instruments on macOS
instruments -t "Allocations" target/release/ham status

# Using Valgrind on Linux
valgrind --tool=massif target/release/ham status
ms_print massif.out.<pid>
```

### Performance Monitoring

Monitor cache effectiveness:

```rust
// Add to your test code
let repo = Repository::new(path);
println!("Cache size before: {}", repo.cache_size());

// ... perform operations ...

println!("Cache size after: {}", repo.cache_size());
```

## Stress Testing

### Rapid Operations Test

```bash
#!/bin/bash
# Test repeated status calls
for i in {1..100}; do
    ham status > /dev/null
done

# Measure average time
time for i in {1..100}; do
    ham status > /dev/null
done
```

### Concurrent Operations

```bash
# Test if operations can be run concurrently
ham status &
ham log -n 10 &
wait
```

## Performance Checklist

Before claiming performance improvements, verify:

- [ ] Tested with release build (`--release` flag)
- [ ] Tested with at least 100 files
- [ ] Tested with at least 1000 files for scalability
- [ ] Ran full benchmark suite
- [ ] Compared with baseline (before optimizations)
- [ ] Tested both cold and warm cache scenarios
- [ ] Verified parallel operations work correctly
- [ ] Profiled to confirm optimizations are effective
- [ ] No memory leaks or unbounded cache growth
- [ ] Performance scales well with repository size

## Common Issues

### "No performance improvement seen"

1. Make sure you're using `--release` build
2. Ensure test repository is large enough (100+ files)
3. Check if running on a single-core machine (parallel gains limited)
4. Verify SSD vs HDD (disk I/O bottleneck)

### "Benchmarks fail to compile"

1. Make sure all dependencies are updated: `cargo update`
2. Check Rust version: `rustc --version` (need 1.70+)
3. Clean and rebuild: `cargo clean && cargo bench`

### "Memory usage too high"

1. Check cache size with `repo.cache_size()`
2. Clear cache periodically with `repo.clear_cache()`
3. Profile memory usage with valgrind or instruments

## Reporting Performance Results

When reporting performance improvements, include:

1. **System specs**: CPU cores, RAM, disk type (SSD/HDD)
2. **Test scenario**: Number of files, repository structure
3. **Measurements**: Before/after timings with multiple runs
4. **Benchmark results**: Output from `cargo bench`
5. **Profiling data**: Flamegraphs or other profiler output

Example:
```
System: Apple M1 Pro, 8 cores, 16GB RAM, SSD
Test: 500 files across 10 directories

Results:
- ham status: 450ms -> 180ms (2.5x faster)
- ham add .: 600ms -> 220ms (2.7x faster)
- Cache hit rate: 85% on second log run
```
