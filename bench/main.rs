mod utils;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use utils::Test;

fn bench_gitnu(n: usize) {
    let t = Test::new();
    t.setup(n);
    t.run_gitnu();
}

fn bench_git(n: usize) {
    let t = Test::new();
    t.setup(n);
    t.run_git();
}

const RUNS: &[usize] = &[50];

fn bench_gits(c: &mut Criterion) {
    let mut group = c.benchmark_group("Git");
    group.warm_up_time(Duration::new(2, 0));
    group.sample_size(100);
    for i in RUNS.iter() {
        group.bench_with_input(BenchmarkId::new("gitnu", i), i, |b, i| {
            b.iter(|| bench_gitnu(*i))
        });
        group.bench_with_input(BenchmarkId::new("git", i), i, |b, i| {
            b.iter(|| bench_git(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_gits);
criterion_main!(benches);
