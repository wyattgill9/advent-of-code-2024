use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_day1(c: &mut Criterion) {
    c.bench_function("day1", |b| {
        b.iter(|| {
            // Your day1 code here
        });
    });
}

fn benchmark_day2(c: &mut Criterion) {
    c.bench_function("day2", |b| {
        b.iter(|| {
            // Your day2 code here
        });
    });
}

criterion_group!(benches, benchmark_day1, benchmark_day2);
criterion_main!(benches);
