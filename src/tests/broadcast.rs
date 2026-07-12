use crate::parse_and_eval;

use crate::tests::common::{expect_matrix, expect_vector};

#[test]
fn test_complex_broadcast_expression() {
    expect_vector(
        &parse_and_eval("sqrt.([1,4,9]) .+ [10,20,30]").unwrap(),
        &[11.0, 22.0, 33.0],
        1e-9,
    );
}

// Scalar modulo with vectors

#[test]
fn test_modulo_vector() {
    expect_vector(
        &parse_and_eval("[10, 11, 12] % 3").unwrap(),
        &[1.0, 2.0, 0.0],
        1e-9,
    );
}

#[test]
fn test_modulo_scalar_vector() {
    expect_vector(
        &parse_and_eval("10 % [3, 4, 5]").unwrap(),
        &[1.0, 2.0, 0.0],
        1e-9,
    );
}

// Broadcast tests

#[test]
fn test_broadcast_sqrt() {
    expect_vector(
        &parse_and_eval("sqrt.([4, 9, 16])").unwrap(),
        &[2.0, 3.0, 4.0],
        1e-9,
    );
}

#[test]
fn test_broadcast_abs() {
    expect_vector(
        &parse_and_eval("abs.([-1, 2, -3])").unwrap(),
        &[1.0, 2.0, 3.0],
        1e-9,
    );
}

#[test]
fn test_broadcast_floor() {
    expect_vector(
        &parse_and_eval("floor.([1.7, 2.3, -0.5])").unwrap(),
        &[1.0, 2.0, -1.0],
        1e-9,
    );
}

#[test]
fn test_broadcast_on_matrix() {
    // [[4,9],[16,25]] sqrt → [[2,3],[4,5]]
    let m: Vec<Vec<f64>> = vec![vec![2.0, 3.0], vec![4.0, 5.0]];
    expect_matrix(
        &parse_and_eval("sqrt.([[4,9],[16,25]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

// Broadcast error tests

#[test]
fn test_det_broadcast_errors() {
    assert!(parse_and_eval("det.([[1,2],[3,4]])").is_err());
}

#[test]
fn test_tr_broadcast_errors() {
    assert!(parse_and_eval("tr.([[1,2],[3,4]])").is_err());
}

#[test]
fn test_normalize_broadcast_errors() {
    assert!(parse_and_eval("normalize.([[3,4],[0,0]])").is_err());
}

#[test]
fn test_inv_broadcast_errors() {
    assert!(parse_and_eval("inv.([[1,2],[3,4]])").is_err());
}
