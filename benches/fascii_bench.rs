use criterion::{criterion_group, criterion_main, Criterion};
use fascii::is_ascii;

pub fn criterion_benchmark(c: &mut Criterion) {
    // goal is 100 μs for 6MB
    let s = vec![122; 6_000_000];
    let s = String::from_utf8(s).unwrap();
    c.bench_function("FASCII is_ascii 6MB", |b| b.iter(|| is_ascii(&s)));
    c.bench_function("STD is_ascii 6MB", |b| b.iter(|| s.is_ascii()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
