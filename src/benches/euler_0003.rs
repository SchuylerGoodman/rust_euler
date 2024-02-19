use euler::lpf;

use criterion::{ black_box, criterion_group, criterion_main, Criterion };

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("0003");
    group.bench_function("0003", |b| b.iter(|| lpf(black_box(13195))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
