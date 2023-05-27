use vecrs::matrices::{Mat2x2, Mat3x3, Mat4x4};

// Test the matrix inversion
//
// Test matrix inversion for 2x2 matrix
#[test]
fn mat2x2_inverse_fails() {
    // Test fail cases
    let matrix = Mat2x2::new([
        [2., 2.],
        [2., 2.],
    ]);
    assert_eq!(matrix.inverse(), None);

    let matrix = Mat2x2::new([
        [0., 3.],
        [0., 2.],
    ]);
    assert_eq!(matrix.inverse(), None);

    let matrix = Mat2x2::new([
        [0., 0.],
        [3., 2.],
    ]);
    assert_eq!(matrix.inverse(), None);

    let matrix = Mat2x2::new([
        [3., 2.],
        [0., 0.],
    ]);
    assert_eq!(matrix.inverse(), None);

    let matrix = Mat2x2::new([
        [2., 0.],
        [3., 0.],
    ]);
    assert_eq!(matrix.inverse(), None);
}

#[test]
fn mat2x2_inverse() {
    let identity = Mat2x2::identity();

    // Test cases
    let matrix = Mat2x2::new([
        [2., 5.],
        [9., 7.],
    ]);
    let test_id = Mat2x2::dot(&matrix.inverse().unwrap(), &matrix);
    assert!(Mat2x2::relative_eq(test_id, identity, 1e9));
    
    // Test cases
    let matrix = Mat2x2::new([
        [35., 0.],
        [9., 32.],
    ]);
    let test_id = Mat2x2::dot(&matrix.inverse().unwrap(), &matrix);
    assert!(Mat2x2::relative_eq(test_id, identity, 1e9));

    // Test cases
    let matrix = Mat2x2::new([
        [2., 5.],
        [9., 0.],
    ]);
    let test_id = Mat2x2::dot(&matrix.inverse().unwrap(), &matrix);
    assert!(Mat2x2::relative_eq(test_id, identity, 1e9));

    // Test cases
    let matrix = Mat2x2::new([
        [0., 4.3],
        [67., 7.],
    ]);
    let test_id = Mat2x2::dot(&matrix.inverse().unwrap(), &matrix);
    assert!(Mat2x2::relative_eq(test_id, identity, 1e9));
}

// Test the matrix inversion
//
// Test matrix inversion for 3x3 matrix#[test]
#[test]
fn mat3x3_inverse_fails() {
    // Test fail cases
    let matrix = Mat3x3::new([
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.],
    ]);
    assert_eq!(matrix.inverse(), None);

    let matrix = Mat3x3::new([
        [0.,5.,8.],
        [0.,2.,6.],
        [0.,4.,1.],
    ]);
    assert_eq!(matrix.inverse(), None);

    let matrix = Mat3x3::new([
        [4.,0.,8.],
        [4.,0.,6.],
        [9.,0.,0.],
    ]);
    assert_eq!(matrix.inverse(), None);
}

#[test]
fn mat3x3_inverse() {
    let identity = Mat3x3::identity();

    // Test cases
    let matrix = Mat3x3::new([
        [7.,5.,8.],
        [3.,2.,6.],
        [6.,4.,1.],
    ]);
    let test_id = Mat3x3::dot(&matrix.inverse().unwrap(), &matrix);
    assert!(Mat3x3::relative_eq(test_id, identity, 1e9));

    let matrix = Mat3x3::new([
        [6.,5.,32.],
        [32.,2.,6.],
        [8.,320.,3.],
    ]);
    let test_id = Mat3x3::dot(&matrix.inverse().unwrap(), &matrix);
    assert!(Mat3x3::relative_eq(test_id, identity, 1e9));
    
    let matrix = Mat3x3::new([
        [5.,5.,7.],
        [0.,19.,6.],
        [2.,4.,0.],
    ]);
    let test_id = Mat3x3::dot(&matrix.inverse().unwrap(), &matrix);
    assert!(Mat3x3::relative_eq(test_id, identity, 1e9));
    
    let matrix = Mat3x3::new([
        [6.,21.,8.],
        [3.,0.,8.],
        [12.,0.,1.],
    ]);
    let test_id = Mat3x3::dot(&matrix.inverse().unwrap(), &matrix);
    assert!(Mat3x3::relative_eq(test_id, identity, 1e9));
}

// Test the matrix inversion
//
// Test matrix inversion for 3x3 matrix#[test]
#[test]
fn mat4x4_inverse_fails() {
    // Test fail cases
    let matrix = Mat4x4::new([
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 10., 11., 12.],
        [13., 14., 15., 16.],
    ]);
    assert_eq!(matrix.inverse(), None);

    let matrix = Mat4x4::new([
        [0.,5.,8.,4.],
        [0.,2.,6.,6.],
        [0.,4.,1.,1.],
        [0.,8.,2.,5.],
    ]);
    assert_eq!(matrix.inverse(), None);
}

#[test]
fn mat4x4_inverse() {
    let identity = Mat4x4::identity();

    // Test cases
    let matrix = Mat4x4::new([
        [7.,5.,8.,4.],
        [3.,2.,6.,6.],
        [6.,4.,1.,1.],
        [3.,8.,2.,5.],
    ]);
    let test_id = Mat4x4::dot(&matrix.inverse().unwrap(), &matrix);
    assert!(Mat4x4::relative_eq(test_id, identity, 1e9));

    let matrix = Mat4x4::new([
        [7.,5.,8.,4.],
        [34.,2.,6.,2323.],
        [6.,0.,0.,1.],
        [33.,8.,2.,35.],
    ]);
    let test_id = Mat4x4::dot(&matrix.inverse().unwrap(), &matrix);
    assert!(Mat4x4::relative_eq(test_id, identity, 1e9));
}

// Test the determinant function
//
// Test 2x2 matrix determinant
#[test]
fn mat2x2_determinant() {
    let matrix = Mat2x2::new([
        [1., 2.],
        [3., 4.],
    ]);

    // Test fucntion
    assert_eq!(matrix.determinant(), -2.)
}

// Test 3x3 matrix determinant
#[test]
fn mat3x3_determinant() {
let matrix = Mat3x3::new([
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.],
    ]);

    // Test fucntion
    assert_eq!(matrix.determinant(), 0.)
}

// Test 4x4 matrix determinant
#[test]
fn mat4x4_determinant() {
let matrix = Mat4x4::new([
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 10., 11., 12.],
        [13., 14., 15., 16.],
    ]);

    // Test fucntion
    assert_eq!(matrix.determinant(), 0.)
}

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
