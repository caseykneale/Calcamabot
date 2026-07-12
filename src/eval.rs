pub mod matrix;
pub mod range;
pub mod scalar;

use anyhow::Result;

use crate::ast::Expr;
use crate::eval::range::create_range;
use crate::value::Value;

/// Evaluate an `Expr` AST node to a `Value` (scalar, vector, or matrix).
///
/// Recursively walks the AST, applying binary operators with broadcasting,
/// unary/broadcast functions, matrix multiplication, and matrix power.
pub fn eval(expr: &Expr) -> Result<Value> {
    match expr {
        Expr::Number(n) => Ok(Value::Scalar(*n)),
        Expr::Complex(c) => Ok(Value::Complex(*c)),
        Expr::BinOp(left, op, right) => {
            let l = eval(left)?;
            let r = eval(right)?;
            scalar::apply_binop(*op, l, r)
        }
        Expr::UnaryMinus(inner) => match eval(inner)? {
            Value::Scalar(n) => Ok(Value::Scalar(-n)),
            Value::Complex(c) => Ok(Value::Complex(-c)),
            Value::Matrix(m) => Ok(Value::Matrix(
                m.iter()
                    .map(|row| row.iter().map(|x| -x).collect())
                    .collect(),
            )),
            Value::ComplexMatrix(m) => Ok(Value::ComplexMatrix(
                m.iter()
                    .map(|col| col.iter().map(|c| -c).collect())
                    .collect(),
            )),
        },
        Expr::Call(name, args) => {
            let arg = if args.len() == 1 {
                eval(&args[0])?
            } else {
                anyhow::bail!("Function {} expects 1 argument", name);
            };
            apply_unary_fn(name, arg)
        }
        Expr::BroadcastCall(name, args) => {
            let arg = if args.len() == 1 {
                eval(&args[0])?
            } else {
                anyhow::bail!("Broadcast function {} expects 1 argument", name);
            };
            apply_broadcast_fn(name, arg)
        }
        Expr::Matrix(rows) => {
            let mut has_complex = false;
            // First pass: evaluate and check for complex elements
            let evaluated: Vec<Vec<Value>> = rows
                .iter()
                .map(|row| row.iter().map(eval).collect::<Result<Vec<_>, _>>())
                .collect::<Result<Vec<_>, _>>()?;
            for row in &evaluated {
                for v in row {
                    if matches!(v, Value::Complex(_) | Value::ComplexMatrix(_)) {
                        has_complex = true;
                        break;
                    }
                }
                if has_complex {
                    break;
                }
            }
            if has_complex {
                // Build ComplexMatrix in row-major format (same as real matrices)
                // evaluated[i][j] = element at row i, column j
                let mut rows_complex: Vec<Vec<num_complex::Complex<f64>>> =
                    Vec::with_capacity(evaluated.len());
                for row in &evaluated {
                    let complex_row: Vec<num_complex::Complex<f64>> = row
                        .iter()
                        .map(|v| match v {
                            Value::Scalar(n) => num_complex::Complex::new(*n, 0.0),
                            Value::Complex(c) => *c,
                            _ => unreachable!(),
                        })
                        .collect();
                    rows_complex.push(complex_row);
                }
                Ok(Value::ComplexMatrix(rows_complex))
            } else {
                let mut vals = Vec::with_capacity(evaluated.len());
                for row in evaluated {
                    let mut row_vals = Vec::with_capacity(row.len());
                    for v in row {
                        match v {
                            Value::Scalar(n) => row_vals.push(n),
                            _ => unreachable!(),
                        }
                    }
                    vals.push(row_vals);
                }
                Ok(Value::Matrix(vals))
            }
        }
        Expr::Range(start, stop, step) => {
            let start_val = eval(start)?
                .as_scalar()
                .ok_or_else(|| anyhow::anyhow!("Range start must be scalar"))?;
            let stop_val = eval(stop)?
                .as_scalar()
                .ok_or_else(|| anyhow::anyhow!("Range stop must be scalar"))?;
            let step_val = match step {
                Some(s) => eval(s)?
                    .as_scalar()
                    .ok_or_else(|| anyhow::anyhow!("Range step must be scalar"))?,
                None => {
                    if start_val < stop_val {
                        1.0
                    } else {
                        -1.0
                    }
                }
            };
            create_range(start_val, stop_val, step_val)
        }
        Expr::Transpose(inner) => match eval(inner)? {
            Value::Scalar(_) | Value::Complex(_) => anyhow::bail!("Cannot transpose a scalar"),
            Value::Matrix(m) => {
                let rows = m.len();
                let cols = m.first().map_or(0, |r| r.len());
                let mut transposed = vec![vec![0.0; rows]; cols];
                for i in 0..rows {
                    for j in 0..cols {
                        transposed[j][i] = m[i][j];
                    }
                }
                Ok(Value::Matrix(transposed))
            }
            Value::ComplexMatrix(m) => {
                let rows = m.len();
                let cols = m.first().map_or(0, |r| r.len());
                let mut transposed = vec![vec![num_complex::Complex::new(0.0, 0.0); rows]; cols];
                for i in 0..rows {
                    for j in 0..cols {
                        transposed[j][i] = m[i][j];
                    }
                }
                Ok(Value::ComplexMatrix(transposed))
            }
        },
        Expr::ConjugateTranspose(inner) => match eval(inner)? {
            Value::Scalar(n) => Ok(Value::Scalar(n)),
            Value::Complex(c) => Ok(Value::Complex(c.conj())),
            Value::Matrix(m) => {
                // Real matrix: conjugate is identity, so just transpose
                let rows = m.len();
                let cols = m.first().map_or(0, |r| r.len());
                let mut transposed = vec![vec![0.0; rows]; cols];
                for i in 0..rows {
                    for j in 0..cols {
                        transposed[j][i] = m[i][j];
                    }
                }
                Ok(Value::Matrix(transposed))
            }
            Value::ComplexMatrix(m) => {
                // Conjugate transpose: swap rows/cols and conjugate each element
                let rows = m.len();
                let cols = m.first().map_or(0, |r| r.len());
                let mut result = vec![vec![num_complex::Complex::new(0.0, 0.0); rows]; cols];
                for i in 0..rows {
                    for j in 0..cols {
                        result[j][i] = m[i][j].conj();
                    }
                }
                Ok(Value::ComplexMatrix(result))
            }
        },
    }
}

/// Apply a unary function: matrix ops go to matrix module,
/// scalar math funcs go to scalar module.
fn apply_unary_fn(name: &str, arg: Value) -> Result<Value> {
    if let Some(crate::function::FunctionKind::MatrixOp) = crate::function::function_kind(name) {
        matrix::apply_matrix_unary(name, arg)
    } else {
        scalar::apply_scalar_unary(name, arg)
    }
}

/// Apply a unary math function element-wise across a `Value`.
///
/// Accepts scalars, vectors, and matrices. Matrix-specific operations
/// (`det`, `tr`, `inv`, `norm`) are not broadcastable and will error.
fn apply_broadcast_fn(name: &str, arg: Value) -> Result<Value> {
    // Matrix-specific operations cannot be broadcast
    if matches!(
        crate::function::function_kind(name),
        Some(crate::function::FunctionKind::MatrixOp)
    ) {
        anyhow::bail!("Function {} cannot be broadcast", name);
    }
    // Special handling for real/imag: they change the value type
    if name == "real" {
        return apply_real_broadcast(arg);
    }
    if name == "imag" {
        return apply_imag_broadcast(arg);
    }
    match arg {
        Value::Scalar(n) => scalar::apply_scalar_unary(name, Value::Scalar(n)),
        Value::Complex(c) => scalar::apply_scalar_unary(name, Value::Complex(c)),
        Value::Matrix(m) => {
            // Promote to complex matrix if any element would need complex promotion
            if m.iter().any(|row| {
                row.iter().any(|&x| scalar::needs_complex_promotion(name, x))
            }) {
                let complex_m: Vec<Vec<num_complex::Complex<f64>>> = m
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|&x| num_complex::Complex::new(x, 0.0))
                            .collect()
                    })
                    .collect();
                return apply_broadcast_fn(name, Value::ComplexMatrix(complex_m));
            }
            let result: Vec<Vec<f64>> = m
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|x| scalar::apply_unary_fn_scalar(name, *x))
                        .collect()
                })
                .collect();
            Ok(Value::Matrix(result))
        }
        Value::ComplexMatrix(m) => {
            let result: Vec<Vec<num_complex::Complex<f64>>> = m
                .iter()
                .map(|col| {
                    col.iter()
                        .map(|c| apply_broadcast_element(name, *c))
                        .collect()
                })
                .collect();
            Ok(Value::ComplexMatrix(result))
        }
    }
}

/// Apply a broadcastable unary function to a single complex number.
fn apply_broadcast_element(name: &str, c: num_complex::Complex<f64>) -> num_complex::Complex<f64> {
    match name {
        "conj" => c.conj(),
        "sin" => c.sin(),
        "cos" => c.cos(),
        "tan" => c.tan(),
        "asin" => c.asin(),
        "acos" => c.acos(),
        "atan" => c.atan(),
        "exp" => c.exp(),
        "ln" => c.ln(),
        "log" => c.log10(),
        "sqrt" => c.sqrt(),
        "abs" => num_complex::Complex::new(c.norm(), 0.0),
        "floor" => num_complex::Complex::new(c.re.floor(), c.im.floor()),
        "ceil" => num_complex::Complex::new(c.re.ceil(), c.im.ceil()),
        "round" => num_complex::Complex::new(c.re.round(), c.im.round()),
        "sinh" => c.sinh(),
        "cosh" => c.cosh(),
        "tanh" => c.tanh(),
        _ => panic!(
            "Broadcast function {} does not accept complex arguments",
            name
        ),
    }
}

/// Broadcast `real()` across a `Value`, returning a real `Matrix`.
///
/// - Scalar/Complex → `Scalar` (the real part)
/// - Matrix → `Matrix` (identity)
/// - ComplexMatrix → `Matrix` (extract real parts as f64)
fn apply_real_broadcast(arg: Value) -> Result<Value> {
    match arg {
        Value::Scalar(n) => Ok(Value::Scalar(n)),
        Value::Complex(c) => Ok(Value::Scalar(c.re)),
        Value::Matrix(m) => Ok(Value::Matrix(m)),
        Value::ComplexMatrix(m) => {
            let result: Vec<Vec<f64>> = m
                .iter()
                .map(|row| row.iter().map(|c| c.re).collect())
                .collect();
            Ok(Value::Matrix(result))
        }
    }
}

/// Broadcast `imag()` across a `Value`, returning a `Complex` scalar or `ComplexMatrix`.
///
/// - Scalar → `Complex(0, 0)`
/// - Complex → `Complex(0, im)`
/// - Matrix → `ComplexMatrix` (pure imaginary zeros)
/// - ComplexMatrix → `ComplexMatrix` (extract imaginary parts as pure imaginary)
fn apply_imag_broadcast(arg: Value) -> Result<Value> {
    match arg {
        Value::Scalar(_) => Ok(Value::Complex(num_complex::Complex::new(0.0, 0.0))),
        Value::Complex(c) => Ok(Value::Complex(num_complex::Complex::new(0.0, c.im))),
        Value::Matrix(m) => {
            let result: Vec<Vec<num_complex::Complex<f64>>> = m
                .iter()
                .map(|row| row.iter().map(|_| num_complex::Complex::new(0.0, 0.0)).collect())
                .collect();
            Ok(Value::ComplexMatrix(result))
        }
        Value::ComplexMatrix(m) => {
            let result: Vec<Vec<num_complex::Complex<f64>>> = m
                .iter()
                .map(|row| row.iter().map(|c| num_complex::Complex::new(0.0, c.im)).collect())
                .collect();
            Ok(Value::ComplexMatrix(result))
        }
    }
}
