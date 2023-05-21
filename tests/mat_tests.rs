use vecrs::matrices::{Mat2x2, Mat3x3, Mat4x4};

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
    
    // check the dot fucntion
    assert_eq!(Mat2x2::dot(&a, &b), result);

    // ouput matrix for the dot_ref operation
    let mut output = Mat2x2::default();
    Mat2x2::dot_ref(&a, &b, &mut output);
    assert_eq!(output, result);
}

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
    
    // check the dot fucntion
    assert_eq!(Mat3x3::dot(&a, &b), result);

    // ouput matrix for the dot_ref operation
    let mut output = Mat3x3::default();
    Mat3x3::dot_ref(&a, &b, &mut output);
    assert_eq!(output, result);
}

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
    
    // check the dot fucntion
    assert_eq!(Mat4x4::dot(&a, &b), result);

    // ouput matrix for the dot_ref operation
    let mut output = Mat4x4::default();
    Mat4x4::dot_ref(&a, &b, &mut output);
    assert_eq!(output, result);
}
