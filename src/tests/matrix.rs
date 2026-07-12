use crate::parse_and_eval;

use crate::tests::common::{expect_matrix, expect_scalar, expect_vector};

// Vectors, Matrices, Transpose, etc

#[test]
fn test_vector_literal() {
    expect_vector(
        &parse_and_eval("[1, 2, 3]").unwrap(),
        &[1.0, 2.0, 3.0],
        1e-9,
    );
}

#[test]
fn test_vector_empty() {
    expect_vector(&parse_and_eval("[]").unwrap(), &[], 1e-9);
}

#[test]
fn test_matrix_literal() {
    // [[1,2],[3,4]] should equal [[1,2],[3,4]]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    expect_matrix(
        &parse_and_eval("[[1, 2], [3, 4]]").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_matrix_3x3() {
    // [[1,2,3],[4,5,6],[7,8,9]] should equal [[1,2,3],[4,5,6],[7,8,9]]
    let m: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    expect_matrix(
        &parse_and_eval("[[1, 2, 3], [4, 5, 6], [7, 8, 9]]").unwrap(),
        &[&m[0], &m[1], &m[2]],
        1e-9,
    );
}

#[test]
fn test_vector_addition() {
    expect_vector(
        &parse_and_eval("[1, 2, 3] + [4, 5, 6]").unwrap(),
        &[5.0, 7.0, 9.0],
        1e-9,
    );
}

#[test]
fn test_vector_subtraction() {
    expect_vector(
        &parse_and_eval("[10, 20, 30] - [1, 2, 3]").unwrap(),
        &[9.0, 18.0, 27.0],
        1e-9,
    );
}

#[test]
fn test_scalar_vector_add() {
    expect_vector(
        &parse_and_eval("2 + [1, 2, 3]").unwrap(),
        &[3.0, 4.0, 5.0],
        1e-9,
    );
}

#[test]
fn test_vector_scalar_add() {
    expect_vector(
        &parse_and_eval("[1, 2, 3] + 2").unwrap(),
        &[3.0, 4.0, 5.0],
        1e-9,
    );
}

#[test]
fn test_scalar_matrix_add() {
    // [[1,2],[3,4]] + 1 → [[2,3],[4,5]]
    let m: Vec<Vec<f64>> = vec![vec![2.0, 3.0], vec![4.0, 5.0]];
    expect_matrix(
        &parse_and_eval("1 + [[1, 2], [3, 4]]").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_shape_mismatch_error() {
    assert!(parse_and_eval("[1, 2] + [1, 2, 3]").is_err());
}

#[test]
fn test_vector_multiplication_scalar() {
    expect_vector(
        &parse_and_eval("2 * [1, 2, 3]").unwrap(),
        &[2.0, 4.0, 6.0],
        1e-9,
    );
}

#[test]
fn test_vector_scalar_multiplication() {
    expect_vector(
        &parse_and_eval("[1, 2, 3] * 2").unwrap(),
        &[2.0, 4.0, 6.0],
        1e-9,
    );
}

#[test]
fn test_matrix_multiplication() {
    // [[1,3],[2,4]] (col0=[1,3],col1=[2,4]) * [[5,7],[6,8]] (col0=[5,7],col1=[6,8])
    // Standard: [[1,2],[3,4]] * [[5,6],[7,8]] = [[19,22],[43,50]]
    // Column-major result: col0=[19,43], col1=[22,50] → [[19,43],[22,50]]
    let m: Vec<Vec<f64>> = vec![vec![19.0, 43.0], vec![22.0, 50.0]];
    expect_matrix(
        &parse_and_eval("[[1,3],[2,4]] * [[5,7],[6,8]]").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_matrix_multiplication_dimension_error() {
    // [[1,2],[3,4],[5,6]] is 2x3 (3 cols of len 2)
    // [[1,2],[3,4]] is 2x2 (2 cols of len 2)
    // 2x3 * 2x2 → inner dims 3≠2 → error
    assert!(parse_and_eval("[[1,2],[3,4],[5,6]] * [[1,2],[3,4]]").is_err());
}

#[test]
fn test_matrix_multiplication_2x2_by_2x3() {
    // [[1,2],[3,4]] = col0=[1,2], col1=[3,4] → 2x2 matrix [[1,3],[2,4]]
    // [[1,4],[2,5],[3,6]] = col0=[1,4], col1=[2,5], col2=[3,6] → 2x3 matrix [[1,2,3],[4,5,6]]
    // Result: [[13,18],[17,24],[21,30]] (column-major)
    let m: Vec<Vec<f64>> = vec![vec![13.0, 18.0], vec![17.0, 24.0], vec![21.0, 30.0]];
    expect_matrix(
        &parse_and_eval("[[1,2],[3,4]] * [[1,4],[2,5],[3,6]]").unwrap(),
        &[&m[0], &m[1], &m[2]],
        1e-9,
    );
}

#[test]
fn test_elementwise_power() {
    expect_vector(
        &parse_and_eval("[1,2,3] .^ 2").unwrap(),
        &[1.0, 4.0, 9.0],
        1e-9,
    );
}

#[test]
fn test_elementwise_power_scalar_base() {
    expect_vector(
        &parse_and_eval("2 .^ [1,2,3]").unwrap(),
        &[2.0, 4.0, 8.0],
        1e-9,
    );
}

#[test]
fn test_elementwise_power_matrix() {
    // [[1,2],[3,4]] .^ 2 → [[1,4],[9,16]]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 4.0], vec![9.0, 16.0]];
    expect_matrix(
        &parse_and_eval("[[1,2],[3,4]] .^ 2").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_elementwise_add() {
    expect_vector(
        &parse_and_eval("[1,2,3] .+ [4,5,6]").unwrap(),
        &[5.0, 7.0, 9.0],
        1e-9,
    );
}

#[test]
fn test_elementwise_mul() {
    expect_vector(
        &parse_and_eval("[1,2,3] .* [4,5,6]").unwrap(),
        &[4.0, 10.0, 18.0],
        1e-9,
    );
}

#[test]
fn test_elementwise_div() {
    expect_vector(
        &parse_and_eval("[10,20,30] ./ [2,4,5]").unwrap(),
        &[5.0, 5.0, 6.0],
        1e-9,
    );
}

#[test]
fn test_elementwise_sub() {
    expect_vector(
        &parse_and_eval("[10,20,30] .- [1,2,3]").unwrap(),
        &[9.0, 18.0, 27.0],
        1e-9,
    );
}

#[test]
fn test_transpose_vector() {
    // [1,2,3] is 3x1 column, transpose is 1x3 row = 3 columns of length 1
    let m: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0], vec![3.0]];
    expect_matrix(
        &parse_and_eval("[1,2,3]'").unwrap(),
        &[&m[0], &m[1], &m[2]],
        1e-9,
    );
}

#[test]
fn test_transpose_matrix() {
    // [[1,2],[3,4]]' → [[1,3],[2,4]]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 3.0], vec![2.0, 4.0]];
    expect_matrix(
        &parse_and_eval("[[1,2],[3,4]]'").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_transpose_then_multiply() {
    // [1,2,3]' (1x3 matrix) * [1,2,3] (3x1 vector) = Vector([14])
    expect_vector(
        &parse_and_eval("[1,2,3]' * [1,2,3]").unwrap(),
        &[14.0],
        1e-9,
    );
}

#[test]
fn test_nested_vector_in_expression() {
    expect_vector(&parse_and_eval("[1,2] + [3,4]").unwrap(), &[4.0, 6.0], 1e-9);
}

#[test]
fn test_vector_in_parens() {
    expect_vector(
        &parse_and_eval("([1,2,3])").unwrap(),
        &[1.0, 2.0, 3.0],
        1e-9,
    );
}

#[test]
fn test_matrix_in_parens() {
    // [[1,2],[3,4]] should equal [[1,2],[3,4]]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    expect_matrix(
        &parse_and_eval("([[1,2],[3,4]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_display_vector() {
    let result = parse_and_eval("[1, 2, 3]").unwrap();
    // Vector is now a 1-column matrix: [[1, 2, 3]]
    assert_eq!(format!("{}", result), "[[1, 2, 3]]");
}

#[test]
fn test_display_matrix() {
    let result = parse_and_eval("[[1, 2], [3, 4]]").unwrap();
    // [[1,2],[3,4]] should display as [[1, 2], [3, 4]]
    assert_eq!(format!("{}", result), "[[1, 2], [3, 4]]");
}

#[test]
fn test_transpose_double() {
    // [1,2,3]' is 1x3 (3 cols of len 1), '' transposes back to 3x1 (1 col of len 3)
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0]];
    expect_matrix(&parse_and_eval("[1,2,3]''").unwrap(), &[&m[0]], 1e-9);
}

#[test]
fn test_transpose_triple() {
    // [1,2,3]''' = ([1,2,3]'')' = [1,2,3]' = 1x3 row = 3 cols of len 1
    let m: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0], vec![3.0]];
    expect_matrix(
        &parse_and_eval("[1,2,3]'''").unwrap(),
        &[&m[0], &m[1], &m[2]],
        1e-9,
    );
}

#[test]
fn test_transpose_matrix_double() {
    // [[1,2],[3,4]]'' is identity → [[1,2],[3,4]]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    expect_matrix(
        &parse_and_eval("[[1,2],[3,4]]''").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_transpose_paren_double() {
    // [[1,2],[3,4]]'' is identity → [[1,2],[3,4]]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    expect_matrix(
        &parse_and_eval("([[1,2],[3,4]])''").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_transpose_scalar_error() {
    // Transpose on scalar should still error
    assert!(parse_and_eval("5'").is_err());
}

#[test]
fn test_transpose_matrix_basic() {
    // [[1,2],[3,4]]' should equal [[1,3],[2,4]]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 3.0], vec![2.0, 4.0]];
    expect_matrix(
        &parse_and_eval("[[1,2],[3,4]]'").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_transpose_vector_to_column() {
    // [1,2,3]' is 1x3 row = 3 columns of length 1
    let m: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0], vec![3.0]];
    expect_matrix(
        &parse_and_eval("[1,2,3]'").unwrap(),
        &[&m[0], &m[1], &m[2]],
        1e-9,
    );
}

#[test]
fn test_matrix_no_transpose() {
    // [[1,2],[3,4]] should equal [[1,2],[3,4]] (no implicit transpose)
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    expect_matrix(
        &parse_and_eval("[[1,2],[3,4]]").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_transpose_then_multiply_after_fix() {
    // [1,2,3]'' (3x1) * [1,2,3]' (1x3) = 3x3 outer product
    let m: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0, 3.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 6.0, 9.0],
    ];
    expect_matrix(
        &parse_and_eval("[1,2,3]'' * [1,2,3]'").unwrap(),
        &[&m[0], &m[1], &m[2]],
        1e-9,
    );
}

#[test]
fn test_vector_matrix_outer_product() {
    // [1,2,3] (3x1) * [1,1]' (1x2) = 3x2
    // Standard: [[1,1],[2,2],[3,3]] → column-major: col0=[1,2,3], col1=[1,2,3]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]];
    expect_matrix(
        &parse_and_eval("[1,2,3] * [1,1]'").unwrap(),
        &[&m[0], &m[1]],
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

// Phase 4: Outer product and dimension mismatch tests

#[test]
fn test_outer_product_vector_matrix() {
    // [1,2,3] (3x1) * [1,1]' (1x2) = 3x2 matrix
    // Standard: [[1,1],[2,2],[3,3]] → column-major: col0=[1,2,3], col1=[1,2,3]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]];
    expect_matrix(
        &parse_and_eval("[1,2,3] * [1,1]'").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_outer_product_vector_transposed_matrix() {
    // [1,2,3] (3x1) * [1,1]' (1x2) = 3x2 matrix
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]];
    expect_matrix(
        &parse_and_eval("[1,2,3] * [1,1]'").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_outer_product_vector_transposed_vector() {
    // [1,2,3] (3x1) * [1,1]' (1x2) = 3x2 matrix
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]];
    expect_matrix(
        &parse_and_eval("[1,2,3] * [1,1]'").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_dimension_mismatch_vector_vector() {
    // [1,2,3] * [1,1] should error (both are column vectors, inner dims don't match)
    assert!(parse_and_eval("[1,2,3] * [1,1]").is_err());
}

#[test]
fn test_dimension_mismatch_vector_matrix_2x2() {
    // [1,2,3] * [[1,2],[3,4]] should error (2x2 matrix is neither column nor row)
    assert!(parse_and_eval("[1,2,3] * [[1,2],[3,4]]").is_err());
}

// Phase 6: Matrix power

#[test]
fn test_matrix_pow_integer() {
    // [[1,1],[0,1]] ^ 3 = [[1,3],[0,1]]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 3.0], vec![0.0, 1.0]];
    expect_matrix(
        &parse_and_eval("[[1,1],[0,1]] ^ 3").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_scalar_pow_vector() {
    expect_vector(
        &parse_and_eval("2 ^ [1,2,3]").unwrap(),
        &[2.0, 4.0, 8.0],
        1e-9,
    );
}

#[test]
fn test_vector_pow_scalar() {
    expect_vector(
        &parse_and_eval("[2,3,4] ^ 2").unwrap(),
        &[4.0, 9.0, 16.0],
        1e-9,
    );
}

// Phase 7: Matrix operations — det, tr, inv, fnorm, norm

#[test]
fn test_det_2x2() {
    // det([[1,2],[3,4]]) = 1*4 - 2*3 = -2
    expect_scalar(&parse_and_eval("det([[1,2],[3,4]])").unwrap(), -2.0, 1e-9);
}

#[test]
fn test_det_3x3() {
    // det([[1,2,3],[4,5,6],[7,8,9]]) = 0 (singular)
    expect_scalar(
        &parse_and_eval("det([[1,2,3],[4,5,6],[7,8,9]])").unwrap(),
        0.0,
        1e-9,
    );
}

#[test]
fn test_det_identity() {
    expect_scalar(&parse_and_eval("det([[1,0],[0,1]])").unwrap(), 1.0, 1e-9);
}

#[test]
fn test_det_diagonal() {
    // det([[2,0],[0,3]]) = 6
    expect_scalar(&parse_and_eval("det([[2,0],[0,3]])").unwrap(), 6.0, 1e-9);
}

#[test]
fn test_det_1x1() {
    expect_scalar(&parse_and_eval("det([[5]])").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_det_non_square_error() {
    assert!(parse_and_eval("det([[1,2,3],[4,5,6]])").is_err());
}

#[test]
fn test_tr_2x2() {
    // tr([[1,2],[3,4]]) = 1 + 4 = 5
    expect_scalar(&parse_and_eval("tr([[1,2],[3,4]])").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_tr_3x3() {
    // tr([[1,2,3],[4,5,6],[7,8,9]]) = 1 + 5 + 9 = 15
    expect_scalar(
        &parse_and_eval("tr([[1,2,3],[4,5,6],[7,8,9]])").unwrap(),
        15.0,
        1e-9,
    );
}

#[test]
fn test_tr_identity() {
    // tr(I_3) = 3
    expect_scalar(
        &parse_and_eval("tr([[1,0,0],[0,1,0],[0,0,1]])").unwrap(),
        3.0,
        1e-9,
    );
}

#[test]
fn test_tr_1x1() {
    expect_scalar(&parse_and_eval("tr([[7]])").unwrap(), 7.0, 1e-9);
}

#[test]
fn test_tr_non_square_error() {
    assert!(parse_and_eval("tr([[1,2,3],[4,5,6]])").is_err());
}

#[test]
fn test_inv_2x2() {
    // inv([[1,2],[3,4]]) in column-major: col0=[1,2], col1=[3,4]
    // matrix = |1 3|, det=-2, inv = | -2   1.5 |
    //         |2 4|                |  1  -0.5 |
    // In column-major: col0=[-2,1], col1=[1.5,-0.5]
    let m: Vec<Vec<f64>> = vec![vec![-2.0, 1.0], vec![1.5, -0.5]];
    expect_matrix(
        &parse_and_eval("inv([[1,2],[3,4]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_inv_identity() {
    // inv(I) = I
    let m: Vec<Vec<f64>> = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    expect_matrix(
        &parse_and_eval("inv([[1,0],[0,1]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_inv_3x3() {
    // inv([[1,0,0],[0,2,0],[0,0,3]]) = [[1,0,0],[0,0.5,0],[0,0,0.333...]]
    let m: Vec<Vec<f64>> = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 0.5, 0.0],
        vec![0.0, 0.0, 1.0 / 3.0],
    ];
    expect_matrix(
        &parse_and_eval("inv([[1,0,0],[0,2,0],[0,0,3]])").unwrap(),
        &[&m[0], &m[1], &m[2]],
        1e-9,
    );
}

#[test]
fn test_inv_scalar() {
    // inv(5) = [[0.2]]
    let m: Vec<Vec<f64>> = vec![vec![0.2]];
    expect_matrix(&parse_and_eval("inv(5)").unwrap(), &[&m[0]], 1e-9);
}

#[test]
fn test_inv_singular_error() {
    // inv of singular matrix should error
    assert!(parse_and_eval("inv([[1,2],[2,4]])").is_err());
}

#[test]
fn test_inv_non_square_error() {
    assert!(parse_and_eval("inv([[1,2,3],[4,5,6]])").is_err());
}

#[test]
fn test_inv_a_times_inv_a_is_identity() {
    // A * inv(A) = I
    let a = parse_and_eval("[[1,2],[3,4]]").unwrap();
    let ai = parse_and_eval("inv([[1,2],[3,4]])").unwrap();
    let result = parse_and_eval(&format!("{} * {}", a, ai)).unwrap();
    // I in column-major: col0=[1,0], col1=[0,1]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    expect_matrix(&result, &[&m[0], &m[1]], 1e-9);
}

#[test]
fn test_fnorm_2x2() {
    // norm([[3,4],[0,0]]) = sqrt(9+16) = 5
    expect_scalar(&parse_and_eval("fnorm([[3,4],[0,0]])").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_fnorm_identity() {
    // norm(I_2) = sqrt(1+1) = sqrt(2)
    expect_scalar(
        &parse_and_eval("fnorm([[1,0],[0,1]])").unwrap(),
        std::f64::consts::SQRT_2,
        1e-9,
    );
}

#[test]
fn test_fnorm_vector() {
    // norm([3,4,0]) = sqrt(9+16) = 5
    expect_scalar(&parse_and_eval("fnorm([3,4,0])").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_fnorm_zero_matrix() {
    expect_scalar(&parse_and_eval("fnorm([[0,0],[0,0]])").unwrap(), 0.0, 1e-9);
}

// Euclidean column normalization (normalize)

#[test]
fn test_normalize_identity() {
    // normalize(I_2) = I_2 (each column is already unit length)
    let m: Vec<Vec<f64>> = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    expect_matrix(
        &parse_and_eval("normalize([[1,0],[0,1]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_normalize_2x2() {
    // normalize([[3,0],[0,4]]) → col0=[3,0]/3=[1,0], col1=[0,4]/4=[0,1] → [[1,0],[0,1]]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    expect_matrix(
        &parse_and_eval("normalize([[3,0],[0,4]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_normalize_vector() {
    // normalize([3,4]) → col=[3/5, 4/5] = [0.6, 0.8]
    let m: Vec<Vec<f64>> = vec![vec![0.6, 0.8]];
    expect_matrix(&parse_and_eval("normalize([3,4])").unwrap(), &[&m[0]], 1e-9);
}

#[test]
fn test_normalize_zero_column() {
    // normalize([[0,3],[0,4]]) — col0=[0,3]/3=[0,1], col1=[0,4]/4=[0,1]
    // Column-major result: col0=[0,1], col1=[0,1] → [[0,0],[1,1]]
    let m: Vec<Vec<f64>> = vec![vec![0.0, 1.0], vec![0.0, 1.0]];
    expect_matrix(
        &parse_and_eval("normalize([[0,3],[0,4]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_normalize_scalar() {
    // normalize(5) → [[5]] → col0=[5]/5=[1] → [[1]]
    let m: Vec<Vec<f64>> = vec![vec![1.0]];
    expect_matrix(&parse_and_eval("normalize(5)").unwrap(), &[&m[0]], 1e-9);
}

#[test]
fn test_normalize_in_expression() {
    // normalize([3,4]) + [1,1] → col=[1.6, 1.8]
    let m: Vec<Vec<f64>> = vec![vec![1.6, 1.8]];
    expect_matrix(
        &parse_and_eval("normalize([3,4]) + [1,1]").unwrap(),
        &[&m[0]],
        1e-9,
    );
}

#[test]
fn test_normalize_broadcast_error() {
    // normalize cannot be broadcast
    assert!(parse_and_eval("normalize.([1,2,3])").is_err());
}

// Column-wise L2 norm (norm) — returns a scalar vector

#[test]
fn test_norm_identity() {
    // norm(I_2) = [1.0, 1.0]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 1.0]];
    expect_matrix(
        &parse_and_eval("norm([[1,0],[0,1]])").unwrap(),
        &[&m[0]],
        1e-9,
    );
}

#[test]
fn test_norm_2x2() {
    // norm([[3,0],[0,4]]) → col0 norm=3, col1 norm=4 → [3.0, 4.0]
    let m: Vec<Vec<f64>> = vec![vec![3.0, 4.0]];
    expect_matrix(
        &parse_and_eval("norm([[3,0],[0,4]])").unwrap(),
        &[&m[0]],
        1e-9,
    );
}

#[test]
fn test_norm_vector() {
    // norm([3,4]) → col norm = 5
    let m: Vec<Vec<f64>> = vec![vec![5.0]];
    expect_matrix(&parse_and_eval("norm([3,4])").unwrap(), &[&m[0]], 1e-9);
}

#[test]
fn test_norm_zero_column() {
    // norm([[0,3],[0,4]]) → col0 norm=3, col1 norm=4 → [3.0, 4.0]
    let m: Vec<Vec<f64>> = vec![vec![3.0, 4.0]];
    expect_matrix(
        &parse_and_eval("norm([[0,3],[0,4]])").unwrap(),
        &[&m[0]],
        1e-9,
    );
}

#[test]
fn test_norm_scalar() {
    // norm(5) → [[5]] → col0 norm = 5
    let m: Vec<Vec<f64>> = vec![vec![5.0]];
    expect_matrix(&parse_and_eval("norm(5)").unwrap(), &[&m[0]], 1e-9);
}

#[test]
fn test_norm_in_expression() {
    // norm([3,4]) + 1 = 6
    let m: Vec<Vec<f64>> = vec![vec![6.0]];
    expect_matrix(&parse_and_eval("norm([3,4]) + 1").unwrap(), &[&m[0]], 1e-9);
}

#[test]
fn test_norm_broadcast_error() {
    // norm cannot be broadcast
    assert!(parse_and_eval("norm.([1,2,3])").is_err());
}

// Frobenius normalization (fnormalize)

#[test]
fn test_fnormalize_2x2() {
    // fnorm([[3,4],[0,0]]) = 5, so fnormalize = [[0.6, 0.8], [0, 0]]
    let m: Vec<Vec<f64>> = vec![vec![0.6, 0.8], vec![0.0, 0.0]];
    expect_matrix(
        &parse_and_eval("fnormalize([[3,4],[0,0]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_fnormalize_identity() {
    // fnorm(I_2) = sqrt(2), so fnormalize(I_2) = I_2 / sqrt(2)
    let s = std::f64::consts::SQRT_2;
    let m: Vec<Vec<f64>> = vec![vec![1.0 / s, 0.0], vec![0.0, 1.0 / s]];
    expect_matrix(
        &parse_and_eval("fnormalize([[1,0],[0,1]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_fnormalize_vector() {
    // fnorm([3,4]) = 5, so fnormalize = [0.6, 0.8]
    let m: Vec<Vec<f64>> = vec![vec![0.6, 0.8]];
    expect_matrix(
        &parse_and_eval("fnormalize([3,4])").unwrap(),
        &[&m[0]],
        1e-9,
    );
}

#[test]
fn test_fnormalize_zero_matrix() {
    // fnorm of zero matrix = 0, so fnormalize returns unchanged
    let m: Vec<Vec<f64>> = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
    expect_matrix(
        &parse_and_eval("fnormalize([[0,0],[0,0]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_fnormalize_scalar() {
    // fnormalize(5) → [[5]] / 5 = [[1]]
    let m: Vec<Vec<f64>> = vec![vec![1.0]];
    expect_matrix(&parse_and_eval("fnormalize(5)").unwrap(), &[&m[0]], 1e-9);
}

#[test]
fn test_fnormalize_in_expression() {
    // fnormalize([3,4]) + [1,1] = [1.6, 1.8]
    let m: Vec<Vec<f64>> = vec![vec![1.6, 1.8]];
    expect_matrix(
        &parse_and_eval("fnormalize([3,4]) + [1,1]").unwrap(),
        &[&m[0]],
        1e-9,
    );
}

#[test]
fn test_fnormalize_broadcast_error() {
    // fnormalize cannot be broadcast
    assert!(parse_and_eval("fnormalize.([1,2,3])").is_err());
}

#[test]
fn test_inv_in_expression() {
    // inv([[1,0],[0,1]]) = [[1,0],[0,1]]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    expect_matrix(
        &parse_and_eval("inv([[1,0],[0,1]])").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}
// Scalar-resulting matrix functions

#[test]
fn test_det_scalar() {
    // det(5) = det([5]) = 5
    expect_scalar(&parse_and_eval("det(5)").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_tr_scalar() {
    // tr(5) = tr([5]) = 5
    expect_scalar(&parse_and_eval("tr(5)").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_fnorm_scalar() {
    // norm(5) = 5
    expect_scalar(&parse_and_eval("fnorm(5)").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_det_in_expression() {
    // det([[1,2],[3,4]]) + 10 = 8
    expect_scalar(
        &parse_and_eval("det([[1,2],[3,4]]) + 10").unwrap(),
        8.0,
        1e-9,
    );
}

#[test]
fn test_tr_in_expression() {
    // tr([[1,2],[3,4]]) * 2 = 10
    expect_scalar(
        &parse_and_eval("tr([[1,2],[3,4]]) * 2").unwrap(),
        10.0,
        1e-9,
    );
}

#[test]
fn test_fnorm_in_expression() {
    // norm([3,4]) + 1 = 6
    expect_scalar(&parse_and_eval("fnorm([3,4]) + 1").unwrap(), 6.0, 1e-9);
}

#[test]
fn test_det_nested() {
    // det(inv([[1,2],[3,4]])) = det([[1,2],[3,4]])^(-1) = 1/(-2) = -0.5
    expect_scalar(
        &parse_and_eval("det(inv([[1,2],[3,4]]))").unwrap(),
        -0.5,
        1e-9,
    );
}

// ── eye() tests ────────────────────────────────────────────────────

#[test]
fn test_eye_2x2() {
    // eye(2) = [[1,0],[0,1]] in column-major: col0=[1,0], col1=[0,1]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    expect_matrix(&parse_and_eval("eye(2)").unwrap(), &[&m[0], &m[1]], 1e-9);
}

#[test]
fn test_eye_3x3() {
    // eye(3) in column-major: col0=[1,0,0], col1=[0,1,0], col2=[0,0,1]
    let m: Vec<Vec<f64>> = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    expect_matrix(
        &parse_and_eval("eye(3)").unwrap(),
        &[&m[0], &m[1], &m[2]],
        1e-9,
    );
}

#[test]
fn test_eye_1x1() {
    let m: Vec<Vec<f64>> = vec![vec![1.0]];
    expect_matrix(&parse_and_eval("eye(1)").unwrap(), &[&m[0]], 1e-9);
}

#[test]
fn test_eye_zero() {
    // eye(0) should return an empty matrix
    let result = parse_and_eval("eye(0)").unwrap();
    match result {
        crate::value::Value::Matrix(m) => assert!(m.is_empty()),
        other => panic!("Expected empty matrix, got {:?}", other),
    }
}

#[test]
fn test_eye_in_expression() {
    // eye(2) * [1,2] = [1,2]
    expect_vector(
        &parse_and_eval("eye(2) * [1,2]").unwrap(),
        &[1.0, 2.0],
        1e-9,
    );
}

#[test]
fn test_eye_non_integer_error() {
    assert!(parse_and_eval("eye(2.5)").is_err());
}

#[test]
fn test_eye_negative_error() {
    assert!(parse_and_eval("eye(-1)").is_err());
}

// ── diag() tests ───────────────────────────────────────────────────

#[test]
fn test_diag_vector_to_matrix() {
    // diag([1,2,3]) should create a 3x3 diagonal matrix
    // In column-major: col0=[1,0,0], col1=[0,2,0], col2=[0,0,3]
    let m: Vec<Vec<f64>> = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 3.0],
    ];
    expect_matrix(
        &parse_and_eval("diag([1,2,3])").unwrap(),
        &[&m[0], &m[1], &m[2]],
        1e-9,
    );
}

#[test]
fn test_diag_scalar() {
    // diag(5) should return a 1x1 matrix [[5]]
    let m: Vec<Vec<f64>> = vec![vec![5.0]];
    expect_matrix(&parse_and_eval("diag(5)").unwrap(), &[&m[0]], 1e-9);
}

#[test]
fn test_diag_matrix_extract() {
    // diag([[1,2],[3,4]]) should extract diagonal [1,4]
    // In column-major: col0=[1,4]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 4.0]];
    expect_matrix(
        &parse_and_eval("diag([[1,2],[3,4]])").unwrap(),
        &[&m[0]],
        1e-9,
    );
}

#[test]
fn test_diag_2x2_matrix() {
    // diag([[2,0],[0,3]]) should extract diagonal [2,3]
    let m: Vec<Vec<f64>> = vec![vec![2.0, 3.0]];
    expect_matrix(
        &parse_and_eval("diag([[2,0],[0,3]])").unwrap(),
        &[&m[0]],
        1e-9,
    );
}

#[test]
fn test_diag_non_square_extract() {
    // diag([[1,2,3],[4,5,6]]) should extract min(2,3)=2 diagonal elements: [1,5]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 5.0]];
    expect_matrix(
        &parse_and_eval("diag([[1,2,3],[4,5,6]])").unwrap(),
        &[&m[0]],
        1e-9,
    );
}

#[test]
fn test_diag_roundtrip() {
    // diag(diag(A)) should give back the diagonal as a 1x1 matrix for a diagonal matrix
    // Actually: diag([1,2,3]) creates diag matrix, then diag of that extracts [1,2,3]
    let m: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0]];
    expect_matrix(
        &parse_and_eval("diag(diag([1,2,3]))").unwrap(),
        &[&m[0]],
        1e-9,
    );
}

#[test]
fn test_diag_in_expression() {
    // diag([1,2]) + eye(2) = [[2,0],[0,3]]
    let m: Vec<Vec<f64>> = vec![vec![2.0, 0.0], vec![0.0, 3.0]];
    expect_matrix(
        &parse_and_eval("diag([1,2]) + eye(2)").unwrap(),
        &[&m[0], &m[1]],
        1e-9,
    );
}

#[test]
fn test_diag_broadcast_error() {
    // diag cannot be broadcast
    assert!(parse_and_eval("diag.([1,2,3])").is_err());
}

#[test]
fn test_eye_broadcast_error() {
    // eye cannot be broadcast
    assert!(parse_and_eval("eye.([1,2,3])").is_err());
}

// ── Pretty print display tests ─────────────────────────────────────

#[test]
fn test_pretty_print_real_matrix() {
    let result = parse_and_eval("[[1,3],[4,5]]").unwrap();
    let s = format!("{result:#}");
    // Should display as a 2x2 matrix with correct row/column ordering
    assert!(
        s.contains("2x2 real matrix"),
        "Display should contain dimensions: {}",
        s
    );
    // The matrix [[1,3],[4,5]] in column-major is col0=[1,4], col1=[3,5]
    // So the display should show the transpose:
    // 1 4
    // 3 5
    let lines: Vec<&str> = s.lines().collect();
    // Skip the empty line and header line (first two lines are empty and dimensions)
    let data_lines: Vec<&str> = lines
        .into_iter()
        .skip(2)
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        data_lines.len(),
        2,
        "Expected 2 data lines, got {}: {}",
        data_lines.len(),
        s
    );
    // First row should be "1 4"
    assert!(
        data_lines[0].contains("1") && data_lines[0].contains("4"),
        "First row should contain 1 and 4, got: {}",
        data_lines[0]
    );
    // Second row should be "3 5"
    assert!(
        data_lines[1].contains("3") && data_lines[1].contains("5"),
        "Second row should contain 3 and 5, got: {}",
        data_lines[1]
    );
}
