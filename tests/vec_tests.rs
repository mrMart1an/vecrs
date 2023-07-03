use vecrs::vectors::{Vec2, Vec3, Vec4};

// Test vector dot product
//
// Test Vec2 dot product
#[test]
fn vec2_dot() {
    let a = Vec2::new(4., 6.);
    let b = Vec2::new(2., 3.);

    let actual = 26f64;

    // Check the result
    assert_eq!(actual, Vec2::dot(a, b));
}

// Test Vec3 dot product
#[test]
fn vec3_dot() {
    let a = Vec3::new(4., 6., 8.);
    let b = Vec3::new(2., 3., 4.);

    let actual = 58f64;

    // Check the result
    assert_eq!(actual, Vec3::dot(a, b));
}

// Test Vec4 dot product
#[test]
fn vec4_dot() {
    let a = Vec4::new(4., 6., 8., 10.);
    let b = Vec4::new(2., 3., 4., 5.);

    let actual = 108f64;

    // Check the result
    assert_eq!(actual, Vec4::dot(a, b));
}

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

// Test vector by vector operators
//
// Test Vec2 assign perators
#[test]
fn vec2_assign_ops() {
    let rhs = Vec2::new(2., 3.);

    let mut a = Vec2::new(4., 6.);
    let mut b = Vec2::new(4., 6.);
    let mut c = Vec2::new(4., 6.);
    let mut d = Vec2::new(4., 6.);

    // Use operators
    a += rhs;
    b -= rhs;
    c *= rhs;
    d /= rhs;

    // Test operators
    assert_eq!(Vec2::new(6., 9.), a, "add ops");
    assert_eq!(Vec2::new(2., 3.), b, "sub ops");
    assert_eq!(Vec2::new(8., 18.), c, "mul ops");
    assert_eq!(Vec2::new(2., 2.), d, "div ops");
}

// Test Vec3 assign operators
#[test]
fn vec3_assign_ops() {
    let rhs = Vec3::new(2., 3., 4.);

    let mut a = Vec3::new(4., 6., 8.);
    let mut b = Vec3::new(4., 6., 8.);
    let mut c = Vec3::new(4., 6., 8.);
    let mut d = Vec3::new(4., 6., 8.);

    // Use operators
    a += rhs;
    b -= rhs;
    c *= rhs;
    d /= rhs;

    // Test operators
    assert_eq!(Vec3::new(6., 9., 12.), a, "add ops");
    assert_eq!(Vec3::new(2., 3., 4.), b, "sub ops");
    assert_eq!(Vec3::new(8., 18., 32.), c, "mul ops");
    assert_eq!(Vec3::new(2., 2., 2.), d, "div ops");
}

// Test Vec4 assign operators
#[test]
fn vec4_assign_ops() {
    let rhs = Vec4::new(2., 3., 4., 5.);

    let mut a = Vec4::new(4., 6., 8., 10.);
    let mut b = Vec4::new(4., 6., 8., 10.);
    let mut c = Vec4::new(4., 6., 8., 10.);
    let mut d = Vec4::new(4., 6., 8., 10.);
    
    // Use operators
    a += rhs;
    b -= rhs;
    c *= rhs;
    d /= rhs;

    // Test operators
    assert_eq!(Vec4::new(6., 9., 12., 15.), a, "add ops");
    assert_eq!(Vec4::new(2., 3., 4., 5.), b, "sub ops");
    assert_eq!(Vec4::new(8., 18., 32., 50.), c, "mul ops");
    assert_eq!(Vec4::new(2., 2., 2., 2.), d, "div ops");
}
