/// Binary Styles Performance Benchmarks
/// 
/// Compares all 5 optimization levels from STYLE.md

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use style::binary::*;
use std::time::Duration;

fn benchmark_level1_binary_ids(c: &mut Criterion) {
    let mut group = c.benchmark_group("Level 1: Binary IDs");
    
    let test_cases = vec![
        (vec!["flex"], "single"),
        (vec!["flex", "items-center", "p-4"], "common"),
        (vec!["flex", "flex-col", "items-center", "justify-center", "p-4", "bg-white", "rounded-lg", "shadow-md"], "complex"),
    ];
    
    for (classes, name) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &classes, |b, classes| {
            b.iter(|| {
                let ids: Vec<StyleId> = classes
                    .iter()
                    .filter_map(|name| style_name_to_id(name))
                    .collect();
                black_box(ids)
            });
        });
    }
    
    group.finish();
}

fn benchmark_level2_csstext(c: &mut Criterion) {
    let mut group = c.benchmark_group("Level 2: Direct cssText");
    
    let test_cases = vec![
        (vec![4u16], "single"),
        (vec![4, 26, 35], "common"),
        (vec![4, 13, 26, 20, 35, 190, 261, 353], "complex"),
    ];
    
    for (ids, name) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &ids, |b, ids| {
            b.iter(|| {
                let css = apply_styles_direct(ids);
                black_box(css)
            });
        });
    }
    
    group.finish();
}

fn benchmark_level3_combos(c: &mut Criterion) {
    let mut group = c.benchmark_group("Level 3: Pre-Computed Combos");
    
    let test_cases = vec![
        (vec![4u16, 26, 35], "flex+items-center+p-4"),
        (vec![172, 203], "text-white+bg-blue-500"),
        (vec![4, 20, 26], "flex+justify-center+items-center"),
    ];
    
    for (ids, name) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &ids, |b, ids| {
            b.iter(|| {
                let css = try_apply_combo(ids).unwrap_or("");
                black_box(css)
            });
        });
    }
    
    group.finish();
}

fn benchmark_level4_varint(c: &mut Criterion) {
    let mut group = c.benchmark_group("Level 4: Varint Encoding");
    
    let test_cases = vec![
        (vec![4u16, 26, 35], "common"),
        ((0..100).collect::<Vec<_>>(), "100_ids"),
        ((0..256).collect::<Vec<_>>(), "256_ids"),
    ];
    
    for (ids, name) in test_cases {
        group.bench_with_input(BenchmarkId::new("encode", name), &ids, |b, ids| {
            b.iter(|| {
                let encoded = encode_id_list(ids);
                black_box(encoded)
            });
        });
        
        let encoded = encode_id_list(&ids);
        group.bench_with_input(BenchmarkId::new("decode", name), &encoded, |b, encoded| {
            b.iter(|| {
                let decoded = decode_id_list(encoded).unwrap();
                black_box(decoded)
            });
        });
    }
    
    group.finish();
}

fn benchmark_level5_binary_values(c: &mut Criterion) {
    let mut group = c.benchmark_group("Level 5: Binary CSS Values");
    
    let test_cases = vec![
        (vec![(CssProperty::Display, DisplayValue::Flex as u8)], "single"),
        (vec![
            (CssProperty::Display, DisplayValue::Flex as u8),
            (CssProperty::AlignItems, AlignItemsValue::Center as u8),
        ], "double"),
        (vec![
            (CssProperty::Display, DisplayValue::Flex as u8),
            (CssProperty::FlexDirection, 2),
            (CssProperty::AlignItems, AlignItemsValue::Center as u8),
            (CssProperty::JustifyContent, 2),
        ], "complex"),
    ];
    
    for (props, name) in test_cases {
        group.bench_with_input(BenchmarkId::new("encode", name), &props, |b, props| {
            b.iter(|| {
                let stream = encode_properties(props);
                black_box(stream)
            });
        });
        
        let stream = encode_properties(&props);
        group.bench_with_input(BenchmarkId::new("decode", name), &stream, |b, stream| {
            b.iter(|| {
                let css = apply_binary_css(stream).unwrap();
                black_box(css)
            });
        });
    }
    
    group.finish();
}

fn benchmark_end_to_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("End-to-End Comparison");
    group.measurement_time(Duration::from_secs(10));
    
    let classes = vec!["flex", "items-center", "p-4", "text-white", "bg-blue-500", "rounded-lg", "shadow-md"];
    
    // Level 1: Class names → IDs
    group.bench_function("level1_lookup", |b| {
        b.iter(|| {
            let ids: Vec<StyleId> = classes
                .iter()
                .filter_map(|name| style_name_to_id(name))
                .collect();
            black_box(ids)
        });
    });
    
    // Level 2: IDs → CSS text
    group.bench_function("level2_csstext", |b| {
        let ids: Vec<StyleId> = classes
            .iter()
            .filter_map(|name| style_name_to_id(name))
            .collect();
        
        b.iter(|| {
            let css = apply_styles_direct(&ids);
            black_box(css)
        });
    });
    
    // Level 3: Try combo, fallback to direct
    group.bench_function("level3_combo", |b| {
        let ids: Vec<StyleId> = classes
            .iter()
            .filter_map(|name| style_name_to_id(name))
            .collect();
        
        b.iter(|| {
            let css = if let Some(combo_css) = try_apply_combo(&ids) {
                combo_css.to_string()
            } else {
                apply_styles_direct(&ids)
            };
            black_box(css)
        });
    });
    
    // Level 4: Full varint roundtrip
    group.bench_function("level4_varint", |b| {
        let ids: Vec<StyleId> = classes
            .iter()
            .filter_map(|name| style_name_to_id(name))
            .collect();
        
        b.iter(|| {
            let encoded = encode_id_list(&ids);
            let decoded = decode_id_list(&encoded).unwrap();
            let css = apply_styles_direct(&decoded);
            black_box(css)
        });
    });
    
    // Auto mode (best path selection)
    group.bench_function("auto_mode", |b| {
        b.iter(|| {
            let css = generate_css_optimized(&classes, EncodingMode::Auto);
            black_box(css)
        });
    });
    
    group.finish();
}

fn benchmark_payload_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Payload Size Comparison");
    
    let classes = vec!["flex", "items-center", "p-4", "text-white", "bg-blue-500"];
    
    group.bench_function("original_strings", |b| {
        b.iter(|| {
            let size: usize = classes.iter().map(|s| s.len()).sum();
            black_box(size)
        });
    });
    
    group.bench_function("binary_ids_u16", |b| {
        let ids: Vec<StyleId> = classes
            .iter()
            .filter_map(|name| style_name_to_id(name))
            .collect();
        
        b.iter(|| {
            let size = ids.len() * std::mem::size_of::<u16>();
            black_box(size)
        });
    });
    
    group.bench_function("varint_encoded", |b| {
        let ids: Vec<StyleId> = classes
            .iter()
            .filter_map(|name| style_name_to_id(name))
            .collect();
        
        b.iter(|| {
            let encoded = encode_id_list(&ids);
            black_box(encoded.len())
        });
    });
    
    group.bench_function("combo_id", |b| {
        // Assuming it's a combo: just 2 bytes
        b.iter(|| {
            black_box(2)
        });
    });
    
    group.finish();
}

fn benchmark_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("Scalability Test");
    
    let sizes = vec![10, 50, 100, 500, 1000];
    
    for size in sizes {
        let classes: Vec<String> = (0..size)
            .map(|i| format!("class-{}", i % 100))
            .collect();
        let class_refs: Vec<&str> = classes.iter().map(|s| s.as_str()).collect();
        
        group.bench_with_input(
            BenchmarkId::new("auto_mode", size),
            &class_refs,
            |b, classes| {
                b.iter(|| {
                    let css = generate_css_optimized(classes, EncodingMode::Auto);
                    black_box(css)
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_level1_binary_ids,
    benchmark_level2_csstext,
    benchmark_level3_combos,
    benchmark_level4_varint,
    benchmark_level5_binary_values,
    benchmark_end_to_end,
    benchmark_payload_sizes,
    benchmark_scalability,
);

criterion_main!(benches);
