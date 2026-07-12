/// The result type of expression evaluation: a scalar or matrix.
/// Vectors are represented as 1-column matrices (n×1).
/// Matrices are stored in row-major format: each inner `Vec` is a row.
#[derive(Debug, Clone)]
pub enum Value {
    Scalar(f64),
    Complex(num_complex::Complex<f64>),
    Matrix(Vec<Vec<f64>>),
    ComplexMatrix(Vec<Vec<num_complex::Complex<f64>>>),
}

impl Value {
    /// Return the inner `f64` if this is a `Scalar`, otherwise `None`.
    pub fn as_scalar(&self) -> Option<f64> {
        match self {
            Value::Scalar(n) => Some(*n),
            _ => None,
        }
    }

    /// Return the inner `Complex<f64>` if this is a `Complex`, otherwise `None`.
    pub fn as_complex(&self) -> Option<num_complex::Complex<f64>> {
        match self {
            Value::Complex(c) => Some(*c),
            _ => None,
        }
    }

    /// Return the number of rows in this value (1 for scalars and complexes).
    pub fn rows(&self) -> usize {
        match self {
            Value::Scalar(_) | Value::Complex(_) => 1,
            Value::Matrix(m) => m.len(),
            Value::ComplexMatrix(m) => m.len(),
        }
    }

    /// Return the number of columns in this value (1 for scalars and complexes).
    pub fn cols(&self) -> usize {
        match self {
            Value::Scalar(_) | Value::Complex(_) => 1,
            Value::Matrix(m) => m.first().map_or(0, |row| row.len()),
            Value::ComplexMatrix(m) => m.first().map_or(0, |col| col.len()),
        }
    }
}

/// Format a complex number for display, matching the `Value::Complex` format.
fn format_complex_display(c: &num_complex::Complex<f64>) -> String {
    let re = c.re;
    let im = c.im;
    if im == 0.0 {
        format!("{re}")
    } else if re == 0.0 {
        if im == 1.0 {
            "i".to_string()
        } else if im == -1.0 {
            "-i".to_string()
        } else {
            format!("{im}i")
        }
    } else if im > 0.0 {
        format!("{re} + {im}i")
    } else {
        let neg_im = -im;
        format!("{re} - {neg_im}i")
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Scalar(n) => write!(f, "{n}"),
            Value::Complex(c) => write!(f, "{}", format_complex_display(c)),
            Value::Matrix(m) => {
                if f.alternate() {
                    let n_rows = m.len();
                    let n_cols = m.first().map_or(0, |row| row.len());
                    writeln!(f, "\n{}x{} real matrix", n_cols, n_rows)?;
                    if n_cols == 0 || n_rows == 0 {
                        return Ok(());
                    }
                    // Transpose: iterate over columns (which become rows in the display)
                    // Calculate widths per display column (which is an original row)
                    let widths: Vec<usize> = (0..n_rows)
                        .map(|i| {
                            (0..n_cols)
                                .map(|j| format!("{}", m[i][j]).len())
                                .max()
                                .unwrap_or(0)
                        })
                        .collect();
                    for j in 0..n_cols {
                        if j > 0 {
                            writeln!(f)?;
                        }
                        let line: Vec<String> = (0..n_rows)
                            .map(|i| {
                                let x = m[i][j];
                                format!("{:>width$}", x, width = widths[i])
                            })
                            .collect();
                        write!(f, "{}", line.join(" "))?;
                    }
                    Ok(())
                } else {
                    let rows: Vec<String> = m
                        .iter()
                        .map(|row| {
                            let inner: Vec<String> = row.iter().map(|x| format!("{x}")).collect();
                            format!("[{}]", inner.join(", "))
                        })
                        .collect();
                    write!(f, "[{}]", rows.join(", "))
                }
            }
            Value::ComplexMatrix(m) => {
                if f.alternate() {
                    let n_rows = m.len();
                    let n_cols = m.first().map_or(0, |row| row.len());
                    writeln!(f, "\n{}x{} complex matrix", n_cols, n_rows)?;
                    if n_cols == 0 || n_rows == 0 {
                        return Ok(());
                    }
                    // Transpose: iterate over columns (which become rows in the display)
                    // Calculate widths per display column (which is an original row)
                    let widths: Vec<usize> = (0..n_rows)
                        .map(|i| {
                            (0..n_cols)
                                .map(|j| format_complex_display(&m[i][j]).len())
                                .max()
                                .unwrap_or(0)
                        })
                        .collect();
                    for j in 0..n_cols {
                        if j > 0 {
                            writeln!(f)?;
                        }
                        let line: Vec<String> = (0..n_rows)
                            .map(|i| {
                                let s = format_complex_display(&m[i][j]);
                                format!("{:>width$}", s, width = widths[i])
                            })
                            .collect();
                        write!(f, "{}", line.join(" "))?;
                    }
                    Ok(())
                } else {
                    let rows: Vec<String> = m
                        .iter()
                        .map(|col| {
                            let inner: Vec<String> =
                                col.iter().map(format_complex_display).collect();
                            format!("[{}]", inner.join(", "))
                        })
                        .collect();
                    write!(f, "[{}]", rows.join(", "))
                }
            }
        }
    }
}
