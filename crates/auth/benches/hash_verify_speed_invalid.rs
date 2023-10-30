use auth::password::{hash_password, verify_password};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub const TEST_PASSWORD: &str = "testPassword1!";

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hash_password + verify_password (invalid)", |b| {
        b.iter(|| {
            let hash = hash_password(black_box(TEST_PASSWORD)).unwrap();

            verify_password(black_box("Invalid Password!"), black_box(&hash))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
