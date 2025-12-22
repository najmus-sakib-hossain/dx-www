//! Benchmarks for dx-check
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use dx_check::config::CheckerConfig;
use dx_check::engine::Checker;
use dx_check::scanner::PatternScanner;
use std::path::Path;

/// Sample JavaScript code for benchmarking
const SAMPLE_JS: &str = r#"
import { useState, useEffect } from 'react';

export function UserProfile({ userId }) {
    const [user, setUser] = useState(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        async function fetchUser() {
            const response = await fetch(`/api/users/${userId}`);
            const data = await response.json();
            setUser(data);
            setLoading(false);
        }
        fetchUser();
    }, [userId]);

    if (loading) {
        return <div>Loading...</div>;
    }

    return (
        <div className="user-profile">
            <h1>{user.name}</h1>
            <p>{user.email}</p>
            <p>{user.bio}</p>
        </div>
    );
}
"#;

/// Sample code with issues
const SAMPLE_WITH_ISSUES: &str = r#"
// Bad code for testing
var x = 1;
console.log(x);
debugger;
if (x == 1) {
    eval('alert("hello")');
}
"#;

fn bench_parse_and_lint(c: &mut Criterion) {
    let checker = Checker::new(CheckerConfig::default());
    
    c.bench_function("parse_and_lint_simple", |b| {
        b.iter(|| {
            black_box(checker.check_source(Path::new("test.js"), SAMPLE_JS))
        })
    });

    c.bench_function("parse_and_lint_with_issues", |b| {
        b.iter(|| {
            black_box(checker.check_source(Path::new("test.js"), SAMPLE_WITH_ISSUES))
        })
    });
}

fn bench_simd_scanner(c: &mut Criterion) {
    let scanner = PatternScanner::new();
    
    // Generate larger samples
    let large_sample = SAMPLE_JS.repeat(100);
    
    let mut group = c.benchmark_group("simd_scanner");
    
    group.throughput(Throughput::Bytes(large_sample.len() as u64));
    
    group.bench_function("scan_large_file", |b| {
        b.iter(|| {
            black_box(scanner.scan(large_sample.as_bytes()))
        })
    });
    
    group.bench_function("has_any_match_clean", |b| {
        b.iter(|| {
            black_box(scanner.has_any_match(SAMPLE_JS.as_bytes()))
        })
    });
    
    group.bench_function("has_any_match_with_issues", |b| {
        b.iter(|| {
            black_box(scanner.has_any_match(SAMPLE_WITH_ISSUES.as_bytes()))
        })
    });
    
    group.finish();
}

fn bench_scaling(c: &mut Criterion) {
    let checker = Checker::new(CheckerConfig::default());
    
    let mut group = c.benchmark_group("scaling");
    
    for size in [1, 10, 100, 1000].iter() {
        let code = SAMPLE_JS.repeat(*size);
        group.throughput(Throughput::Bytes(code.len() as u64));
        
        group.bench_with_input(
            BenchmarkId::new("lint", size),
            &code,
            |b, code| {
                b.iter(|| {
                    black_box(checker.check_source(Path::new("test.js"), code))
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_parse_and_lint,
    bench_simd_scanner,
    bench_scaling,
);

criterion_main!(benches);
