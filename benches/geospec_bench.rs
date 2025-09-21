use criterion::{black_box, Criterion, criterion_group, criterion_main};
use geospec::*;

fn bench_sphere(c: &mut Criterion) {
    let s = Sphere { radius: 10.0 };

    c.bench_function("sphere volume", |b| {
        b.iter(|| black_box(s.volume()))
    });

    c.bench_function("sphere surface_area", |b| {
        b.iter(|| black_box(s.surface_area()))
    });
}

fn bench_box(c: &mut Criterion) {
    let b = BoxShape { length: 10.0, width: 5.0, height: 2.0 };

    c.bench_function("box volume", |bch| {
        bch.iter(|| black_box(b.volume()))
    });

    c.bench_function("box surface_area", |bch| {
        bch.iter(|| black_box(b.surface_area()))
    });
}

criterion_group!(benches, bench_sphere, bench_box);
criterion_main!(benches);
