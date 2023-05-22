use vecrs::vectors::{Vec2, Vec3, Vec4};

// Test vector by vector operators
//
// Test Vec2 by Vec2 operators
#[test]
fn vec2_by_vec2_ops() {
    let a = Vec2::new(4., 6.);
    let b = Vec2::new(2., 3.);

    // Test operators
    assert_eq!(Vec2::new(6., 9.), a + b, "add ops");
    assert_eq!(Vec2::new(2., 3.), a - b, "sub ops");
    assert_eq!(Vec2::new(8., 18.), a * b, "mul ops");
    assert_eq!(Vec2::new(2., 2.), a / b, "div ops");
}

// Test Vec3 by Vec3 operators
#[test]
fn vec3_by_vec3_ops() {
    let a = Vec3::new(4., 6., 8.);
    let b = Vec3::new(2., 3., 4.);

    // Test operators
    assert_eq!(Vec3::new(6., 9., 12.), a + b, "add ops");
    assert_eq!(Vec3::new(2., 3., 4.), a - b, "sub ops");
    assert_eq!(Vec3::new(8., 18., 32.), a * b, "mul ops");
    assert_eq!(Vec3::new(2., 2., 2.), a / b, "div ops");
}

// Test Vec4 by Vec4 operators
#[test]
fn vec4_by_vec4_ops() {
    let a = Vec4::new(4., 6., 8., 10.);
    let b = Vec4::new(2., 3., 4., 5.);

    // Test operators
    assert_eq!(Vec4::new(6., 9., 12., 15.), a + b, "add ops");
    assert_eq!(Vec4::new(2., 3., 4., 5.), a - b, "sub ops");
    assert_eq!(Vec4::new(8., 18., 32., 50.), a * b, "mul ops");
    assert_eq!(Vec4::new(2., 2., 2., 2.), a / b, "div ops");
}

// Test vector by float operators
//
// Test Vec2 by float operators
#[test]
fn vec2_by_f64_ops() {
    let a = Vec2::new(4., 6.);
    let b = 2f64;

    // Test operators
    assert_eq!(Vec2::new(6., 8.), a + b, "add ops");
    assert_eq!(Vec2::new(2., 4.), a - b, "sub ops");
    assert_eq!(Vec2::new(8., 12.), a * b, "mul ops");
    assert_eq!(Vec2::new(2., 3.), a / b, "div ops");
}

// Test Vec3 by float operators
#[test]
fn vec3_by_f64_ops() {
    let a = Vec3::new(4., 6., 8.);
    let b = 2f64;

    // Test operators
    assert_eq!(Vec3::new(6., 8., 10.), a + b, "add ops");
    assert_eq!(Vec3::new(2., 4., 6.), a - b, "sub ops");
    assert_eq!(Vec3::new(8., 12., 16.), a * b, "mul ops");
    assert_eq!(Vec3::new(2., 3., 4.), a / b, "div ops");
}

// Test Vec4 by float operators
#[test]
fn vec4_by_f64_ops() {
    let a = Vec4::new(4., 6., 8., 10.);
    let b = 2f64;

    // Test operators
    assert_eq!(Vec4::new(6., 8., 10., 12.), a + b, "add ops");
    assert_eq!(Vec4::new(2., 4., 6., 8.), a - b, "sub ops");
    assert_eq!(Vec4::new(8., 12., 16., 20.), a * b, "mul ops");
    assert_eq!(Vec4::new(2., 3., 4., 5.), a / b, "div ops");
}
