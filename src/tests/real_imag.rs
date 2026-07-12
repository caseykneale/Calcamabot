use crate::parse_and_eval;

use crate::tests::common::{expect_complex_vector, expect_scalar, expect_vector};
use crate::value::Value;

// real() tests

#[test]
fn test_real_complex() {
    let result = parse_and_eval("real(3 + 4i)").unwrap();
    expect_scalar(&result, 3.0, 1e-9);
}

#[test]
fn test_real_real_scalar() {
    let result = parse_and_eval("real(5)").unwrap();
    expect_scalar(&result, 5.0, 1e-9);
}

#[test]
fn test_real_zero() {
    let result = parse_and_eval("real(0)").unwrap();
    expect_scalar(&result, 0.0, 1e-9);
}

#[test]
fn test_real_negative_imaginary() {
    let result = parse_and_eval("real(-2 + 7i)").unwrap();
    expect_scalar(&result, -2.0, 1e-9);
}

#[test]
fn test_real_pure_imaginary() {
    let result = parse_and_eval("real(5i)").unwrap();
    expect_scalar(&result, 0.0, 1e-9);
}

// imag() tests

#[test]
fn test_imag_complex() {
    let result = parse_and_eval("imag(3 + 4i)").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(
                approx_eq(c.re, 0.0, 1e-9) && approx_eq(c.im, 4.0, 1e-9),
                "Expected 4i, got {}", c
            );
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_imag_real_scalar() {
    let result = parse_and_eval("imag(5)").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(
                approx_eq(c.re, 0.0, 1e-9) && approx_eq(c.im, 0.0, 1e-9),
                "Expected 0i, got {}", c
            );
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_imag_zero() {
    let result = parse_and_eval("imag(0)").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(
                approx_eq(c.re, 0.0, 1e-9) && approx_eq(c.im, 0.0, 1e-9),
                "Expected 0i, got {}", c
            );
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_imag_negative() {
    let result = parse_and_eval("imag(2 - 3i)").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(
                approx_eq(c.re, 0.0, 1e-9) && approx_eq(c.im, -3.0, 1e-9),
                "Expected -3i, got {}", c
            );
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

// Broadcasting tests

#[test]
fn test_real_broadcast_vector() {
    expect_vector(
        &parse_and_eval("real.([1+2i, 3+4i, 5+6i])").unwrap(),
        &[1.0, 3.0, 5.0],
        1e-9,
    );
}

#[test]
fn test_imag_broadcast_vector() {
    expect_complex_vector(
        &parse_and_eval("imag.([1+2i, 3+4i, 5+6i])").unwrap(),
        &[2.0, 4.0, 6.0],
        1e-9,
    );
}

#[test]
fn test_real_broadcast_real_vector() {
    expect_vector(
        &parse_and_eval("real.([1, 2, 3])").unwrap(),
        &[1.0, 2.0, 3.0],
        1e-9,
    );
}

#[test]
fn test_imag_broadcast_real_vector() {
    expect_complex_vector(
        &parse_and_eval("imag.([1, 2, 3])").unwrap(),
        &[0.0, 0.0, 0.0],
        1e-9,
    );
}

// Error tests — direct matrix call should fail

#[test]
fn test_real_on_matrix_errors() {
    assert!(parse_and_eval("real([[1,2],[3,4]])").is_err());
}

#[test]
fn test_imag_on_matrix_errors() {
    assert!(parse_and_eval("imag([[1,2],[3,4]])").is_err());
}

// Display tests

#[test]
fn test_imag_display_complex() {
    let result = parse_and_eval("imag(3 + 4i)").unwrap();
    assert_eq!(format!("{}", result), "4i");
}

#[test]
fn test_imag_display_real_input() {
    let result = parse_and_eval("imag(5)").unwrap();
    assert_eq!(format!("{}", result), "0");
}

#[test]
fn test_real_display() {
    let result = parse_and_eval("real(3 + 4i)").unwrap();
    assert_eq!(format!("{}", result), "3");
}

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}
