use crate::value::Value;

pub fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}

pub fn expect_scalar(result: &Value, expected: f64, eps: f64) {
    match result {
        Value::Scalar(n) => {
            if expected.is_infinite() {
                assert!(n.is_infinite() && (expected.is_sign_positive() == n.is_sign_positive()),
                    "Expected {}, got {}", expected, n);
            } else {
                assert!(approx_eq(*n, expected, eps), "Expected {}, got {}", expected, n);
            }
        }
        other => panic!("Expected scalar, got {:?}", other),
    }
}

pub fn expect_vector(result: &Value, expected: &[f64], eps: f64) {
    // Vectors are now 1-column matrices: Matrix(vec![vec![...]])
    match result {
        Value::Matrix(m) => {
            assert_eq!(m.len(), 1, "Expected 1-column matrix (vector), got {} columns", m.len());
            let v = &m[0];
            assert_eq!(v.len(), expected.len(), "Vector length mismatch");
            for (i, (a, b)) in v.iter().zip(expected.iter()).enumerate() {
                assert!(approx_eq(*a, *b, eps), "Element {} mismatch: expected {}, got {}", i, b, a);
            }
        }
        other => panic!("Expected vector (1-column matrix), got {:?}", other),
    }
}

pub fn expect_matrix(result: &Value, expected: &[&[f64]], eps: f64) {
    match result {
        Value::Matrix(m) => {
            assert_eq!(m.len(), expected.len(), "Matrix row count mismatch");
            for (i, (row_a, row_b)) in m.iter().zip(expected.iter()).enumerate() {
                assert_eq!(row_a.len(), row_b.len(), "Row {} length mismatch", i);
                for (j, (a, b)) in row_a.iter().zip(row_b.iter()).enumerate() {
                    assert!(approx_eq(*a, *b, eps), "Element [{},{}] mismatch: expected {}, got {}", i, j, b, a);
                }
            }
        }
        other => panic!("Expected matrix, got {:?}", other),
    }
}

pub fn expect_complex_vector(result: &Value, expected: &[f64], eps: f64) {
    // Complex vectors are 1-column ComplexMatrix: ComplexMatrix(vec![vec![...]])
    match result {
        Value::ComplexMatrix(m) => {
            assert_eq!(m.len(), 1, "Expected 1-column complex matrix (complex vector), got {} columns", m.len());
            let v = &m[0];
            assert_eq!(v.len(), expected.len(), "Complex vector length mismatch");
            for (i, (a, b)) in v.iter().zip(expected.iter()).enumerate() {
                assert!(approx_eq(a.im, *b, eps), "Element {} imaginary mismatch: expected {}, got {}", i, b, a.im);
                assert!(approx_eq(a.re, 0.0, eps), "Element {} real part should be 0, got {}", i, a.re);
            }
        }
        other => panic!("Expected complex vector (1-column complex matrix), got {:?}", other),
    }
}
