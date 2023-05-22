use vecrs::matrices::{Mat2x2, Mat3x3, Mat4x4};

// Test transpose function
//
// Test transpose for 2x2 matrix
#[test]
fn mat2x2_transpose() {
    let input = Mat2x2::new([
        [1., 2.],
        [3., 4.],
    ]);
    let transposed = Mat2x2::new([
        [1., 3.],
        [2., 4.],
    ]);

    // Test transpose function
    assert_eq!(input.transpose(), transposed);
}

// Test transpose for 3x3 matrix
#[test]
fn mat3x3_transpose() {
    let input = Mat3x3::new([
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.],
    ]);
    let transposed = Mat3x3::new([
        [1., 4., 7.],
        [2., 5., 8.],
        [3., 6., 9.],
    ]);

    // Test transpose function
    assert_eq!(input.transpose(), transposed);
}

// Test transpose for 4x4 matrix
#[test]
fn mat4x4_transpose() {
    let input = Mat4x4::new([
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 10., 11., 12.],
        [13., 14., 15., 16.],
    ]);
    let transposed = Mat4x4::new([
        [1., 5., 9., 13.],
        [2., 6., 10., 14.],
        [3., 7., 11., 15.],
        [4., 8., 12., 16.],
    ]);

    // Test transpose function
    assert_eq!(input.transpose(), transposed);
}

// Test dot product function
//
// Test 2x2 dot product
#[test]
fn mat2x2_dot() {
    let a = Mat2x2::new([
        [1., 2.],
        [3., 4.],
    ]);
    let b = Mat2x2::new([
        [2., 3.],
        [4., 5.],
    ]);
    
    // The correct result
    let result = Mat2x2::new([
        [10., 13.],
        [22., 29.],
    ]);
    
    // check the dot function
    assert_eq!(Mat2x2::dot(&a, &b), result, "Mat2x2: dot test");

    // ouput matrix for the dot_ref operation
    let mut output = Mat2x2::default();
    Mat2x2::dot_ref(&a, &b, &mut output);
    assert_eq!(output, result, "Mat2x2: dot_ref test");
}

// Test 3x3 dot product
#[test]
fn mate3x3_dot() {
    let a = Mat3x3::new([
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.],
    ]);
    let b = Mat3x3::new([
        [2., 3., 4.],
        [5., 6., 7.],
        [8., 9., 10.],
    ]);
    
    // The correct result
    let result = Mat3x3::new([
        [36., 42., 48.],
        [81., 96., 111.],
        [126., 150., 174.],
    ]);
    
    // check the dot function
    assert_eq!(Mat3x3::dot(&a, &b), result, "Mat3x3: dot test");

    // ouput matrix for the dot_ref operation
    let mut output = Mat3x3::default();
    Mat3x3::dot_ref(&a, &b, &mut output);
    assert_eq!(output, result, "Mat3x3: dot_ref test");
}

// Test 4x4 dot product
#[test]
fn mate4x4_dot() {
    let a = Mat4x4::new([
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 10., 11., 12.],
        [13., 14., 15., 16.],
    ]);
    let b = Mat4x4::new([
        [2., 3., 4., 5.],
        [6., 7., 8., 9.],
        [10., 11., 12., 13.],
        [14., 15., 16., 17.],
    ]);
    
    // The correct result
    let result = Mat4x4::new([
        [100., 110., 120., 130.],
        [228., 254., 280., 306.],
        [356., 398., 440., 482.],
        [484., 542., 600., 658.],
    ]);
    
    // check the dot function
    assert_eq!(Mat4x4::dot(&a, &b), result, "Mat4x4: dot test");

    // ouput matrix for the dot_ref operation
    let mut output = Mat4x4::default();
    Mat4x4::dot_ref(&a, &b, &mut output);
    assert_eq!(output, result, "Mat4x4: dot_ref test");
}
