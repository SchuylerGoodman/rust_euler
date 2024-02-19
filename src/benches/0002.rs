use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("0002");
    group.bench_function("0002 fast", |b| b.iter(|| fib_fast()));
    group.bench_function("0002 slow", |b| b.iter(|| fib_slow()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);