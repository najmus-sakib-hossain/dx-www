//! Collections Benchmarks
//!
//! Benchmarks for SIMD-accelerated collection operations.

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use dx_py_collections::{SimdList, SwissDict};

fn bench_simd_list_sum_int(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_list_sum_int");
    
    for size in [100, 1000, 10000, 100000].iter() {
        let list = SimdList::from_ints((0..*size as i64).collect());
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("simd", size), size, |b, _| {
            b.iter(|| list.sum_int())
        });
    }
    
    group.finish();
}

fn bench_simd_list_sum_float(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_list_sum_float");
    
    for size in [100, 1000, 10000, 100000].iter() {
        let list = SimdList::from_floats((0..*size).map(|i| i as f64).collect());
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("simd", size), size, |b, _| {
            b.iter(|| list.sum_float())
        });
    }
    
    group.finish();
}

fn bench_simd_list_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_list_filter");
    
    for size in [100, 1000, 10000].iter() {
        let list = SimdList::from_ints((0..*size as i64).collect());
        let threshold = (*size as i64) / 2;
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("simd", size), size, |b, _| {
            b.iter(|| list.filter_gt_int(threshold))
        });
    }
    
    group.finish();
}

fn bench_simd_list_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_list_index");
    
    for size in [100, 1000, 10000].iter() {
        let list = SimdList::from_ints((0..*size as i64).collect());
        let target = (*size as i64) - 1; // Search for last element (worst case)
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("simd", size), size, |b, _| {
            b.iter(|| list.index_int(target))
        });
    }
    
    group.finish();
}

fn bench_simd_list_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_list_count");
    
    for size in [100, 1000, 10000].iter() {
        // Create list with some repeated values
        let list = SimdList::from_ints((0..*size as i64).map(|i| i % 10).collect());
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("simd", size), size, |b, _| {
            b.iter(|| list.count_int(5))
        });
    }
    
    group.finish();
}

fn bench_swiss_dict_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("swiss_dict_insert");
    
    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("swiss", size), size, |b, size| {
            b.iter(|| {
                let mut dict: SwissDict<i64, i64> = SwissDict::new();
                for i in 0..**size as i64 {
                    dict.insert(i, i * 2);
                }
                dict
            })
        });
    }
    
    group.finish();
}

fn bench_swiss_dict_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("swiss_dict_get");
    
    for size in [100, 1000, 10000].iter() {
        let mut dict: SwissDict<i64, i64> = SwissDict::new();
        for i in 0..*size as i64 {
            dict.insert(i, i * 2);
        }
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("swiss", size), size, |b, size| {
            b.iter(|| {
                let mut sum = 0i64;
                for i in 0..**size as i64 {
                    if let Some(v) = dict.get(&i) {
                        sum += v;
                    }
                }
                sum
            })
        });
    }
    
    group.finish();
}

fn bench_swiss_dict_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("swiss_dict_contains");
    
    for size in [100, 1000, 10000].iter() {
        let mut dict: SwissDict<i64, i64> = SwissDict::new();
        for i in 0..*size as i64 {
            dict.insert(i, i * 2);
        }
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("swiss", size), size, |b, size| {
            b.iter(|| {
                let mut count = 0;
                for i in 0..**size as i64 {
                    if dict.contains_key(&i) {
                        count += 1;
                    }
                }
                count
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_simd_list_sum_int,
    bench_simd_list_sum_float,
    bench_simd_list_filter,
    bench_simd_list_index,
    bench_simd_list_count,
    bench_swiss_dict_insert,
    bench_swiss_dict_get,
    bench_swiss_dict_contains,
);

criterion_main!(benches);
