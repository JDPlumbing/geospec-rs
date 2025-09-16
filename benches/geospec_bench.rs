use criterion::{criterion_group, criterion_main, Criterion};
use geospec::{encode_mask, decode_mask, normalize_quaternion, bubble_from_geospec, shape_from_spec};

fn bench_encode_mask(c: &mut Criterion) {
    c.bench_function("encode_mask", |b| {
        b.iter(|| encode_mask(&["length", "radius", "volume"]))
    });
}

fn bench_decode_mask(c: &mut Criterion) {
    let mask = encode_mask(&["length", "radius", "volume"]);
    c.bench_function("decode_mask", |b| {
        b.iter(|| decode_mask(mask))
    });
}

fn bench_quaternion(c: &mut Criterion) {
    let q = [0.5, 0.5, 0.5, 0.5];
    c.bench_function("normalize_quaternion", |b| {
        b.iter(|| normalize_quaternion(q))
    });
}

fn bench_shape(c: &mut Criterion) {
    let mask = encode_mask(&["radius", "length"]);
    let values = vec![2.0, 10.0];
    c.bench_function("shape_from_spec", |b| {
        b.iter(|| shape_from_spec(mask, &values, None))
    });
}

criterion_group!(benches, bench_encode_mask, bench_decode_mask, bench_quaternion, bench_shape);
criterion_main!(benches);
