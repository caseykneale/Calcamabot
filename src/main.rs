use anyhow::{Context, Result};
use calcamabot::export::{ensure_csv_ext, write_csv};
use calcamabot::parse_and_eval;
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Evaluate mathematical expressions")]
struct Cli {
    #[command(flatten)]
    input: InputSource,

    /// Export result to a CSV file (adds .csv extension if omitted)
    #[arg(long = "csv")]
    csv: Option<String>,

    /// Pretty-print matrices with aligned columns
    #[arg(short = 'p', long = "pretty")]
    pretty: bool,
}

#[derive(clap::Args)]
#[group(required = true, multiple = false)]
struct InputSource {
    /// A string expression to evaluate
    #[arg(short = 'e', long = "expression")]
    #[arg(allow_hyphen_values = true)]
    expr: Option<String>,

    /// Path to a file containing an expression
    #[arg(short = 'f', long = "file")]
    file: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.pretty && cli.csv.is_some() {
        anyhow::bail!(
            "Cannot pretty print while exporting to CSV. These arguments are mutually exclusive."
        );
    }

    let expression = match (cli.input.expr, cli.input.file) {
        (Some(expr), _) => expr,
        (_, Some(path)) => std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path))?,
        (None, None) => anyhow::bail!("No input provided"),
    };

    let result = parse_and_eval(&expression).context("Failed to evaluate expression")?;

    if cli.pretty {
        println!("{} = {:#}", expression.trim(), result);
    } else {
        println!("{} = {}", expression.trim(), result);
    }

    if let Some(path) = &cli.csv {
        write_csv(&result, path)?;
        eprintln!("Exported result to {}", ensure_csv_ext(path));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_csv_ext_no_extension() {
        assert_eq!(ensure_csv_ext("output"), "output.csv");
    }

    #[test]
    fn test_ensure_csv_ext_already_csv() {
        assert_eq!(ensure_csv_ext("output.csv"), "output.csv");
    }

    #[test]
    fn test_ensure_csv_ext_with_path() {
        assert_eq!(ensure_csv_ext("/tmp/data"), "/tmp/data.csv");
    }

    #[test]
    fn test_write_csv_scalar() {
        let result = parse_and_eval("42").unwrap();
        write_csv(&result, "/tmp/test_csv_scalar").unwrap();
        let content = std::fs::read_to_string("/tmp/test_csv_scalar.csv").unwrap();
        assert_eq!(content, "x_0\n42\n");
    }

    #[test]
    fn test_write_csv_matrix() {
        let result = parse_and_eval("[[1,2],[3,4]]").unwrap();
        write_csv(&result, "/tmp/test_csv_matrix").unwrap();
        let content = std::fs::read_to_string("/tmp/test_csv_matrix.csv").unwrap();
        assert_eq!(content, "x_0,x_1\n1,2\n3,4\n");
    }

    #[test]
    fn test_write_csv_vector() {
        let result = parse_and_eval("[1,2,3]").unwrap();
        write_csv(&result, "/tmp/test_csv_vector").unwrap();
        let content = std::fs::read_to_string("/tmp/test_csv_vector.csv").unwrap();
        assert_eq!(content, "x_0,x_1,x_2\n1,2,3\n");
    }

    #[test]
    fn test_write_csv_range() {
        let result = parse_and_eval("{0,3,2}").unwrap();
        write_csv(&result, "/tmp/test_csv_range").unwrap();
        let content = std::fs::read_to_string("/tmp/test_csv_range.csv").unwrap();
        assert_eq!(content, "x_0,x_1\n0,2\n");
    }

    #[test]
    fn test_write_csv_preserves_csv_extension() {
        let result = parse_and_eval("3.14").unwrap();
        write_csv(&result, "/tmp/test_csv_preserve.csv").unwrap();
        assert!(std::path::Path::new("/tmp/test_csv_preserve.csv").exists());
        let content = std::fs::read_to_string("/tmp/test_csv_preserve.csv").unwrap();
        assert_eq!(content, "x_0\n3.14\n");
    }
}
