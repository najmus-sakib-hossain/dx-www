//! GC Benchmarks
//!
//! Benchmarks for garbage collection operations.

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use dx_py_gc::{LockFreeRefCount, EpochGc};
use std::sync::Arc;

fn bench_refcount_inc_dec(c: &mut Criterion) {
    let mut group = c.benchmark_group("refcount_inc_dec");
    
    for count in [100, 1000, 10000].iter() {
        let rc = LockFreeRefCount::new();
        
        group.throughput(Throughput::Elements(*count as u64 * 2)); // inc + dec
        
        group.bench_with_input(BenchmarkId::new("atomic", count), count, |b, count| {
            b.iter(|| {
                for _ in 0..**count {
                    rc.inc_strong();
                }
                for _ in 0..**count {
                    rc.dec_strong();
                }
            })
        });
    }
    
    group.finish();
}

fn bench_refcount_concurrent(c: &mut Criterion) {
    let mut group = c.benchmark_group("refcount_concurrent");
    
    for threads in [2, 4, 8].iter() {
        let rc = Arc::new(LockFreeRefCount::new());
        
        group.throughput(Throughput::Elements(1000 * *threads as u64));
        
        group.bench_with_input(BenchmarkId::new("threads", threads), threads, |b, threads| {
            b.iter(|| {
                let handles: Vec<_> = (0..**threads)
                    .map(|_| {
                        let rc = Arc::clone(&rc);
                        std::thread::spawn(move || {
                            for _ in 0..1000 {
                                rc.inc_strong();
                                rc.dec_strong();
                            }
                        })
                    })
                    .collect();
                
                for h in handles {
                    h.join().unwrap();
                }
            })
        });
    }
    
    group.finish();
}

fn bench_epoch_enter_exit(c: &mut Criterion) {
    let mut group = c.benchmark_group("epoch_enter_exit");
    
    let gc = EpochGc::new();
    
    for count in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("epoch", count), count, |b, count| {
            b.iter(|| {
                for _ in 0..**count {
                    let _guard = gc.enter_epoch();
                }
            })
        });
    }
    
    group.finish();
}

fn bench_epoch_defer_free(c: &mut Criterion) {
    let mut group = c.benchmark_group("epoch_defer_free");
    
    for count in [100, 1000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("defer", count), count, |b, count| {
            b.iter(|| {
                let gc = EpochGc::new();
                for _ in 0..**count {
                    let data = Box::new(42i64);
                    let ptr = Box::into_raw(data);
                    gc.defer_free(ptr as *mut u8, std::mem::size_of::<i64>());
                }
                // Collect to free memory
                for _ in 0..5 {
                    gc.try_collect();
                }
            })
        });
    }
    
    group.finish();
}

fn bench_epoch_collect(c: &mut Criterion) {
    let mut group = c.benchmark_group("epoch_collect");
    
    for garbage_count in [100, 1000].iter() {
        let gc = EpochGc::new();
        
        // Pre-populate with garbage
        for _ in 0..*garbage_count {
            let data = Box::new(42i64);
            let ptr = Box::into_raw(data);
            gc.defer_free(ptr as *mut u8, std::mem::size_of::<i64>());
        }
        
        group.throughput(Throughput::Elements(*garbage_count as u64));
        
        group.bench_with_input(BenchmarkId::new("collect", garbage_count), garbage_count, |b, _| {
            b.iter(|| {
                gc.try_collect();
            })
        });
    }
    
    group.finish();
}

fn bench_refcount_weak(c: &mut Criterion) {
    let mut group = c.benchmark_group("refcount_weak");
    
    for count in [100, 1000, 10000].iter() {
        let rc = LockFreeRefCount::new();
        
        group.throughput(Throughput::Elements(*count as u64 * 2));
        
        group.bench_with_input(BenchmarkId::new("weak", count), count, |b, count| {
            b.iter(|| {
                for _ in 0..**count {
                    rc.inc_weak();
                }
                for _ in 0..**count {
                    rc.dec_weak();
                }
            })
        });
    }
    
    group.finish();
}

fn bench_refcount_mark(c: &mut Criterion) {
    let mut group = c.benchmark_group("refcount_mark");
    
    for count in [100, 1000, 10000].iter() {
        let rc = LockFreeRefCount::new();
        
        group.throughput(Throughput::Elements(*count as u64 * 2));
        
        group.bench_with_input(BenchmarkId::new("mark_unmark", count), count, |b, count| {
            b.iter(|| {
                for _ in 0..**count {
                    rc.mark_for_cycle();
                    rc.unmark();
                }
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_refcount_inc_dec,
    bench_refcount_concurrent,
    bench_epoch_enter_exit,
    bench_epoch_defer_free,
    bench_epoch_collect,
    bench_refcount_weak,
    bench_refcount_mark,
);

criterion_main!(benches);
