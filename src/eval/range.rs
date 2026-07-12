use crate::value::Value;
use anyhow::Result;

pub fn create_range(start_val: f64, stop_val: f64, step_val: f64) -> Result<Value> {
    if step_val == 0.0 {
        anyhow::bail!("Range step cannot be zero");
    }
    let epsilon = 1e-9;
    let mut values = Vec::new();
    if step_val > 0.0 {
        let mut v = start_val;
        while v <= stop_val + epsilon * (stop_val.abs().max(1.0)) {
            values.push(v);
            v += step_val;
        }
    } else {
        let mut v = start_val;
        while v >= stop_val - epsilon * (stop_val.abs().max(1.0)) {
            values.push(v);
            v += step_val;
        }
    }

    Ok(Value::Matrix(vec![values]))
}
