/// An abstract syntax tree node representing a mathematical expression.
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Complex(num_complex::Complex<f64>),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UnaryMinus(Box<Expr>),
    Call(String, Vec<Expr>),
    BroadcastCall(String, Vec<Expr>),
    Matrix(Vec<Vec<Expr>>),
    Range(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Transpose(Box<Expr>),
    ConjugateTranspose(Box<Expr>),
}

/// Binary operators supported by the expression evaluator, including
/// element-wise (dot-prefixed) variants for vector/matrix operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    DotAdd,
    DotSub,
    DotMul,
    DotDiv,
    DotPow,
    DotMod,
}
