use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tempfile::TempDir;
use std::fs;
use std::path::PathBuf;

// Mock minimal parts of hamster for benchmarking
mod hamster_bench {
    use std::path::PathBuf;
    use sha2::{Sha256, Digest};
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::io::Write;
    
    pub fn hash_object(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
    
    pub fn compress_data(data: &[u8]) -> Vec<u8> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }
    
    pub fn create_test_file(path: &PathBuf, size: usize) {
        let content = vec![b'a'; size];
        std::fs::write(path, content).unwrap();
    }
}

fn bench_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashing");
    
    for size in [1024, 10_240, 102_400, 1_024_000].iter() {
        let data = vec![b'x'; *size];
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                hamster_bench::hash_object(black_box(&data))
            });
        });
    }
    
    group.finish();
}

fn bench_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression");
    
    for size in [1024, 10_240, 102_400].iter() {
        let data = vec![b'x'; *size];
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                hamster_bench::compress_data(black_box(&data))
            });
        });
    }
    
    group.finish();
}

fn bench_file_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_operations");
    let temp_dir = TempDir::new().unwrap();
    
    for size in [1024, 10_240, 102_400].iter() {
        let file_path = temp_dir.path().join(format!("test_{}.txt", size));
        hamster_bench::create_test_file(&file_path, *size);
        
        group.bench_with_input(BenchmarkId::new("read", size), size, |b, _| {
            b.iter(|| {
                fs::read(black_box(&file_path))
            });
        });
    }
    
    group.finish();
}

fn bench_parallel_vs_sequential(c: &mut Criterion) {
    use rayon::prelude::*;
    
    let mut group = c.benchmark_group("parallel_comparison");
    let data: Vec<Vec<u8>> = (0..100).map(|_| vec![b'x'; 1024]).collect();
    
    group.bench_function("sequential_hashing", |b| {
        b.iter(|| {
            let _: Vec<_> = data.iter()
                .map(|d| hamster_bench::hash_object(black_box(d)))
                .collect();
        });
    });
    
    group.bench_function("parallel_hashing", |b| {
        b.iter(|| {
            let _: Vec<_> = data.par_iter()
                .map(|d| hamster_bench::hash_object(black_box(d)))
                .collect();
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_hashing,
    bench_compression,
    bench_file_operations,
    bench_parallel_vs_sequential
);
criterion_main!(benches);
