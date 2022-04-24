use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rubikscube::{Cube, MetricKind, Turn};

fn criterion_benchmark(c: &mut Criterion) {
    let mut cube = Cube::new(MetricKind::HalfTurnMetric);
    c.bench_function("cube turn", |b| b.iter(|| cube._turn(black_box(Turn::F))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
