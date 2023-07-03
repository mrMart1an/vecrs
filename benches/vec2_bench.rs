use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vecrs::vectors::Vec2;


// Benchmark the normalization function
pub fn vec2_normilze(c: &mut Criterion) {
    let a = black_box(Vec2::new(1.,2.));

    c.bench_function("vec2 normalize", |b| b.iter(|| a.normalize()));
}

// Benchmark the dot product function
pub fn vec2_dot(c: &mut Criterion) {
    let a_vec = black_box(Vec2::new(1.,2.));
    let b_vec = black_box(Vec2::new(2.,3.));

    c.bench_function("vec2 dot", |b| b.iter(|| Vec2::dot(a_vec, b_vec)));
}

// Benchmark the add function
pub fn vec2_add(c: &mut Criterion) {
    let a_vec = black_box(Vec2::new(1.,2.));
    let b_vec = black_box(Vec2::new(2.,3.));

    c.bench_function("vec2 add", |b| b.iter(|| a_vec + b_vec));
}

// Benchmark the add and the dot operation
pub fn vec2_add_dot(c: &mut Criterion) {
    let a_vec = black_box(Vec2::new(1.,2.));
    let b_vec = black_box(Vec2::new(2.,3.));

    c.bench_function("vec2 add + dot", |b| b.iter(|| {
        let res = a_vec + b_vec;
        Vec2::dot(a_vec, res)
    }));
}

// Create the benchmarks group
criterion_group!(
    benches, 
    
    vec2_normilze,
    vec2_dot,
    vec2_add,
    vec2_add_dot,
);

// Run the benchmarks
criterion_main!(benches);
