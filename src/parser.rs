use anyhow::Result;

use crate::ast::{BinOp, Expr};
use crate::lexer::Token;
use num_complex::Complex;

// ── Binding powers ───────────────────────────────────────────────────
// (left_bp, right_bp)
//
// Left-associative ops use left_bp < right_bp.
// Right-associative ops use left_bp > right_bp so the second occurrence
// recurses with a higher min_bp and binds tighter.
const UNARY_MINUS_BP: u8 = 10;

fn infix_binding_power(op: &Token) -> Option<(u8, u8)> {
    match op {
        Token::Plus | Token::Minus | Token::Dot => Some((1, 2)),
        Token::Star | Token::Slash | Token::Percent => Some((3, 4)),
        Token::Caret => Some((7, 6)), // right-associative
        _ => None,
    }
}

/// Pratt parser that builds an `Expr` AST from a token slice.
///
/// Operator precedence and associativity are driven by the `infix_binding_power`
/// table. Supports unary minus, binary ops, function calls, broadcast calls,
/// vector/matrix literals, transpose postfix, and element-wise operators.
pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<()> {
        if self.peek() == expected {
            self.advance();
            Ok(())
        } else {
            anyhow::bail!("Expected {:?}, found {:?}", expected, self.peek())
        }
    }

    /// Parse a primary expression (number, function call, parenthesised expr, bracket literals, unary minus).
    fn parse_primary(&mut self, min_bp: u8) -> Result<Expr> {
        let expr = match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Expr::Number(n)
            }
            Token::Ident(name) => {
                self.advance();
                if let Some(bc) = self.try_parse_broadcast_call(&name)? {
                    return Ok(bc);
                }
                self.parse_function_call(&name)?
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr(0)?;
                self.expect(&Token::RParen)?;
                expr
            }
            Token::LBracket => self.parse_bracket_literal()?,
            Token::LBrace => self.parse_range_literal()?,
            Token::Minus if min_bp < UNARY_MINUS_BP => {
                self.advance();
                let inner = self.parse_primary(UNARY_MINUS_BP)?;
                return Ok(Expr::UnaryMinus(Box::new(inner)));
            }
            Token::ImaginaryUnit => {
                self.advance();
                return Ok(Expr::Complex(Complex::new(0.0, 1.0)));
            }
            other => anyhow::bail!("Unexpected token: {:?}", other),
        };
        self.parse_postfix(expr)
    }

    /// If the current position is `name . (`, parse as a broadcast call and return `Some(expr)`.
    fn try_parse_broadcast_call(&mut self, name: &str) -> Result<Option<Expr>> {
        if crate::function::function_kind(name).is_none() || !matches!(self.peek(), Token::Dot) {
            return Ok(None);
        }
        let next = self.tokens.get(self.pos + 1).cloned();
        if !matches!(next.as_ref(), Some(Token::LParen)) {
            return Ok(None);
        }
        self.advance(); // consume Dot
        self.expect(&Token::LParen)?;
        let mut args = Vec::new();
        if !matches!(self.peek(), Token::RParen) {
            args.push(self.parse_expr(0)?);
        }
        self.expect(&Token::RParen)?;
        Ok(Some(Expr::BroadcastCall(name.to_string(), args)))
    }

    /// Parse a standard function call `name(args)`.
    fn parse_function_call(&mut self, name: &str) -> Result<Expr> {
        if crate::function::function_kind(name).is_none() {
            anyhow::bail!("Unknown identifier: {}", name)
        }
        self.expect(&Token::LParen)?;
        let mut args = Vec::new();
        if !matches!(self.peek(), Token::RParen) {
            args.push(self.parse_expr(0)?);
            while matches!(self.peek(), Token::Comma) {
                self.advance();
                args.push(self.parse_expr(0)?);
            }
        }
        self.expect(&Token::RParen)?;
        Ok(Expr::Call(name.to_string(), args))
    }

    /// Parse a bracket literal: matrix `[[...], [...]]` or vector `[...]`.
    fn parse_bracket_literal(&mut self) -> Result<Expr> {
        self.advance(); // consume first [
        if matches!(self.peek(), Token::LBracket) {
            let mut rows = Vec::new();
            loop {
                self.expect(&Token::LBracket)?;
                let row = self.parse_comma_exprs()?;
                self.expect(&Token::RBracket)?;
                rows.push(row);
                if !matches!(self.peek(), Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect(&Token::RBracket)?;
            Ok(Expr::Matrix(rows))
        } else {
            let elems = self.parse_comma_exprs()?;
            self.expect(&Token::RBracket)?;
            Ok(Expr::Matrix(vec![elems]))
        }
    }

    /// Parse a range literal `{start,stop}` or `{start,stop,step}`.
    fn parse_range_literal(&mut self) -> Result<Expr> {
        self.advance(); // consume {
        let start = self.parse_expr(0)?;
        self.expect(&Token::Comma)?;
        let stop = self.parse_expr(0)?;
        let step = if matches!(self.peek(), Token::Comma) {
            self.advance();
            Some(Box::new(self.parse_expr(0)?))
        } else {
            None
        };
        self.expect(&Token::RBrace)?;
        Ok(Expr::Range(Box::new(start), Box::new(stop), step))
    }

    /// Apply postfix operators to an expression: `'` (transpose), `"` (conjugate transpose), and `i` (imaginary unit).
    fn parse_postfix(&mut self, mut expr: Expr) -> Result<Expr> {
        loop {
            if matches!(self.peek(), Token::Apostrophe) {
                self.advance();
                expr = Expr::Transpose(Box::new(expr));
            } else if matches!(self.peek(), Token::DoubleQuote) {
                self.advance();
                expr = Expr::ConjugateTranspose(Box::new(expr));
            } else {
                break;
            }
        }
        if matches!(self.peek(), Token::ImaginaryUnit) {
            self.advance();
            expr = match expr {
                Expr::Number(n) => Expr::Complex(Complex::new(0.0, n)),
                Expr::UnaryMinus(inner) => {
                    if let Expr::Number(n) = inner.as_ref() {
                        Expr::Complex(Complex::new(0.0, -n))
                    } else {
                        anyhow::bail!("Unexpected expression before imaginary unit");
                    }
                }
                _ => anyhow::bail!("Imaginary unit requires a numeric coefficient"),
            };
        }
        Ok(expr)
    }

    /// Parse a comma-separated list of expressions (does NOT consume surrounding brackets).
    fn parse_comma_exprs(&mut self) -> Result<Vec<Expr>> {
        let mut elems = Vec::new();
        if !matches!(self.peek(), Token::RBracket) {
            elems.push(self.parse_expr(0)?);
            while matches!(self.peek(), Token::Comma) {
                self.advance();
                elems.push(self.parse_expr(0)?);
            }
        }
        Ok(elems)
    }

    /// Pratt-style expression parsing.
    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr> {
        let mut lhs = self.parse_primary(min_bp)?;

        loop {
            // Check for dot-prefixed element-wise operator: .+, .-, .*, ./, .^
            if matches!(self.peek(), Token::Dot) {
                let next = self.tokens.get(self.pos + 1).cloned();
                if let Some(next_op) = next {
                    let binop = match next_op {
                        Token::Plus => Some(BinOp::DotAdd),
                        Token::Minus => Some(BinOp::DotSub),
                        Token::Star => Some(BinOp::DotMul),
                        Token::Slash => Some(BinOp::DotDiv),
                        Token::Caret => Some(BinOp::DotPow),
                        Token::Percent => Some(BinOp::DotMod),
                        _ => None,
                    };
                    if let Some(op) = binop {
                        self.advance(); // consume Dot
                        self.advance(); // consume operator
                        let rhs = self.parse_expr(min_bp)?;
                        lhs = Expr::BinOp(Box::new(lhs), op, Box::new(rhs));
                        continue;
                    }
                }
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(self.peek()) {
                if l_bp < min_bp {
                    break;
                }
                let op = self.advance();
                let rhs = self.parse_expr(r_bp)?;
                let binop = match op {
                    Token::Plus => BinOp::Add,
                    Token::Minus => BinOp::Sub,
                    Token::Star => BinOp::Mul,
                    Token::Slash => BinOp::Div,
                    Token::Caret => BinOp::Pow,
                    Token::Percent => BinOp::Mod,
                    _ => unreachable!(),
                };
                lhs = Expr::BinOp(Box::new(lhs), binop, Box::new(rhs));
            } else {
                break;
            }
        }

        Ok(lhs)
    }
}

/// Parse a token slice into an `Expr` AST.
///
/// Entry point for the parser. Verifies that the entire token stream is
/// consumed (i.e. no trailing tokens after the expression).
pub fn parse(tokens: &[Token]) -> Result<Expr> {
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expr(0)?;
    if !matches!(parser.peek(), Token::Eof) {
        anyhow::bail!("Unexpected token after expression: {:?}", parser.peek())
    }
    Ok(expr)
}
