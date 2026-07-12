use anyhow::Result;

use crate::ast::BinOp;
use crate::value::Value;

/// Apply a scalar function element-wise across matching shapes of two `Value`s.
///
/// Handles all valid combinations of scalar/vector/matrix operands with
/// automatic broadcasting. Returns an error on shape mismatch.
pub fn apply_element_wise<F: Fn(f64, f64) -> f64>(f: F, l: &Value, r: &Value) -> Result<Value> {
    match (l, r) {
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(f(*a, *b))),
        (Value::Scalar(a), Value::Matrix(b)) => Ok(Value::Matrix(
            b.iter()
                .map(|row| row.iter().map(|x| f(*a, *x)).collect())
                .collect(),
        )),
        (Value::Matrix(a), Value::Scalar(b)) => Ok(Value::Matrix(
            a.iter()
                .map(|row| row.iter().map(|x| f(*x, *b)).collect())
                .collect(),
        )),
        (Value::Matrix(a), Value::Matrix(b)) => {
            if a.len() != b.len()
                || a.first()
                    .map_or(true, |row| row.len() != b.first().map_or(0, |r| r.len()))
            {
                anyhow::bail!("Shape mismatch: {:?} vs {:?}", l, r);
            }
            Ok(Value::Matrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(row_a, row_b)| {
                        row_a
                            .iter()
                            .zip(row_b.iter())
                            .map(|(x, y)| f(*x, *y))
                            .collect()
                    })
                    .collect(),
            ))
        }
        (_, _) => anyhow::bail!("Operator not supported for this operand type"),
    }
}

/// Apply a scalar math function to a single `f64`.
pub fn apply_unary_fn_scalar(name: &str, arg: f64) -> f64 {
    match name {
        "sin" => arg.sin(),
        "cos" => arg.cos(),
        "tan" => arg.tan(),
        "asin" => arg.asin(),
        "acos" => arg.acos(),
        "atan" => arg.atan(),
        "exp" => arg.exp(),
        "ln" => arg.ln(),
        "log" => arg.log10(),
        "sqrt" => arg.sqrt(),
        "abs" => arg.abs(),
        "floor" => arg.floor(),
        "ceil" => arg.ceil(),
        "round" => arg.round(),
        "sinh" => arg.sinh(),
        "cosh" => arg.cosh(),
        "tanh" => arg.tanh(),
        "real" => arg,
        "imag" => 0.0,
        _ => panic!("Unknown broadcast function: {}", name),
    }
}

/// Dispatch a scalar math function on an `f64` value.
macro_rules! scalar_unary {
    ($n:expr, $name:expr) => {
        match $name {
            "sin" => Ok(Value::Scalar($n.sin())),
            "cos" => Ok(Value::Scalar($n.cos())),
            "tan" => Ok(Value::Scalar($n.tan())),
            "asin" => Ok(Value::Scalar($n.asin())),
            "acos" => Ok(Value::Scalar($n.acos())),
            "atan" => Ok(Value::Scalar($n.atan())),
            "exp" => Ok(Value::Scalar($n.exp())),
            "ln" => Ok(Value::Scalar($n.ln())),
            "log" => Ok(Value::Scalar($n.log10())),
            "sqrt" => Ok(Value::Scalar($n.sqrt())),
            "abs" => Ok(Value::Scalar($n.abs())),
            "floor" => Ok(Value::Scalar($n.floor())),
            "ceil" => Ok(Value::Scalar($n.ceil())),
            "round" => Ok(Value::Scalar($n.round())),
            "sinh" => Ok(Value::Scalar($n.sinh())),
            "cosh" => Ok(Value::Scalar($n.cosh())),
            "tanh" => Ok(Value::Scalar($n.tanh())),
            "conj" | "real" => Ok(Value::Scalar($n)),
            "imag" => Ok(Value::Complex(num_complex::Complex::new(0.0, 0.0))),
            _ => anyhow::bail!("Unknown function: {}", $name),
        }
    };
}

/// Dispatch a scalar math function on a `Complex<f64>` value.
macro_rules! complex_unary {
    ($c:expr, $name:expr) => {
        match $name {
            "sin" => Ok(Value::Complex($c.sin())),
            "cos" => Ok(Value::Complex($c.cos())),
            "tan" => Ok(Value::Complex($c.tan())),
            "asin" => Ok(Value::Complex($c.asin())),
            "acos" => Ok(Value::Complex($c.acos())),
            "atan" => Ok(Value::Complex($c.atan())),
            "exp" => Ok(Value::Complex($c.exp())),
            "ln" => Ok(Value::Complex($c.ln())),
            "log" => Ok(Value::Complex($c.log10())),
            "sqrt" => Ok(Value::Complex($c.sqrt())),
            "abs" => Ok(Value::Scalar($c.norm())),
            "sinh" => Ok(Value::Complex($c.sinh())),
            "cosh" => Ok(Value::Complex($c.cosh())),
            "tanh" => Ok(Value::Complex($c.tanh())),
            "conj" => Ok(Value::Complex($c.conj())),
            "real" => Ok(Value::Scalar($c.re)),
            "imag" => Ok(Value::Complex(num_complex::Complex::new(0.0, $c.im))),
            _ => anyhow::bail!("Function {} does not accept complex arguments", $name),
        }
    };
}

/// Returns true when applying a scalar function to a real `f64` would produce
/// `NaN`, but the function has a well-defined complex result.
///
/// Used to promote real scalars/matrices to complex before evaluation so that
/// e.g. `sqrt(-1)` returns `i` instead of `NaN`.
pub fn needs_complex_promotion(name: &str, arg: f64) -> bool {
    match name {
        "sqrt" => arg < 0.0,
        "ln" | "log" => arg <= 0.0,
        "asin" | "acos" => arg.abs() > 1.0,
        _ => false,
    }
}

/// Returns true when `base^exp` on real numbers would produce `NaN` but has a
/// well-defined complex result (negative base with non-integer exponent).
fn needs_complex_pow_promotion(base: f64, exp: f64) -> bool {
    base < 0.0 && exp.fract() != 0.0
}

/// Apply a scalar-only unary function (sin, cos, tan, asin, acos, atan, exp, ln,
/// log, sqrt, abs, floor, ceil, round, sinh, cosh, tanh) to a `Value`.
///
/// Accepts `Value::Scalar` and `Value::Complex`. Rejects matrices.
/// For complex args, uses `num-complex` trait impls for all transcendental functions.
///
/// Real scalars whose evaluation would yield `NaN` are automatically promoted to
/// `Value::Complex` so the complex implementation handles them (e.g. `sqrt(-1)` → `i`).
pub fn apply_scalar_unary(name: &str, arg: Value) -> Result<Value> {
    // Promote real scalars to complex when the function would produce NaN
    // but has a well-defined complex result.
    let arg = match arg {
        Value::Scalar(n) if needs_complex_promotion(name, n) => {
            Value::Complex(num_complex::Complex::new(n, 0.0))
        }
        other => other,
    };
    match arg {
        Value::Scalar(n) => scalar_unary!(n, name),
        Value::Complex(c) => complex_unary!(c, name),
        Value::ComplexMatrix(m) => match name {
            "conj" => Ok(Value::ComplexMatrix(
                m.iter()
                    .map(|col| col.iter().map(|c| c.conj()).collect())
                    .collect(),
            )),
            _ => anyhow::bail!("Function {} does not accept complex matrix arguments", name),
        },
        other => anyhow::bail!("Function {} does not accept {:?}", name, other),
    }
}

/// Apply a binary operator to two `Value`s, handling broadcasting and
/// the distinction between element-wise (dot) and matrix-multiplication semantics.
///
/// Delegates to element-wise or matrix-specific logic based on operand shapes.
pub fn apply_binop(op: BinOp, l: Value, r: Value) -> Result<Value> {
    // Route complex operands through complex arithmetic
    if has_complex(&l) || has_complex(&r) {
        return apply_complex_binop(op, l, r);
    }
    match op {
        BinOp::DotAdd | BinOp::Add => apply_element_wise(|a, b| a + b, &l, &r),
        BinOp::DotSub | BinOp::Sub => apply_element_wise(|a, b| a - b, &l, &r),
        BinOp::DotDiv | BinOp::Div => apply_element_wise(|a, b| a / b, &l, &r),
        BinOp::DotMod | BinOp::Mod => apply_element_wise(|a, b| a.rem_euclid(b), &l, &r),
        BinOp::DotMul => apply_element_wise(|a, b| a * b, &l, &r),
        BinOp::Mul => match (&l, &r) {
            (Value::Matrix(a), Value::Matrix(b)) => {
                crate::eval::matrix::matmul(a, b).map(Value::Matrix)
            }
            (Value::Matrix(a), Value::Scalar(b)) => Ok(Value::Matrix(
                a.iter()
                    .map(|row| row.iter().map(|x| x * b).collect())
                    .collect(),
            )),
            (Value::Scalar(a), Value::Matrix(b)) => Ok(Value::Matrix(
                b.iter()
                    .map(|row| row.iter().map(|x| a * x).collect())
                    .collect(),
            )),
            (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a * b)),
            (_, _) => anyhow::bail!("Multiplication not supported for these operand types"),
        },
        BinOp::DotPow => apply_element_wise(|a, b| a.powf(b), &l, &r),
        BinOp::Pow => match (&l, &r) {
            (Value::Scalar(a), Value::Scalar(b)) => {
                if needs_complex_pow_promotion(*a, *b) {
                    let ca = num_complex::Complex::new(*a, 0.0);
                    let cb = num_complex::Complex::new(*b, 0.0);
                    return Ok(Value::Complex(ca.powc(cb)));
                }
                Ok(Value::Scalar(a.powf(*b)))
            }
            (Value::Scalar(base), Value::Matrix(exp)) => {
                if exp
                    .iter()
                    .any(|row| row.iter().any(|&e| needs_complex_pow_promotion(*base, e)))
                {
                    let base_c = num_complex::Complex::new(*base, 0.0);
                    return Ok(Value::ComplexMatrix(
                        exp.iter()
                            .map(|row| row.iter().map(|&e| base_c.powc(e.into())).collect())
                            .collect(),
                    ));
                }
                Ok(Value::Matrix(
                    exp.iter()
                        .map(|row| row.iter().map(|e| base.powf(*e)).collect())
                        .collect(),
                ))
            }
            (Value::Matrix(base), Value::Scalar(exp)) => {
                let n = base.len();
                if base.first().map_or(true, |r| r.len() != n) {
                    if base
                        .iter()
                        .any(|row| row.iter().any(|&b| needs_complex_pow_promotion(b, *exp)))
                    {
                        let base_c: Vec<Vec<num_complex::Complex<f64>>> = base
                            .iter()
                            .map(|row| {
                                row.iter()
                                    .map(|&b| num_complex::Complex::new(b, 0.0))
                                    .collect()
                            })
                            .collect();
                        return Ok(Value::ComplexMatrix(
                            base_c
                                .iter()
                                .map(|row| row.iter().map(|b| b.powf(*exp)).collect())
                                .collect(),
                        ));
                    }
                    Ok(Value::Matrix(
                        base.iter()
                            .map(|row| row.iter().map(|b| b.powf(*exp)).collect())
                            .collect(),
                    ))
                } else {
                    crate::eval::matrix::matrix_pow(base, *exp).map(Value::Matrix)
                }
            }
            _ => anyhow::bail!("Invalid power: {:?} ^ {:?}", l, r),
        },
    }
}

/// Returns true if the value is a `Complex` scalar or a `ComplexMatrix`.
fn has_complex(v: &Value) -> bool {
    matches!(v, Value::Complex(_) | Value::ComplexMatrix(_))
}

/// Complex-aware binary operator dispatch.
///
/// When either operand is complex, use `num-complex` arithmetic.
/// Handles scalar, complex, matrix, and complex-matrix operands with broadcasting.
fn apply_complex_binop(op: BinOp, l: Value, r: Value) -> Result<Value> {
    match op {
        BinOp::Add | BinOp::DotAdd => apply_complex_element_wise(|a, b| a + b, l, r),
        BinOp::Sub | BinOp::DotSub => apply_complex_element_wise(|a, b| a - b, l, r),
        BinOp::DotMul => apply_complex_element_wise(|a, b| a * b, l, r),
        BinOp::DotDiv | BinOp::Div => apply_complex_element_wise(|a, b| a / b, l, r),
        BinOp::DotMod | BinOp::DotPow => {
            anyhow::bail!("Operator {:?} not supported for complex numbers", op)
        }
        BinOp::Mul => match (&l, &r) {
            (Value::ComplexMatrix(a), Value::ComplexMatrix(b)) => {
                cmatmul(a, b).map(Value::ComplexMatrix)
            }
            (Value::ComplexMatrix(a), Value::Complex(b)) => Ok(Value::ComplexMatrix(
                a.iter()
                    .map(|col| col.iter().map(|c| *c * b).collect())
                    .collect(),
            )),
            (Value::Complex(b), Value::ComplexMatrix(a)) => Ok(Value::ComplexMatrix(
                a.iter()
                    .map(|col| col.iter().map(|c| *b * c).collect())
                    .collect(),
            )),
            (Value::ComplexMatrix(a), Value::Scalar(b)) => Ok(Value::ComplexMatrix(
                a.iter()
                    .map(|col| {
                        col.iter()
                            .map(|c| *c * num_complex::Complex::from(b))
                            .collect()
                    })
                    .collect(),
            )),
            (Value::Scalar(a), Value::ComplexMatrix(b)) => Ok(Value::ComplexMatrix(
                b.iter()
                    .map(|col| {
                        col.iter()
                            .map(|c| num_complex::Complex::from(a) * c)
                            .collect()
                    })
                    .collect(),
            )),
            (Value::Matrix(a), Value::ComplexMatrix(b)) => {
                // Convert real matrix to complex and multiply
                let ac: Vec<Vec<num_complex::Complex<f64>>> = a
                    .iter()
                    .map(|col| col.iter().map(|x| num_complex::Complex::from(*x)).collect())
                    .collect();
                cmatmul(&ac, b).map(Value::ComplexMatrix)
            }
            (Value::ComplexMatrix(a), Value::Matrix(b)) => {
                let bc: Vec<Vec<num_complex::Complex<f64>>> = b
                    .iter()
                    .map(|col| col.iter().map(|x| num_complex::Complex::from(*x)).collect())
                    .collect();
                cmatmul(a, &bc).map(Value::ComplexMatrix)
            }
            (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a * b)),
            (Value::Complex(a), Value::Scalar(b)) => {
                Ok(Value::Complex(a * num_complex::Complex::from(b)))
            }
            (Value::Scalar(a), Value::Complex(b)) => {
                Ok(Value::Complex(num_complex::Complex::from(a) * b))
            }
            (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a * b)),
            _ => anyhow::bail!("Multiplication not supported for these operand types"),
        },
        BinOp::Pow => {
            let l_clone = l.clone();
            let r_clone = r.clone();
            match (l, r) {
                (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a.powc(b))),
                (Value::Complex(a), Value::Scalar(b)) => Ok(Value::Complex(a.powf(b))),
                (Value::Scalar(a), Value::Complex(b)) => {
                    Ok(Value::Complex(num_complex::Complex::new(a, 0.0).powc(b)))
                }
                (Value::ComplexMatrix(base), Value::Scalar(exp)) => Ok(Value::ComplexMatrix(
                    base.iter()
                        .map(|col| col.iter().map(|b| b.powf(exp)).collect())
                        .collect(),
                )),
                (Value::Scalar(base), Value::ComplexMatrix(exp)) => Ok(Value::ComplexMatrix(
                    exp.iter()
                        .map(|col| {
                            col.iter()
                                .map(|e| num_complex::Complex::new(base, 0.0).powc(*e))
                                .collect()
                        })
                        .collect(),
                )),
                _ => anyhow::bail!("Invalid power: {:?} ^ {:?}", l_clone, r_clone),
            }
        }
        _ => anyhow::bail!("Operator {:?} not supported for complex numbers", op),
    }
}

/// Element-wise complex arithmetic with broadcasting between scalar/complex/matrix/complex-matrix.
fn apply_complex_element_wise<F>(f: F, l: Value, r: Value) -> Result<Value>
where
    F: Fn(num_complex::Complex<f64>, num_complex::Complex<f64>) -> num_complex::Complex<f64>,
{
    match (l, r) {
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(f(a, b))),
        (Value::Complex(a), Value::Scalar(b)) => {
            Ok(Value::Complex(f(a, num_complex::Complex::from(b))))
        }
        (Value::Scalar(a), Value::Complex(b)) => {
            Ok(Value::Complex(f(num_complex::Complex::from(a), b)))
        }
        (Value::Complex(a), Value::ComplexMatrix(b)) => Ok(Value::ComplexMatrix(
            b.iter()
                .map(|col| col.iter().map(|c| f(a, *c)).collect())
                .collect(),
        )),
        (Value::ComplexMatrix(b), Value::Complex(a)) => Ok(Value::ComplexMatrix(
            b.iter()
                .map(|col| col.iter().map(|c| f(*c, a)).collect())
                .collect(),
        )),
        (Value::Scalar(a), Value::ComplexMatrix(b)) => Ok(Value::ComplexMatrix(
            b.iter()
                .map(|col| {
                    col.iter()
                        .map(|c| f(num_complex::Complex::from(a), *c))
                        .collect()
                })
                .collect(),
        )),
        (Value::ComplexMatrix(b), Value::Scalar(a)) => Ok(Value::ComplexMatrix(
            b.iter()
                .map(|col| {
                    col.iter()
                        .map(|c| f(*c, num_complex::Complex::from(a)))
                        .collect()
                })
                .collect(),
        )),
        (Value::ComplexMatrix(a), Value::ComplexMatrix(b)) => {
            if a.len() != b.len()
                || a.first()
                    .map_or(true, |col| col.len() != b.first().map_or(0, |c| c.len()))
            {
                anyhow::bail!(
                    "Shape mismatch: {:?} vs {:?}",
                    Value::ComplexMatrix(a.clone()),
                    Value::ComplexMatrix(b.clone())
                )
            }
            Ok(Value::ComplexMatrix(
                a.iter()
                    .zip(b.iter())
                    .map(|(col_a, col_b)| {
                        col_a
                            .iter()
                            .zip(col_b.iter())
                            .map(|(x, y)| f(*x, *y))
                            .collect()
                    })
                    .collect(),
            ))
        }
        (Value::Matrix(a), Value::ComplexMatrix(b)) => {
            if a.len() != b.len()
                || a.first()
                    .map_or(true, |col| col.len() != b.first().map_or(0, |c| c.len()))
            {
                anyhow::bail!(
                    "Shape mismatch: {:?} vs {:?}",
                    Value::Matrix(a.clone()),
                    Value::ComplexMatrix(b.clone())
                )
            }
            let ac: Vec<Vec<num_complex::Complex<f64>>> = a
                .iter()
                .map(|col| col.iter().map(|x| num_complex::Complex::from(*x)).collect())
                .collect();
            Ok(Value::ComplexMatrix(
                ac.iter()
                    .zip(b.iter())
                    .map(|(col_a, col_b)| {
                        col_a
                            .iter()
                            .zip(col_b.iter())
                            .map(|(x, y)| f(*x, *y))
                            .collect()
                    })
                    .collect(),
            ))
        }
        (Value::ComplexMatrix(a), Value::Matrix(b)) => {
            if a.len() != b.len()
                || a.first()
                    .map_or(true, |col| col.len() != b.first().map_or(0, |c| c.len()))
            {
                anyhow::bail!(
                    "Shape mismatch: {:?} vs {:?}",
                    Value::ComplexMatrix(a.clone()),
                    Value::Matrix(b.clone())
                )
            }
            let bc: Vec<Vec<num_complex::Complex<f64>>> = b
                .iter()
                .map(|col| col.iter().map(|x| num_complex::Complex::from(*x)).collect())
                .collect();
            Ok(Value::ComplexMatrix(
                a.iter()
                    .zip(bc.iter())
                    .map(|(col_a, col_b)| {
                        col_a
                            .iter()
                            .zip(col_b.iter())
                            .map(|(x, y)| f(*x, *y))
                            .collect()
                    })
                    .collect(),
            ))
        }
        _ => anyhow::bail!("Operator not supported for this operand type"),
    }
}

/// O(n³) complex matrix multiplication with dimension-checking.
///
/// Matrices are stored in column-major format: each inner Vec is a column.
/// `a` has `n_cols_a` columns each of length `n_rows_a`.
/// `b` has `n_cols_b` columns each of length `n_rows_b`.
/// For multiplication: `n_cols_a` must equal `n_rows_b`.
/// Result has `n_cols_b` columns each of length `n_rows_a`.
fn cmatmul(
    a: &[Vec<num_complex::Complex<f64>>],
    b: &[Vec<num_complex::Complex<f64>>],
) -> Result<Vec<Vec<num_complex::Complex<f64>>>> {
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
    let mut result = vec![vec![num_complex::Complex::new(0.0, 0.0); n_rows_a]; n_cols_b];
    for j in 0..n_cols_b {
        for k in 0..n_cols_a {
            let b_val = b[j][k];
            let a_col = &a[k];
            for i in 0..n_rows_a {
                result[j][i] = result[j][i] + a_col[i] * b_val;
            }
        }
    }
    Ok(result)
}
