pub mod ast;
pub mod eval;
pub mod export;
pub mod function;
pub mod lexer;
pub mod parser;
pub mod value;

use anyhow::Result;

/// Parse and evaluate a mathematical expression string, returning the Value result.
pub fn parse_and_eval(input: &str) -> Result<value::Value> {
    let tokens = lexer::tokenize(input)?;
    let ast = parser::parse(&tokens)?;
    eval::eval(&ast)
}

#[cfg(test)]
mod tests;
