use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vecrs::vectors::Vec3;


// Becnhmrk the normalization function
pub fn vec3_normilze(c: &mut Criterion) {
    let a = black_box(Vec3::new(1.,2.,1.5));

    c.bench_function("vec3 normalize", |b| b.iter(|| a.normalize()));
}

// Benchmark the dot product function
pub fn vec3_dot(c: &mut Criterion) {
    let a_vec = black_box(Vec3::new(1.,2.,1.5));
    let b_vec = black_box(Vec3::new(2.,3.,5.2));

    c.bench_function("vec3 dot", |b| b.iter(|| Vec3::dot(&a_vec, &b_vec)));
}

// Benchmark the cross product function
pub fn vec3_cross(c: &mut Criterion) {
    let a_vec = black_box(Vec3::new(1.,2.,1.5));
    let b_vec = black_box(Vec3::new(2.,3.,5.2));

    c.bench_function("vec3 cross", |b| b.iter(|| Vec3::cross(&a_vec, &b_vec)));
}

// Benchmark the add function
pub fn vec3_add(c: &mut Criterion) {
    let a_vec = black_box(Vec3::new(1.,2.,1.5));
    let b_vec = black_box(Vec3::new(2.,3.,5.2));

    c.bench_function("vec3 add", |b| b.iter(|| a_vec + b_vec));
}

// Becnhmrk the add and the dot operation
pub fn vec3_add_dot(c: &mut Criterion) {
    let a_vec = black_box(Vec3::new(1.,2.,1.5));
    let b_vec = black_box(Vec3::new(2.,3.,5.2));

    c.bench_function("vec3 add + dot", |b| b.iter(|| {
        let res = a_vec + b_vec;
        Vec3::dot(&a_vec, &res)
    }));
}

// Create the benchmarks group
criterion_group!(
    benches, 
    
    vec3_normilze,
    vec3_dot,
    vec3_cross,
    vec3_add,
    vec3_add_dot,
);

// Run the benchmarks
criterion_main!(benches);
