use auth::password::hash_password;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub const TEST_PASSWORD: &str = "testPassword1!";

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hash_password", |b| {
        b.iter(|| hash_password(black_box(TEST_PASSWORD)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
