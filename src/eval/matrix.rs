use anyhow::Result;

use crate::value::Value;

/// O(n³) matrix multiplication with dimension-checking.
///
/// Matrices are stored in column-major format: each inner `Vec` is a column.
/// `a` has `n_cols_a` columns each of length `n_rows_a` (matrix is `n_rows_a × n_cols_a`).
/// `b` has `n_cols_b` columns each of length `n_rows_b` (matrix is `n_rows_b × n_cols_b`).
/// For multiplication: `n_cols_a` must equal `n_rows_b`.
/// Result has `n_cols_b` columns each of length `n_rows_a`.
pub fn matmul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
    let n_cols_a = a.len();
    let n_rows_a = a.first().map_or(0, |r| r.len());
    let n_cols_b = b.len();
    let n_rows_b = b.first().map_or(0, |r| r.len());
    if n_cols_a != n_rows_b {
        anyhow::bail!(
            "Matrix dimension mismatch: {}x{} * {}x{}",
            n_rows_a,
            n_cols_a,
            n_rows_b,
            n_cols_b
        );
    }
    // Result: n_cols_b columns, each of length n_rows_a
    let mut result = vec![vec![0.0; n_rows_a]; n_cols_b];
    for j in 0..n_cols_b {
        for k in 0..n_cols_a {
            let b_val = b[j][k];
            let a_col = &a[k];
            for i in 0..n_rows_a {
                result[j][i] += a_col[i] * b_val;
            }
        }
    }
    Ok(result)
}

/// Compute `base^exp` for a square matrix using repeated squaring.
///
/// Requires a non-negative integer exponent and a square matrix.
pub fn matrix_pow(base: &[Vec<f64>], exp: f64) -> Result<Vec<Vec<f64>>> {
    if exp != exp.floor() || exp < 0.0 {
        anyhow::bail!("Matrix power requires non-negative integer exponent");
    }
    let n = base.len();
    if base.first().map_or(true, |r| r.len() != n) {
        anyhow::bail!("Matrix power requires square matrix");
    }
    let mut result = identity(n);
    let mut b = base.to_vec();
    let mut e = exp as u64;
    while e > 0 {
        if e % 2 == 1 {
            result = matmul(&result, &b)?;
        }
        b = matmul(&b, &b)?;
        e /= 2;
    }
    Ok(result)
}

pub fn identity(n: usize) -> Vec<Vec<f64>> {
    let mut mat = vec![vec![0.0; n]; n];
    for i in 0..n {
        mat[i][i] = 1.0;
    }
    mat
}

/// Compute the determinant of a square matrix using LU decomposition with partial pivoting.
///
/// Matrices are stored in column-major format: each inner `Vec` is a column.
/// Returns an error if the matrix is not square.
pub fn matrix_det(mat: &[Vec<f64>]) -> Result<f64> {
    let n = mat.len();
    if n == 0 || mat.first().map_or(true, |r| r.len() != n) {
        anyhow::bail!(
            "det() requires a square matrix, got {}x{}",
            n,
            mat.first().map_or(0, |r| r.len())
        );
    }
    if n == 1 {
        return Ok(mat[0][0]);
    }

    // Convert column-major to row-major for standard LU decomposition.
    let mut a: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for col in 0..n {
        for row in 0..n {
            a[row][col] = mat[col][row];
        }
    }

    // LU decomposition with partial pivoting on row-major matrix a[row][col]
    let mut det = 1.0;
    for col in 0..n {
        // Find pivot row (partial pivoting)
        let mut pivot_row = col;
        let mut max_val = a[col][col].abs();
        for row in (col + 1)..n {
            if a[row][col].abs() > max_val {
                max_val = a[row][col].abs();
                pivot_row = row;
            }
        }

        // Check for singular matrix
        if max_val < 1e-15 {
            return Ok(0.0);
        }

        // Swap rows if needed
        if pivot_row != col {
            a.swap(col, pivot_row);
            det *= -1.0;
        }

        det *= a[col][col];

        // Eliminate below
        for row in (col + 1)..n {
            let factor = a[row][col] / a[col][col];
            for j in (col + 1)..n {
                a[row][j] -= factor * a[col][j];
            }
        }
    }

    Ok(det)
}

/// Compute the trace of a square matrix (sum of diagonal elements).
///
/// Matrices are stored in column-major format: each inner `Vec` is a column.
/// Returns an error if the matrix is not square.
pub fn matrix_trace(mat: &[Vec<f64>]) -> Result<f64> {
    let n = mat.len();
    if n == 0 || mat.first().map_or(true, |r| r.len() != n) {
        anyhow::bail!(
            "tr() requires a square matrix, got {}x{}",
            n,
            mat.first().map_or(0, |r| r.len())
        );
    }
    // Diagonal: element at (i,i) = mat[i][i] in column-major
    Ok((0..n).map(|i| mat[i][i]).sum())
}

/// Compute the inverse of a square matrix using Gauss-Jordan elimination with partial pivoting.
///
/// Matrices are stored in column-major format: each inner `Vec` is a column.
/// Returns an error if the matrix is not square or is singular.
pub fn matrix_inv(mat: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
    let n = mat.len();
    if n == 0 || mat.first().map_or(true, |r| r.len() != n) {
        anyhow::bail!(
            "inv() requires a square matrix, got {}x{}",
            n,
            mat.first().map_or(0, |r| r.len())
        );
    }

    // Convert column-major to row-major
    let mut a: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for col in 0..n {
        for row in 0..n {
            a[row][col] = mat[col][row];
        }
    }

    // Build augmented matrix [A | I] in row-major: aug[row][col] for col in 0..2n
    let mut aug: Vec<Vec<f64>> = vec![vec![0.0; 2 * n]; n];
    for row in 0..n {
        for col in 0..n {
            aug[row][col] = a[row][col];
        }
        aug[row][n + row] = 1.0;
    }

    // Gauss-Jordan elimination with partial pivoting on row-major augmented matrix
    for col in 0..n {
        // Find pivot row
        let mut pivot_row = col;
        let mut max_val = aug[col][col].abs();
        for row in (col + 1)..n {
            if aug[row][col].abs() > max_val {
                max_val = aug[row][col].abs();
                pivot_row = row;
            }
        }

        if max_val < 1e-15 {
            anyhow::bail!("inv() matrix is singular (non-invertible)");
        }

        // Swap rows
        if pivot_row != col {
            aug.swap(col, pivot_row);
        }

        // Scale pivot row to make pivot = 1
        let pivot_val = aug[col][col];
        for j in 0..2 * n {
            aug[col][j] /= pivot_val;
        }

        // Eliminate all other rows
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = aug[row][col];
            for j in 0..2 * n {
                aug[row][j] -= factor * aug[col][j];
            }
        }
    }

    // Extract inverse from right half and convert back to column-major
    Ok((0..n)
        .map(|col| (0..n).map(|row| aug[row][n + col]).collect())
        .collect())
}

/// Compute the Frobenius norm of a matrix (sqrt of sum of squared elements).
///
/// Works on any matrix (square or rectangular).
pub fn matrix_fnorm(mat: &[Vec<f64>]) -> Result<f64> {
    let sum_sq: f64 = mat.iter().flat_map(|col| col.iter()).map(|x| x * x).sum();
    Ok(sum_sq.sqrt())
}

/// Compute the Euclidean (L2) norm of each column and normalize.
///
/// Returns a matrix of the same shape where each column is divided by its
/// Euclidean norm. Columns with zero norm are left unchanged (division by zero
/// yields the original zero column).
pub fn matrix_normalize(mat: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
    let mut result = mat.to_vec();
    for col in &mut result {
        let sum_sq: f64 = col.iter().map(|x| x * x).sum();
        let norm = sum_sq.sqrt();
        if norm > 1e-15 {
            for x in col.iter_mut() {
                *x /= norm;
            }
        }
    }
    Ok(result)
}

/// Compute the Euclidean (L2) norm of each column.
///
/// Returns a vector of norms (one per column). Columns with zero norm
/// produce a norm of 0.0.
pub fn matrix_norm(mat: &[Vec<f64>]) -> Result<Vec<f64>> {
    let norms: Vec<f64> = mat
        .iter()
        .map(|col| {
            let sum_sq: f64 = col.iter().map(|x| x * x).sum();
            sum_sq.sqrt()
        })
        .collect();
    Ok(norms)
}

/// Divide all matrix elements by the Frobenius norm.
///
/// Returns a matrix of the same shape. If the Frobenius norm is zero
/// (all elements are zero), the matrix is returned unchanged.
pub fn matrix_fnormalize(mat: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
    let f = matrix_fnorm(mat)?;
    if f > 1e-15 {
        let result: Vec<Vec<f64>> = mat
            .iter()
            .map(|col| col.iter().map(|x| x / f).collect())
            .collect();
        Ok(result)
    } else {
        Ok(mat.to_vec())
    }
}

/// Compute the determinant of a complex square matrix using LU decomposition with partial pivoting.
///
/// Matrices are stored in column-major format: each inner `Vec` is a column.
/// Returns a `Complex<f64>` (the full complex determinant).
pub fn matrix_det_complex(mat: &[Vec<num_complex::Complex<f64>>]) -> Result<num_complex::Complex<f64>> {
    let n = mat.len();
    if n == 0 || mat.first().map_or(true, |r| r.len() != n) {
        anyhow::bail!(
            "det() requires a square matrix, got {}x{}",
            n,
            mat.first().map_or(0, |r| r.len())
        );
    }
    if n == 1 {
        return Ok(mat[0][0]);
    }

    // Convert column-major to row-major for standard LU decomposition.
    let mut a: Vec<Vec<num_complex::Complex<f64>>> = vec![
        vec![num_complex::Complex::new(0.0, 0.0); n]; n
    ];
    for col in 0..n {
        for row in 0..n {
            a[row][col] = mat[col][row];
        }
    }

    // LU decomposition with partial pivoting on row-major matrix a[row][col]
    // Pivot selection uses norm_sqr() since Complex<f64> has no total order.
    let mut det = num_complex::Complex::new(1.0, 0.0);
    let mut swaps = 0usize;
    for col in 0..n {
        // Find pivot row (partial pivoting by norm_sqr)
        let mut pivot_row = col;
        let mut max_val = a[col][col].norm_sqr();
        for row in (col + 1)..n {
            let ns = a[row][col].norm_sqr();
            if ns > max_val {
                max_val = ns;
                pivot_row = row;
            }
        }

        // Check for singular matrix (norm_sqr < 1e-30 corresponds to norm < 1e-15)
        if max_val < 1e-30 {
            return Ok(num_complex::Complex::new(0.0, 0.0));
        }

        // Swap rows if needed
        if pivot_row != col {
            a.swap(col, pivot_row);
            swaps += 1;
        }

        det *= a[col][col];

        // Eliminate below
        let pivot_row_data = a[col].clone();
        let pivot_val = a[col][col];
        for row in (col + 1)..n {
            let factor = a[row][col] / pivot_val;
            for j in (col + 1)..n {
                a[row][j] -= factor * pivot_row_data[j];
            }
        }
    }

    if swaps % 2 == 1 {
        det = -det;
    }

    Ok(det)
}

/// Compute the inverse of a complex square matrix using Gauss-Jordan elimination with partial pivoting.
///
/// Matrices are stored in column-major format: each inner `Vec` is a column.
/// Returns a `ComplexMatrix` (row-major inner vectors, column-major outer).
pub fn matrix_inv_complex(
    mat: &[Vec<num_complex::Complex<f64>>],
) -> Result<Vec<Vec<num_complex::Complex<f64>>>> {
    let n = mat.len();
    if n == 0 || mat.first().map_or(true, |r| r.len() != n) {
        anyhow::bail!(
            "inv() requires a square matrix, got {}x{}",
            n,
            mat.first().map_or(0, |r| r.len())
        );
    }

    // Convert column-major to row-major
    let mut a: Vec<Vec<num_complex::Complex<f64>>> = vec![
        vec![num_complex::Complex::new(0.0, 0.0); n]; n
    ];
    for col in 0..n {
        for row in 0..n {
            a[row][col] = mat[col][row];
        }
    }

    // Build augmented matrix [A | I] in row-major
    let mut aug: Vec<Vec<num_complex::Complex<f64>>> = vec![
        vec![num_complex::Complex::new(0.0, 0.0); 2 * n]; n
    ];
    for row in 0..n {
        for col in 0..n {
            aug[row][col] = a[row][col];
        }
        aug[row][n + row] = num_complex::Complex::new(1.0, 0.0);
    }

    // Gauss-Jordan elimination with partial pivoting on row-major augmented matrix
    for col in 0..n {
        // Find pivot row (partial pivoting by norm_sqr)
        let mut pivot_row = col;
        let mut max_val = aug[col][col].norm_sqr();
        for row in (col + 1)..n {
            let ns = aug[row][col].norm_sqr();
            if ns > max_val {
                max_val = ns;
                pivot_row = row;
            }
        }

        if max_val < 1e-30 {
            anyhow::bail!("inv() matrix is singular (non-invertible)");
        }

        // Swap rows
        if pivot_row != col {
            aug.swap(col, pivot_row);
        }

        // Scale pivot row to make pivot = 1
        let pivot_val = aug[col][col];
        for j in 0..2 * n {
            aug[col][j] /= pivot_val;
        }

        // Eliminate all other rows
        let pivot_row_data = aug[col].clone();
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = aug[row][col];
            for j in 0..2 * n {
                aug[row][j] -= factor * pivot_row_data[j];
            }
        }
    }

    // Extract inverse from right half and convert back to column-major
    Ok((0..n)
        .map(|col| (0..n).map(|row| aug[row][n + col]).collect())
        .collect())
}

/// Compute the trace of a complex square matrix (sum of diagonal elements).
pub fn matrix_trace_complex(mat: &[Vec<num_complex::Complex<f64>>]) -> Result<f64> {
    let n = mat.len();
    if n == 0 || mat.first().map_or(true, |r| r.len() != n) {
        anyhow::bail!(
            "tr() requires a square matrix, got {}x{}",
            n,
            mat.first().map_or(0, |r| r.len())
        );
    }
    // Diagonal: element at (i,i) = mat[i][i] in column-major
    let sum: num_complex::Complex<f64> = (0..n).map(|i| mat[i][i]).sum();
    // Trace of a matrix with complex entries can be complex, but we return f64
    // For conjugate-symmetric matrices the trace is real; otherwise return the real part.
    Ok(sum.re)
}

/// Compute the Frobenius norm of a complex matrix: sqrt(sum of |x|²).
pub fn matrix_fnorm_complex(mat: &[Vec<num_complex::Complex<f64>>]) -> Result<f64> {
    let sum_sq: f64 = mat
        .iter()
        .flat_map(|col| col.iter())
        .map(|x| x.norm() * x.norm())
        .sum();
    Ok(sum_sq.sqrt())
}

/// Compute the Euclidean (L2) norm of each column of a complex matrix.
pub fn matrix_norm_complex(mat: &[Vec<num_complex::Complex<f64>>]) -> Result<Vec<f64>> {
    let norms: Vec<f64> = mat
        .iter()
        .map(|col| {
            let sum_sq: f64 = col.iter().map(|x| x.norm() * x.norm()).sum();
            sum_sq.sqrt()
        })
        .collect();
    Ok(norms)
}

/// Normalize each column of a complex matrix by its Euclidean norm.
pub fn matrix_normalize_complex(
    mat: &[Vec<num_complex::Complex<f64>>],
) -> Result<Vec<Vec<num_complex::Complex<f64>>>> {
    let mut result = mat.to_vec();
    for col in &mut result {
        let sum_sq: f64 = col.iter().map(|x| x.norm() * x.norm()).sum();
        let norm = sum_sq.sqrt();
        if norm > 1e-15 {
            for x in col.iter_mut() {
                *x = *x / num_complex::Complex::new(norm, 0.0);
            }
        }
    }
    Ok(result)
}

/// Divide all complex matrix elements by the Frobenius norm.
pub fn matrix_fnormalize_complex(
    mat: &[Vec<num_complex::Complex<f64>>],
) -> Result<Vec<Vec<num_complex::Complex<f64>>>> {
    let f = matrix_fnorm_complex(mat)?;
    if f > 1e-15 {
        let result: Vec<Vec<num_complex::Complex<f64>>> = mat
            .iter()
            .map(|col| {
                col.iter()
                    .map(|x| *x / num_complex::Complex::new(f, 0.0))
                    .collect()
            })
            .collect();
        Ok(result)
    } else {
        Ok(mat.to_vec())
    }
}

/// Create an n×n identity matrix.
///
/// If the size argument is complex, the identity matrix is returned as a ComplexMatrix.
pub fn eye(n: crate::value::Value) -> Result<crate::value::Value> {
    let n_val = match n {
        crate::value::Value::Scalar(v) => {
            if v != v.floor() || v < 0.0 {
                anyhow::bail!("eye() requires a non-negative integer size");
            }
            v as usize
        }
        crate::value::Value::Complex(c) => {
            if c.im != 0.0 || c.re != c.re.floor() || c.re < 0.0 {
                anyhow::bail!("eye() requires a non-negative integer size");
            }
            c.re as usize
        }
        _ => anyhow::bail!("eye() requires a scalar integer argument"),
    };
    if n_val == 0 {
        return Ok(crate::value::Value::Matrix(vec![]));
    }
    let mut mat = vec![vec![0.0; n_val]; n_val];
    for i in 0..n_val {
        mat[i][i] = 1.0;
    }
    Ok(crate::value::Value::Matrix(mat))
}

/// Create a diagonal matrix from a vector/scalar, or extract diagonal from a matrix.
///
/// - If input is a scalar: returns a 1×1 matrix containing that scalar.
/// - If input is a vector (1-column matrix): creates a diagonal matrix with those values on the diagonal.
/// - If input is a matrix: extracts diagonal elements as a vector.
///
/// Supports complex values: if any input element is complex, returns a ComplexMatrix.
pub fn diag(arg: crate::value::Value) -> Result<crate::value::Value> {
    match arg {
        crate::value::Value::Scalar(n) => {
            // 1×1 matrix containing the scalar
            Ok(crate::value::Value::Matrix(vec![vec![n]]))
        }
        crate::value::Value::Complex(c) => {
            // 1×1 complex matrix
            Ok(crate::value::Value::ComplexMatrix(vec![vec![c]]))
        }
        crate::value::Value::Matrix(m) => {
            // m is column-major: m.len() = number of columns, m[0].len() = number of rows
            let n_cols = m.len();
            let n_rows = m.first().map_or(0, |col| col.len());
            // Check if this is a vector (single column)
            if n_cols == 1 {
                // Vector → create diagonal matrix
                let n = n_rows;
                let mut mat = vec![vec![0.0; n]; n];
                for i in 0..n {
                    mat[i][i] = m[0][i];
                }
                Ok(crate::value::Value::Matrix(mat))
            } else {
                // Matrix → extract diagonal
                let min_dim = n_rows.min(n_cols);
                let diag_vals: Vec<f64> = (0..min_dim).map(|i| m[i][i]).collect();
                Ok(crate::value::Value::Matrix(vec![diag_vals]))
            }
        }
        crate::value::Value::ComplexMatrix(m) => {
            let n_cols = m.len();
            let n_rows = m.first().map_or(0, |col| col.len());
            if n_cols == 1 {
                // Complex vector → create complex diagonal matrix
                let n = n_rows;
                let mut mat = vec![vec![num_complex::Complex::new(0.0, 0.0); n]; n];
                for i in 0..n {
                    mat[i][i] = m[0][i];
                }
                Ok(crate::value::Value::ComplexMatrix(mat))
            } else {
                // Complex matrix → extract diagonal as complex vector
                let min_dim = n_rows.min(n_cols);
                let diag_vals: Vec<num_complex::Complex<f64>> = (0..min_dim).map(|i| m[i][i]).collect();
                Ok(crate::value::Value::ComplexMatrix(vec![diag_vals]))
            }
        }
    }
}

/// Apply a matrix-specific unary function (det, tr, inv, norm, normalize, fnorm, fnormalize) to a `Value`.
///
/// Accepts scalars (wrapped into 1×1 matrices) or matrices.
pub fn apply_matrix_unary(name: &str, arg: crate::value::Value) -> Result<crate::value::Value> {
    match name {
        "det" => match arg {
            Value::Scalar(n) => {
                let m = vec![vec![n]];
                Ok(Value::Scalar(matrix_det(&m)?))
            }
            Value::Matrix(m) => Ok(Value::Scalar(matrix_det(&m)?)),
            Value::Complex(c) => {
                let m = vec![vec![c]];
                Ok(Value::Complex(matrix_det_complex(&m)?))
            }
            Value::ComplexMatrix(m) => Ok(Value::Complex(matrix_det_complex(&m)?)),
        },
        "tr" => match arg {
            Value::Scalar(n) => {
                let m = vec![vec![n]];
                Ok(Value::Scalar(matrix_trace(&m)?))
            }
            Value::Matrix(m) => Ok(Value::Scalar(matrix_trace(&m)?)),
            Value::Complex(_) => {
                anyhow::bail!("tr() does not support complex numbers yet")
            }
            Value::ComplexMatrix(m) => Ok(Value::Scalar(matrix_trace_complex(&m)?)),
        },
        "inv" => match arg {
            Value::Scalar(n) => {
                let m = vec![vec![n]];
                Ok(Value::Matrix(matrix_inv(&m)?))
            }
            Value::Matrix(m) => Ok(Value::Matrix(matrix_inv(&m)?)),
            Value::Complex(c) => {
                let m = vec![vec![c]];
                Ok(Value::ComplexMatrix(matrix_inv_complex(&m)?))
            }
            Value::ComplexMatrix(m) => Ok(Value::ComplexMatrix(matrix_inv_complex(&m)?)),
        },
        "fnorm" => match arg {
            Value::Scalar(n) => {
                let m = vec![vec![n]];
                Ok(Value::Scalar(matrix_fnorm(&m)?))
            }
            Value::Matrix(m) => Ok(Value::Scalar(matrix_fnorm(&m)?)),
            Value::Complex(_) => {
                anyhow::bail!("fnorm() does not support complex numbers yet")
            }
            Value::ComplexMatrix(m) => Ok(Value::Scalar(matrix_fnorm_complex(&m)?)),
        },
        "fnormalize" => match arg {
            Value::Scalar(n) => {
                let m = vec![vec![n]];
                Ok(Value::Matrix(matrix_fnormalize(&m)?))
            }
            Value::Matrix(m) => Ok(Value::Matrix(matrix_fnormalize(&m)?)),
            Value::Complex(_) => {
                anyhow::bail!("fnormalize() does not support complex numbers yet")
            }
            Value::ComplexMatrix(m) => Ok(Value::ComplexMatrix(matrix_fnormalize_complex(&m)?)),
        },
        "norm" => match arg {
            Value::Scalar(n) => {
                let m = vec![vec![n]];
                let norms = matrix_norm(&m)?;
                Ok(Value::Matrix(vec![norms]))
            }
            Value::Matrix(m) => {
                let norms = matrix_norm(&m)?;
                Ok(Value::Matrix(vec![norms]))
            }
            Value::Complex(_) => {
                anyhow::bail!("norm() does not support complex numbers yet")
            }
            Value::ComplexMatrix(m) => {
                let norms = matrix_norm_complex(&m)?;
                Ok(Value::Matrix(vec![norms]))
            }
        },
        "normalize" => match arg {
            Value::Scalar(n) => {
                let m = vec![vec![n]];
                Ok(Value::Matrix(matrix_normalize(&m)?))
            }
            Value::Matrix(m) => Ok(Value::Matrix(matrix_normalize(&m)?)),
            Value::Complex(_) => {
                anyhow::bail!("normalize() does not support complex numbers yet")
            }
            Value::ComplexMatrix(m) => Ok(Value::ComplexMatrix(matrix_normalize_complex(&m)?)),
        },
        "eye" => eye(arg),
        "diag" => diag(arg),
        _ => anyhow::bail!("Unknown matrix function: {}", name),
    }
}
