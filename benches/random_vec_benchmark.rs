use rendering::math::Vec3;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn vec_benchmarks(c: &mut Criterion) {
    let mut rng = fastrand::Rng::new();
    rng.seed(10);
    let mut group = c.benchmark_group("Vec randomness");
    group.sample_size(10_000);
    group.bench_function("random vec range", |b| b.iter(|| Vec3::random_vec(-1.0, 1.0, &mut rng)));
    group.bench_function("random vec unit disk", |b| b.iter(|| Vec3::random_in_unit_disk(&mut rng)));
    group.bench_function("random vec unit sphere", |b| b.iter(|| Vec3::random_in_unit_sphere(&mut rng)));
    group.finish();
}

criterion_group!(benches, vec_benchmarks);
criterion_main!(benches);
