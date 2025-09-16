use criterion::{criterion_group, criterion_main, Criterion};
use geospec::*;

fn bench_encode_mask(c: &mut Criterion) {
    let params = ["length", "radius", "volume"];
    c.bench_function("encode_mask", |b| {
        b.iter(|| encode_mask(&params))
    });
}

fn bench_decode_mask(c: &mut Criterion) {
    let mask = encode_mask(&["length", "radius", "volume"]);
    c.bench_function("decode_mask", |b| {
        b.iter(|| decode_mask(mask))
    });
}

fn bench_normalize_quaternion(c: &mut Criterion) {
    let q = [2.0, 3.0, 4.0, 5.0];
    c.bench_function("normalize_quaternion", |b| {
        b.iter(|| normalize_quaternion(q))
    });
}

fn bench_bubble_from_geospec(c: &mut Criterion) {
    let mask = encode_mask(&["length", "width", "height"]);
    let values = vec![10.0, 20.0, 5.0];
    c.bench_function("bubble_from_geospec", |b| {
        b.iter(|| bubble_from_geospec(mask, &values))
    });
}

fn bench_shape_from_spec(c: &mut Criterion) {
    let mask = encode_mask(&["radius", "length"]);
    let values = vec![2.0, 50.0];
    c.bench_function("shape_from_spec", |b| {
        b.iter(|| shape_from_spec(mask, &values, None))
    });
}

criterion_group!(
    benches,
    bench_encode_mask,
    bench_decode_mask,
    bench_normalize_quaternion,
    bench_bubble_from_geospec,
    bench_shape_from_spec
);
criterion_main!(benches);
