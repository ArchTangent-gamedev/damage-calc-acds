//! Rust benchmarks for ACDS damage calculations
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use damage_calc_acds::damage_reduction;

/// Alternative using list of (numerator, denominator) fractional values
fn damage_reduction_list(damage: i32, dr_fractions: &Vec<(i32, i32)>) -> i32 {
    let mut output = damage;

    for (numerator, denominator) in dr_fractions.iter() {
        output -= output * numerator / denominator;
    }

    output
}

/// Returns damage after damage reduction (early exit version).
pub fn damage_reduction_early_exit(damage: i32, dr_bytes: [i8; 8], non_zero: u32) -> i32 {
    let mut output = damage;
    let mut numerator = 0;
    let mut non_zeroes_remaining = non_zero;

    for &count in dr_bytes.iter() {
        if non_zeroes_remaining == 0 { 
            break; 
        };

        numerator += 1;

        if count == 0 {
            continue
        }
        if count > 0 {
            for _reduction in 0..count {
                output -= output * numerator / 8
            }
        } else if count < 0 {
            for _reduction in count..0 {
                output += output * numerator / 8
            }
        }
        non_zeroes_remaining -= 1;
    }
    return output
}

// ------------------------------
//         Bench Suite
// ------------------------------

fn suite_default(damage: i32, suite: &Vec<[i8; 8]>) -> i32 {
    let mut output = 0;
    for dr_bytes in suite.iter() {
        output += damage_reduction(damage, *dr_bytes)
    }
    output
}

fn suite_early_exit(damage: i32, suite: &Vec<([i8; 8], u32)>) -> i32 {
    let mut output = 0;
    for (dr_bytes, non_zero) in suite.iter() {
        output += damage_reduction_early_exit(damage, *dr_bytes, *non_zero)
    }
    output
}

fn suite_list(damage: i32, suite: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut output = 0;
    for dr_fractions in suite.iter() {
        output += damage_reduction_list(damage, dr_fractions)
    }
    output
}

// ------------------------------
//          Benchmarks
// ------------------------------

/// Benchmark with empty DR values.
fn bench_empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("Empty");

    let damage = 1000;

    let empty_default: Vec<[i8; 8]> = vec![
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let empty_early_exit: Vec<([i8; 8], u32)> = vec![
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
    ];
    let empty_list: Vec<Vec<(i32, i32)>> = vec![
        vec![],
    ];

    group.bench_with_input(
        BenchmarkId::new("ACDS default", "0 DRs"), &empty_default,
            |b, i| b.iter(|| suite_default(damage, i)
        )
    );

    group.bench_with_input(
        BenchmarkId::new("ACDS early exit", "0 DRs"), &empty_early_exit,
            |b, i| b.iter(|| suite_early_exit(damage, i)
        )
    );

    group.bench_with_input(
        BenchmarkId::new("Fraction List", "0 DRs"), &empty_list,
            |b, i| b.iter(|| suite_list(damage, i)
        )
    );
    group.finish();
}

/// Benchmark with 1 to 2 DR values per calculation.
fn bench_light(c: &mut Criterion) {
    let mut group = c.benchmark_group("Light");

    let damage = 1000;

    let light_default: Vec<[i8; 8]> = vec![
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let light_early_exit: Vec<([i8; 8], u32)> = vec![
        ([1, 0, 0, 0, 0, 0, 0, 0], 1),
        ([0, 1, 0, 0, 0, 0, 0, 0], 1),
        ([0, 0, 1, 0, 0, 0, 0, 0], 1),
        ([0, 0, 0, 1, 0, 0, 0, 0], 1),
        ([0, 0, 0, 0, 1, 0, 0, 0], 1),
        ([0, 0, 0, 0, 0, 1, 0, 0], 1),
        ([0, 0, 0, 0, 0, 0, 1, 0], 1),
        ([0, 0, 0, 0, 0, 0, 0, 1], 1),
    ];
    let light_list: Vec<Vec<(i32, i32)>> = vec![
        vec![(1, 8)],
        vec![(1, 4)],
        vec![(3, 8)],
        vec![(1, 2)],
        vec![(5, 8)],
        vec![(3, 4)],
        vec![(7, 8)],
        vec![(1, 1)],
    ];

    group.bench_with_input(
        BenchmarkId::new("ACDS default", "1 DR"), &light_default,
            |b, i| b.iter(|| suite_default(damage, i)
        )
    );

    group.bench_with_input(
        BenchmarkId::new("ACDS early exit", "1 DR"), &light_early_exit,
            |b, i| b.iter(|| suite_early_exit(damage, i)
        )
    );

    group.bench_with_input(
        BenchmarkId::new("Fraction List", "1 DR"), &light_list,
            |b, i| b.iter(|| suite_list(damage, i)
        )
    );

    group.finish();
}

/// Benchmark with 2 DR values per calculation.
fn bench_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("Medium");

    let damage = 1000;

    let medium_default: Vec<[i8; 8]> = vec![
        [1, 1, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 0, 0, 0, 0, 0],
        [0, 0, 1, 1, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 0, 0, 0],
        [0, 0, 0, 0, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 1, 1, 0],
        [0, 0, 0, 0, 0, 0, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 2],
    ];
    let medium_early_exit: Vec<([i8; 8], u32)> = vec![
        ([1, 1, 0, 0, 0, 0, 0, 0], 2),
        ([0, 1, 1, 0, 0, 0, 0, 0], 2),
        ([0, 0, 1, 1, 0, 0, 0, 0], 2),
        ([0, 0, 0, 1, 1, 0, 0, 0], 2),
        ([0, 0, 0, 0, 1, 1, 0, 0], 2),
        ([0, 0, 0, 0, 0, 1, 1, 0], 2),
        ([0, 0, 0, 0, 0, 0, 1, 1], 2),
        ([0, 0, 0, 0, 0, 0, 0, 2], 2),
    ];
    let medium_list: Vec<Vec<(i32, i32)>> = vec![
        vec![(1, 8), (1, 4)],
        vec![(1, 4), (3, 8)],
        vec![(3, 8), (1, 2)],
        vec![(1, 2), (5, 8)],
        vec![(5, 8), (3, 4)],
        vec![(3, 4), (7, 8)],
        vec![(7, 8), (1, 1)],
        vec![(1, 1), (1, 1)],
    ];

    group.bench_with_input(
        BenchmarkId::new("ACDS default", "2 DRs"), &medium_default,
            |b, i| b.iter(|| suite_default(damage, i)
        )
    );

    group.bench_with_input(
        BenchmarkId::new("ACDS early exit", "2 DRs"), &medium_early_exit,
            |b, i| b.iter(|| suite_early_exit(damage, i)
        )
    );

    group.bench_with_input(
        BenchmarkId::new("Fraction List", "2 DRs"), &medium_list,
            |b, i| b.iter(|| suite_list(damage, i)
        )
    );

    group.finish();
}

/// Benchmark with 5 DR values per calculation.
fn bench_heavy(c: &mut Criterion) {
    let mut group = c.benchmark_group("Heavy");

    let damage = 1000;

    let heavy_default: Vec<[i8; 8]> = vec![
        [2, 1, 1, 1, 0, 0, 0, 0],
        [0, 2, 1, 1, 1, 0, 0, 0],
        [0, 0, 2, 1, 1, 1, 0, 0],
        [0, 0, 0, 2, 1, 1, 1, 0],
        [0, 0, 0, 0, 2, 1, 1, 1],
        [0, 0, 0, 0, 0, 2, 2, 1],
        [0, 0, 0, 0, 0, 0, 3, 2],
        [0, 0, 0, 0, 0, 0, 0, 5],
    ];
    let heavy_early_exit: Vec<([i8; 8], u32)> = vec![
        ([2, 1, 1, 1, 0, 0, 0, 0], 5),
        ([0, 2, 1, 1, 1, 0, 0, 0], 5),
        ([0, 0, 2, 1, 1, 1, 0, 0], 5),
        ([0, 0, 0, 2, 1, 1, 1, 0], 5),
        ([0, 0, 0, 0, 2, 1, 1, 1], 5),
        ([0, 0, 0, 0, 0, 2, 2, 1], 5),
        ([0, 0, 0, 0, 0, 0, 3, 2], 5),
        ([0, 0, 0, 0, 0, 0, 0, 5], 5),
    ];
    let heavy_list: Vec<Vec<(i32, i32)>> = vec![
        vec![(1, 8), (1, 8), (1, 4), (3, 8), (1, 2)],
        vec![(1, 4), (1, 4), (3, 8), (1, 2), (5, 8)],
        vec![(3, 8), (3, 8), (1, 2), (5, 8), (3, 4)],
        vec![(1, 2), (1, 2), (5, 8), (3, 4), (7, 8)],
        vec![(5, 8), (5, 8), (3, 4), (7, 8), (1, 1)],
        vec![(3, 4), (3, 4), (7, 8), (7, 8), (1, 1)],
        vec![(7, 8), (7, 8), (7, 8), (1, 1), (1, 1)],
        vec![(1, 1), (1, 1), (1, 1), (1, 1), (1, 1)],
    ];

    group.bench_with_input(
        BenchmarkId::new("ACDS default", "5 DRs"), &heavy_default,
            |b, i| b.iter(|| suite_default(damage, i)
        )
    );

    group.bench_with_input(
        BenchmarkId::new("ACDS early exit", "5 DRs"), &heavy_early_exit,
            |b, i| b.iter(|| suite_early_exit(damage, i)
        )
    );

    group.bench_with_input(
        BenchmarkId::new("Fraction List", "5 DRs"), &heavy_list,
            |b, i| b.iter(|| suite_list(damage, i)
        )
    );

    group.finish();
}

criterion_group!(benches, bench_empty, bench_light, bench_medium, bench_heavy);
criterion_main!(benches);
