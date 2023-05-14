use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fibext::Fibonacci;

fn fibonacci_u32(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci u32");
    let mut fib = Fibonacci::<u32>::new();

    group.bench_function("next", |b| b.iter(|| black_box(fib.next())));

    group.finish();
}

fn fibonacci_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci u64");
    let mut fib = Fibonacci::<u64>::new();

    group.bench_function("next", |b| b.iter(|| black_box(fib.next())));

    group.finish();
}

#[cfg(feature = "large-numbers")]
fn fibonacci_big_uint(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci BigUint");
    let mut fib = Fibonacci::<num_bigint::BigUint>::new();

    group.bench_function("next", |b| b.iter(|| black_box(fib.next())));

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = fibonacci_u32, fibonacci_u64,
}

#[cfg(feature = "large-numbers")]
criterion_group! {
    name = benches_big_uint;
    config = Criterion::default();
    targets = fibonacci_big_uint,
}

#[cfg(feature = "large-numbers")]
criterion_main!(benches, benches_big_uint);

#[cfg(not(feature = "large-numbers"))]
criterion_main!(benches);
