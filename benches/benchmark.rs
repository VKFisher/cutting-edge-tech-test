use criterion::{criterion_group, criterion_main, Criterion};
use main::solve;

#[allow(clippy::redundant_closure)]
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve", |b| b.iter(|| solve()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
