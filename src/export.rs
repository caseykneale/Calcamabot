use anyhow::{Context, Result};
use std::fs::File;

use crate::value::Value;

/// Ensure the path ends with `.csv`.
pub fn ensure_csv_ext(path: &str) -> String {
    if path.ends_with(".csv") {
        path.to_owned()
    } else {
        format!("{}.csv", path)
    }
}

/// Write a `Value` to a CSV file.
///
/// Scalars produce a single row with one column (`x_0`).
/// Matrices produce one row per row of the matrix, with columns `x_0`, `x_1`, …
pub fn write_csv(value: &Value, path: &str) -> Result<()> {
    let csv_path = ensure_csv_ext(path);
    let file =
        File::create(&csv_path).with_context(|| format!("Failed to create file: {}", csv_path))?;
    let mut wtr = csv::Writer::from_writer(file);

    if value.as_scalar().is_some() {
        eprintln!("Warning: result is a scalar, CSV export will contain a single row/column");
    }

    match value {
        Value::Scalar(n) => {
            wtr.write_record(["x_0"])?;
            wtr.write_record(&[n.to_string()])?;
        }
        Value::Complex(c) => {
            wtr.write_record(["re", "im"])?;
            wtr.write_record(&[c.re.to_string(), c.im.to_string()])?;
        }
        Value::ComplexMatrix(m) => {
            let num_rows = m.len();
            let num_cols = m.first().map_or(0, |col| col.len());
            let headers: Vec<String> = (0..num_cols)
                .flat_map(|j| vec![format!("x_{j}_re"), format!("x_{j}_im")])
                .collect();
            wtr.write_record(&headers)?;
            for i in 0..num_rows {
                let mut row = Vec::new();
                for j in 0..num_cols {
                    let c = m[j][i];
                    row.push(c.re.to_string());
                    row.push(c.im.to_string());
                }
                wtr.write_record(&row)?;
            }
        }
        Value::Matrix(m) => {
            let n_cols = m.first().map_or(0, |row| row.len());
            let headers: Vec<String> = (0..n_cols).map(|j| format!("x_{}", j)).collect();
            wtr.write_record(&headers)?;
            for row in m {
                let cells: Vec<String> = row.iter().map(|v| v.to_string()).collect();
                wtr.write_record(&cells)?;
            }
        }
    }

    wtr.flush()?;
    Ok(())
}
