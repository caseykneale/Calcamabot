use anyhow::{anyhow, Result};

/// A lexical token produced by the lexer from the input string.
///
/// `Number` tokens carry the parsed `f64` value. The special identifiers
/// `pi` and `e` are converted to `Number` tokens at this stage.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Apostrophe,
    DoubleQuote,
    Dot,
    ImaginaryUnit,
    Eof,
}

/// Tokenise an input string into a flat stream of `Token`s.
///
/// Handles numbers (including scientific notation), identifiers (with `pi`/`e`
/// constant substitution), operators, parentheses, brackets, apostrophe
/// (transpose), and dot (broadcast) tokens. Appends an `Eof` sentinel at the end.
pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        // Skip whitespace
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Numbers (including scientific notation)
        if chars[i].is_ascii_digit()
            || (chars[i] == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
        {
            let start = i;
            // Integer part
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            // Decimal part
            if i < chars.len() && chars[i] == '.' {
                i += 1;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }
            // Exponent part
            if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                i += 1;
                if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                    i += 1;
                }
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let num_str: String = chars[start..i].iter().collect();
            let num: f64 = num_str
                .parse()
                .map_err(|_| anyhow!("Invalid number: {}", num_str))?;
            tokens.push(Token::Number(num));
            continue;
        }

        // Identifiers (function names, constants)
        if chars[i].is_ascii_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let ident: String = chars[start..i].iter().collect();
            let lower = ident.to_lowercase();

            // Handle constants pi, e, tau, phi, and imaginary unit i
            match lower.as_str() {
                "pi" => {
                    tokens.push(Token::Number(std::f64::consts::PI));
                }
                "e" => {
                    tokens.push(Token::Number(std::f64::consts::E));
                }
                "tau" => {
                    tokens.push(Token::Number(std::f64::consts::TAU));
                }
                "phi" => {
                    tokens.push(Token::Number(1.618_033_988_749_895));
                }
                "i" => {
                    tokens.push(Token::ImaginaryUnit);
                }
                _ => {
                    tokens.push(Token::Ident(lower));
                }
            }
            continue;
        }

        // Single-character operators and parens
        match chars[i] {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '%' => tokens.push(Token::Percent),
            '^' => tokens.push(Token::Caret),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            ',' => tokens.push(Token::Comma),
            '\'' => tokens.push(Token::Apostrophe),
            '"' => tokens.push(Token::DoubleQuote),
            '.' => tokens.push(Token::Dot),
            _ => return Err(anyhow!("Unexpected character: '{}'", chars[i])),
        }
        i += 1;
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}
