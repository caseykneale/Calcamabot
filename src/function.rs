/// Kind of a registered built-in function.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FunctionKind {
    /// Scalar math function: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, exp, ln, log, sqrt, abs, floor, ceil, round, real, imag
    MathUnary,
    /// Conjugate: works on scalars, complexes, matrices, can be broadcast
    Conj,
    /// Matrix-only operation: det, tr, inv, fnorm, fnormalize, norm, normalize, eye, diag
    MatrixOp,
}

/// Look up a function name in the registry.
/// Returns `None` if the name is not a known function.
pub fn function_kind(name: &str) -> Option<FunctionKind> {
    match name {
        "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sinh" | "cosh" | "tanh" | "exp"
        | "ln" | "log" | "sqrt" | "abs" | "floor" | "ceil" | "round" | "real" | "imag" => {
            Some(FunctionKind::MathUnary)
        }
        "conj" => Some(FunctionKind::Conj),
        "det" | "tr" | "inv" | "fnorm" | "fnormalize" | "norm" | "normalize" | "eye" | "diag" => {
            Some(FunctionKind::MatrixOp)
        }
        _ => None,
    }
}
