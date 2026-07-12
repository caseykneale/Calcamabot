use crate::parse_and_eval;
use crate::value::Value;

use crate::tests::common::approx_eq;

// ── conj function ────────────────────────────────────────────────────

#[test]
fn test_conj_complex() {
    let result = parse_and_eval("conj(3 + 4i)").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(approx_eq(c.re, 3.0, 1e-9));
            assert!(approx_eq(c.im, -4.0, 1e-9));
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_conj_real_scalar() {
    let result = parse_and_eval("conj(5)").unwrap();
    match result {
        Value::Scalar(n) => assert!(approx_eq(n, 5.0, 1e-9)),
        other => panic!("Expected Scalar, got {:?}", other),
    }
}

#[test]
fn test_conj_pure_imaginary() {
    let result = parse_and_eval("conj(3i)").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(approx_eq(c.re, 0.0, 1e-9));
            assert!(approx_eq(c.im, -3.0, 1e-9));
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_conj_negative_imaginary() {
    let result = parse_and_eval("conj(2 - 5i)").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(approx_eq(c.re, 2.0, 1e-9));
            assert!(approx_eq(c.im, 5.0, 1e-9));
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_conj_broadcast_on_complex_matrix() {
    let result = parse_and_eval("conj.([[1+2i, 3-4i]])").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            // Input is 1 col of 2 rows → conj. preserves shape
            assert_eq!(m.len(), 1); // 1 column
            assert_eq!(m[0].len(), 2); // 2 rows
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, 1.0, 1e-9));
            assert!(approx_eq(c0.im, -2.0, 1e-9));
            let c1 = m[0][1];
            assert!(approx_eq(c1.re, 3.0, 1e-9));
            assert!(approx_eq(c1.im, 4.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

// ── Conjugate transpose (") ──────────────────────────────────────────

#[test]
fn test_conjugate_transpose_complex_matrix() {
    // [[1+2i, 3-4i]] is 2×1 (2 rows, 1 col in column-major: 1 col of length 2)
    // Conjugate transpose should be 1×2: [[1-2i, 3+4i]]
    // In column-major: 2 cols of length 1: [1-2i], [3+4i]
    let result = parse_and_eval("[[1+2i, 3-4i]]\"").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 2); // 2 columns
            assert_eq!(m[0].len(), 1); // 1 row
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, 1.0, 1e-9));
            assert!(approx_eq(c0.im, -2.0, 1e-9));
            let c1 = m[1][0];
            assert!(approx_eq(c1.re, 3.0, 1e-9));
            assert!(approx_eq(c1.im, 4.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_conjugate_transpose_real_matrix_is_transpose() {
    // [[1, 2], [3, 4]]" should equal [[1, 3], [2, 4]] (regular transpose)
    let result = parse_and_eval("[[1, 2], [3, 4]]\"").unwrap();
    match result {
        Value::Matrix(m) => {
            assert_eq!(m.len(), 2);
            assert_eq!(m[0].len(), 2);
            // Column-major: col0 = [1, 3], col1 = [2, 4]
            assert!(approx_eq(m[0][0], 1.0, 1e-9));
            assert!(approx_eq(m[0][1], 3.0, 1e-9));
            assert!(approx_eq(m[1][0], 2.0, 1e-9));
            assert!(approx_eq(m[1][1], 4.0, 1e-9));
        }
        other => panic!("Expected Matrix, got {:?}", other),
    }
}

#[test]
fn test_conjugate_transpose_vector() {
    // [1, 2, 3] is 3×1 column vector
    // [1, 2, 3]" should be 1×3 row with same values (real, so conjugate is identity)
    let result = parse_and_eval("[1, 2, 3]\"").unwrap();
    match result {
        Value::Matrix(m) => {
            assert_eq!(m.len(), 3); // 3 columns
            assert_eq!(m[0].len(), 1); // 1 row
            assert!(approx_eq(m[0][0], 1.0, 1e-9));
            assert!(approx_eq(m[1][0], 2.0, 1e-9));
            assert!(approx_eq(m[2][0], 3.0, 1e-9));
        }
        other => panic!("Expected Matrix, got {:?}", other),
    }
}

#[test]
fn test_conjugate_transpose_complex_vector() {
    // [1+2i, 3-4i] is 2×1 column
    // [1+2i, 3-4i]" should be 1×2 row: [1-2i, 3+4i]
    let result = parse_and_eval("[1+2i, 3-4i]\"").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 2); // 2 columns
            assert_eq!(m[0].len(), 1); // 1 row
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, 1.0, 1e-9));
            assert!(approx_eq(c0.im, -2.0, 1e-9));
            let c1 = m[1][0];
            assert!(approx_eq(c1.re, 3.0, 1e-9));
            assert!(approx_eq(c1.im, 4.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_conjugate_transpose_then_multiply() {
    // [1+2i, 3-4i]" * [1+2i, 3-4i] = |1+2i|² + |3-4i|² = 5 + 25 = 30
    let result = parse_and_eval("[1+2i, 3-4i]\" * [1+2i, 3-4i]").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            // Result is 1×1
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 1);
            let val = m[0][0];
            assert!(approx_eq(val.re, 30.0, 1e-9));
            assert!(approx_eq(val.im, 0.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_conjugate_transpose_idempotent() {
    // (A")' = A* (conjugate) for complex matrices
    let result = parse_and_eval("[[1+2i, 3-4i], [5, 6+7i]]\"'").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            // Should equal conjugate of original: 2 columns, col0 has 2 elements, col1 has 2 elements
            assert_eq!(m.len(), 2);
            assert_eq!(m[0].len(), 2);
            assert_eq!(m[1].len(), 2);
            // col0 = [1-2i, 5], col1 = [3+4i, 6-7i]
            assert!(approx_eq(m[0][0].re, 1.0, 1e-9));
            assert!(approx_eq(m[0][0].im, -2.0, 1e-9));
            assert!(approx_eq(m[1][0].re, 5.0, 1e-9));
            assert!(approx_eq(m[1][0].im, 0.0, 1e-9));
            assert!(approx_eq(m[0][1].re, 3.0, 1e-9));
            assert!(approx_eq(m[0][1].im, 4.0, 1e-9));
            assert!(approx_eq(m[1][1].re, 6.0, 1e-9));
            assert!(approx_eq(m[1][1].im, -7.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_conjugate_transpose_display() {
    let result = parse_and_eval("[[1+2i, 3-4i]]\"").unwrap();
    let s = format!("{}", result);
    // Should display as a 2×1 complex matrix
    assert!(s.contains("1 - 2i") || s.contains("1-2i"), "Display should contain conjugate: {}", s);
    assert!(s.contains("3 + 4i") || s.contains("3+4i"), "Display should contain conjugate: {}", s);
}

// ── Complex matrix arithmetic ────────────────────────────────────────

#[test]
fn test_complex_matrix_add() {
    let result = parse_and_eval("[[1+2i, 3-4i]] + [[5, 6+7i]]").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            // [[1+2i, 3-4i]] is 1 row, 2 cols → stored as 1 col of 2 elems (2×1 matrix)
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 2);
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, 6.0, 1e-9));
            assert!(approx_eq(c0.im, 2.0, 1e-9));
            let c1 = m[0][1];
            assert!(approx_eq(c1.re, 9.0, 1e-9));
            assert!(approx_eq(c1.im, 3.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_scalar_add() {
    let result = parse_and_eval("1 + [1+2i, 3-4i]").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            // [1+2i, 3-4i] is 1 row, 2 cols → stored as 1 col of 2 elems
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 2);
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, 2.0, 1e-9));
            assert!(approx_eq(c0.im, 2.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_mul_scalar() {
    let result = parse_and_eval("[[1+2i, 3-4i]] * 2").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 2);
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, 2.0, 1e-9));
            assert!(approx_eq(c0.im, 4.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_multiply() {
    // [[1+2i]] (1×1) * [[3-4i]] (1×1) = [(1+2i)(3-4i)] = [3 - 4i + 6i - 8i²] = [3 + 2i + 8] = [11 + 2i]
    let result = parse_and_eval("[[1+2i]] * [[3-4i]]").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 1);
            let val = m[0][0];
            assert!(approx_eq(val.re, 11.0, 1e-9));
            assert!(approx_eq(val.im, 2.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_elementwise_mul() {
    let result = parse_and_eval("[[1+2i, 3-4i]] .* [[5, 6+7i]]").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 2);
            // (1+2i)*5 = 5+10i
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, 5.0, 1e-9));
            assert!(approx_eq(c0.im, 10.0, 1e-9));
            // (3-4i)*(6+7i) = 18 + 21i - 24i - 28i² = 18 - 3i + 28 = 46 - 3i
            let c1 = m[0][1];
            assert!(approx_eq(c1.re, 46.0, 1e-9));
            assert!(approx_eq(c1.im, -3.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_sub() {
    let result = parse_and_eval("[[1+2i, 3-4i]] - [[5, 6+7i]]").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 2);
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, -4.0, 1e-9));
            assert!(approx_eq(c0.im, 2.0, 1e-9));
            let c1 = m[0][1];
            assert!(approx_eq(c1.re, -3.0, 1e-9));
            assert!(approx_eq(c1.im, -11.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_div_scalar() {
    let result = parse_and_eval("[[2+4i, 6-8i]] / 2").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 2);
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, 1.0, 1e-9));
            assert!(approx_eq(c0.im, 2.0, 1e-9));
            let c1 = m[0][1];
            assert!(approx_eq(c1.re, 3.0, 1e-9));
            assert!(approx_eq(c1.im, -4.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_unary_minus() {
    let result = parse_and_eval("-[[1+2i, 3-4i]]").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 2);
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, -1.0, 1e-9));
            assert!(approx_eq(c0.im, -2.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_pow_scalar() {
    // [[1+2i]]^2 = (1+2i)² = 1 + 4i + 4i² = 1 + 4i - 4 = -3 + 4i
    let result = parse_and_eval("[[1+2i]] ^ 2").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 1);
            let val = m[0][0];
            assert!(approx_eq(val.re, -3.0, 1e-9));
            assert!(approx_eq(val.im, 4.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_fnorm() {
    // [[1+2i, 3-4i]]: |1+2i|² = 5, |3-4i|² = 25, fnorm = sqrt(30)
    let result = parse_and_eval("fnorm([[1+2i, 3-4i]])").unwrap();
    match result {
        Value::Scalar(n) => {
            assert!(approx_eq(n, 30.0_f64.sqrt(), 1e-9));
        }
        other => panic!("Expected Scalar, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_trace() {
    // [[1+2i, 0], [0, 3-4i]] trace = (1+2i) + (3-4i) = 4 - 2i, real part = 4
    let result = parse_and_eval("tr([[1+2i, 0], [0, 3-4i]])").unwrap();
    match result {
        Value::Scalar(n) => {
            assert!(approx_eq(n, 4.0, 1e-9));
        }
        other => panic!("Expected Scalar, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_normalize() {
    // [[3+4i]] has norm |3+4i| = 5, normalized = (3+4i)/5 = 0.6+0.8i
    let result = parse_and_eval("normalize([[3+4i]])").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 1);
            let val = m[0][0];
            assert!(approx_eq(val.re, 0.6, 1e-9));
            assert!(approx_eq(val.im, 0.8, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_fnormalize() {
    // [[3+4i]] has fnorm = 5, fnormalized = (3+4i)/5 = 0.6+0.8i
    let result = parse_and_eval("fnormalize([[3+4i]])").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 1);
            let val = m[0][0];
            assert!(approx_eq(val.re, 0.6, 1e-9));
            assert!(approx_eq(val.im, 0.8, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_norm() {
    // [[3+4i, 0]] is 1 col of 2 rows → norm returns [5.0] (1 column norm)
    let result = parse_and_eval("norm([[3+4i, 0]])").unwrap();
    match result {
        Value::Matrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 1);
            assert!(approx_eq(m[0][0], 5.0, 1e-9));
        }
        other => panic!("Expected Matrix, got {:?}", other),
    }
}

#[test]
fn test_conj_then_multiply_returns_real() {
    // [1+2i, 3-4i]" * [1+2i, 3-4i] = |1+2i|² + |3-4i|² = 5 + 25 = 30
    let result = parse_and_eval("[1+2i, 3-4i]\" * [1+2i, 3-4i]").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 1);
            let val = m[0][0];
            assert!(approx_eq(val.re, 30.0, 1e-9));
            assert!(approx_eq(val.im, 0.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_matrix_display() {
    let result = parse_and_eval("[[1+2i, 3-4i]]").unwrap();
    let s = format!("{}", result);
    assert!(s.contains("1 + 2i") || s.contains("1+2i"), "Display should contain 1+2i: {}", s);
    assert!(s.contains("3 - 4i") || s.contains("3-4i"), "Display should contain 3-4i: {}", s);
}

#[test]
fn test_complex_matrix_negate() {
    let result = parse_and_eval("-[1+2i, 3-4i]").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 2);
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, -1.0, 1e-9));
            assert!(approx_eq(c0.im, -2.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

// ── eye() with complex ─────────────────────────────────────────────

#[test]
fn test_eye_complex_vector() {
    // eye(2) with complex context should still be real identity
    let result = parse_and_eval("eye(2)").unwrap();
    match result {
        Value::Matrix(m) => {
            assert_eq!(m.len(), 2);
            assert_eq!(m[0].len(), 2);
            assert!(approx_eq(m[0][0], 1.0, 1e-9));
            assert!(approx_eq(m[0][1], 0.0, 1e-9));
            assert!(approx_eq(m[1][0], 0.0, 1e-9));
            assert!(approx_eq(m[1][1], 1.0, 1e-9));
        }
        other => panic!("Expected Matrix, got {:?}", other),
    }
}

// ── diag() with complex ────────────────────────────────────────────

#[test]
fn test_diag_complex_vector() {
    // diag([1+2i, 3-4i]) should create a 2x2 complex diagonal matrix
    // In column-major: col0=[1+2i, 0], col1=[0, 3-4i]
    let result = parse_and_eval("diag([1+2i, 3-4i])").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 2); // 2 columns
            assert_eq!(m[0].len(), 2); // 2 rows
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, 1.0, 1e-9));
            assert!(approx_eq(c0.im, 2.0, 1e-9));
            let c1 = m[0][1];
            assert!(approx_eq(c1.re, 0.0, 1e-9));
            assert!(approx_eq(c1.im, 0.0, 1e-9));
            let c2 = m[1][0];
            assert!(approx_eq(c2.re, 0.0, 1e-9));
            assert!(approx_eq(c2.im, 0.0, 1e-9));
            let c3 = m[1][1];
            assert!(approx_eq(c3.re, 3.0, 1e-9));
            assert!(approx_eq(c3.im, -4.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_diag_complex_matrix_extract() {
    // diag([[1+2i, 0], [0, 3-4i]]) should extract diagonal [1+2i, 3-4i]
    // In column-major: 1 col of 2 complex elements
    let result = parse_and_eval("diag([[1+2i, 0], [0, 3-4i]])").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1); // 1 column (vector)
            assert_eq!(m[0].len(), 2); // 2 rows
            let c0 = m[0][0];
            assert!(approx_eq(c0.re, 1.0, 1e-9));
            assert!(approx_eq(c0.im, 2.0, 1e-9));
            let c1 = m[0][1];
            assert!(approx_eq(c1.re, 3.0, 1e-9));
            assert!(approx_eq(c1.im, -4.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_diag_complex_scalar() {
    // diag(2+3i) should return a 1x1 complex matrix
    let result = parse_and_eval("diag(2+3i)").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].len(), 1);
            let val = m[0][0];
            assert!(approx_eq(val.re, 2.0, 1e-9));
            assert!(approx_eq(val.im, 3.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

// ── det() with complex matrices ────────────────────────────────────

#[test]
fn test_complex_det_1x1() {
    let result = parse_and_eval("det([[3+4i]])").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(approx_eq(c.re, 3.0, 1e-9));
            assert!(approx_eq(c.im, 4.0, 1e-9));
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_complex_det_2x2() {
    // det([[1+2i, 3-4i], [5, 6+7i]])
    // = (1+2i)(6+7i) - (3-4i)(5)
    // = (6 + 7i + 12i + 14i²) - (15 - 20i)
    // = (6 - 14 + 19i) - (15 - 20i)
    // = (-8 + 19i) - (15 - 20i)
    // = -23 + 39i
    let result = parse_and_eval("det([[1+2i, 3-4i], [5, 6+7i]])").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(approx_eq(c.re, -23.0, 1e-9));
            assert!(approx_eq(c.im, 39.0, 1e-9));
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_complex_det_identity() {
    // Use an explicitly complex identity matrix so det goes through the complex path
    let result = parse_and_eval("det([[1+0i, 0], [0, 1+0i]])").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(approx_eq(c.re, 1.0, 1e-9));
            assert!(approx_eq(c.im, 0.0, 1e-9));
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_complex_det_singular() {
    // [[1+i, 2+2i], [1, 2]] — col2 = (2+2i) * col1, so det = (1+i)*2 - (2+2i)*1 = 0
    let result = parse_and_eval("det([[1+i, 2+2i], [1, 2]])").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(approx_eq(c.re, 0.0, 1e-9));
            assert!(approx_eq(c.im, 0.0, 1e-9));
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_complex_det_diagonal() {
    // det(diag(1+2i, 3-4i)) = (1+2i)(3-4i) = 3 - 4i + 6i - 8i² = 11 + 2i
    let result = parse_and_eval("det([[1+2i, 0], [0, 3-4i]])").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(approx_eq(c.re, 11.0, 1e-9));
            assert!(approx_eq(c.im, 2.0, 1e-9));
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

#[test]
fn test_complex_det_in_expression() {
    // det([[1+2i, 0], [0, 3-4i]]) + 1 = (11+2i) + 1 = 12+2i
    let result = parse_and_eval("det([[1+2i, 0], [0, 3-4i]]) + 1").unwrap();
    match result {
        Value::Complex(c) => {
            assert!(approx_eq(c.re, 12.0, 1e-9));
            assert!(approx_eq(c.im, 2.0, 1e-9));
        }
        other => panic!("Expected Complex, got {:?}", other),
    }
}

// ── inv() with complex matrices ────────────────────────────────────

#[test]
fn test_complex_inv_2x2_diagonal() {
    // inv(diag(1+2i, 3-4i)) = diag(1/(1+2i), 1/(3-4i))
    // 1/(1+2i) = (1-2i)/5 = 0.2 - 0.4i
    // 1/(3-4i) = (3+4i)/25 = 0.12 + 0.16i
    let result = parse_and_eval("inv([[1+2i, 0], [0, 3-4i]])").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 2); // 2 columns
            assert_eq!(m[0].len(), 2); // 2 rows
            // col0 = [1+2i, 0] → inv col0 = [1/(1+2i), 0]
            let c00 = m[0][0];
            assert!(approx_eq(c00.re, 0.2, 1e-9));
            assert!(approx_eq(c00.im, -0.4, 1e-9));
            let c01 = m[0][1];
            assert!(approx_eq(c01.re, 0.0, 1e-9));
            assert!(approx_eq(c01.im, 0.0, 1e-9));
            // col1 = [0, 3-4i] → inv col1 = [0, 1/(3-4i)]
            let c10 = m[1][0];
            assert!(approx_eq(c10.re, 0.0, 1e-9));
            assert!(approx_eq(c10.im, 0.0, 1e-9));
            let c11 = m[1][1];
            assert!(approx_eq(c11.re, 0.12, 1e-9));
            assert!(approx_eq(c11.im, 0.16, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_inv_identity() {
    // Use an explicitly complex identity matrix so inv goes through the complex path
    let result = parse_and_eval("inv([[1+0i, 0], [0, 1+0i]])").unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 2);
            assert_eq!(m[0].len(), 2);
            assert!(approx_eq(m[0][0].re, 1.0, 1e-9));
            assert!(approx_eq(m[0][0].im, 0.0, 1e-9));
            assert!(approx_eq(m[0][1].re, 0.0, 1e-9));
            assert!(approx_eq(m[0][1].im, 0.0, 1e-9));
            assert!(approx_eq(m[1][0].re, 0.0, 1e-9));
            assert!(approx_eq(m[1][0].im, 0.0, 1e-9));
            assert!(approx_eq(m[1][1].re, 1.0, 1e-9));
            assert!(approx_eq(m[1][1].im, 0.0, 1e-9));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_inv_roundtrip() {
    // A * inv(A) should equal identity for a non-trivial complex matrix
    let a = parse_and_eval("[[1+2i, 3-4i], [5, 6+7i]]").unwrap();
    let ai = parse_and_eval("inv([[1+2i, 3-4i], [5, 6+7i]])").unwrap();
    let result = parse_and_eval(&format!("{} * {}", a, ai)).unwrap();
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 2);
            assert_eq!(m[0].len(), 2);
            // Should be identity: col0=[1,0], col1=[0,1]
            assert!(approx_eq(m[0][0].re, 1.0, 1e-6));
            assert!(approx_eq(m[0][0].im, 0.0, 1e-6));
            assert!(approx_eq(m[0][1].re, 0.0, 1e-6));
            assert!(approx_eq(m[0][1].im, 0.0, 1e-6));
            assert!(approx_eq(m[1][0].re, 0.0, 1e-6));
            assert!(approx_eq(m[1][0].im, 0.0, 1e-6));
            assert!(approx_eq(m[1][1].re, 1.0, 1e-6));
            assert!(approx_eq(m[1][1].im, 0.0, 1e-6));
        }
        other => panic!("Expected ComplexMatrix, got {:?}", other),
    }
}

#[test]
fn test_complex_inv_singular_error() {
    // Singular complex matrix should error
    assert!(parse_and_eval("inv([[1+i, 2+2i], [1, 2]])").is_err());
}

// ── Pretty print display tests for complex matrices ────────────────

#[test]
fn test_pretty_print_complex_matrix() {
    let result = parse_and_eval("[[1+2i, 3-4i]]").unwrap();
    let s = format!("{result:#}");
    // Should display as a complex matrix with correct dimensions and ordering
    assert!(s.contains("complex matrix"), "Display should contain 'complex matrix': {}", s);
    // The matrix [[1+2i, 3-4i]] in column-major is col0=[1+2i, 3-4i], so it's 2x1
    // After fix, the display should show the transpose:
    // 1 + 2i
    // 3 - 4i
    let lines: Vec<&str> = s.lines().collect();
    // Skip the empty line and header line (first two lines are empty and dimensions)
    let data_lines: Vec<&str> = lines.into_iter().skip(2).filter(|l| !l.is_empty()).collect();
    // Should have 2 data lines (2 rows) for a 2x1 matrix
    assert_eq!(data_lines.len(), 2, "Expected 2 data lines for 2x1 matrix, got {}: {}", data_lines.len(), s);
    // First row should contain 1+2i
    assert!(
        data_lines[0].contains("1") && data_lines[0].contains("2i"),
        "First row should contain 1+2i, got: {}",
        data_lines[0]
    );
    // Second row should contain 3-4i
    assert!(
        data_lines[1].contains("3") && data_lines[1].contains("4i"),
        "Second row should contain 3-4i, got: {}",
        data_lines[1]
    );
}
