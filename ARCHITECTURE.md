# Architecture

## Pipeline

```
input string
    │
    ▼
┌──────────┐
│  Lexer    │  Tokenises the input into a flat token stream.
│ (lexer.rs)│  Handles numbers (incl. scientific notation), identifiers,
│           │  operators, parentheses, brackets, apostrophe (transpose),
│           │  dot (broadcast), and the constants `pi` / `e` / `tau` / `phi`.
└────┬─────┘
     │ Vec<Token>
     ▼
┌──────────┐
│  Parser   │  Pratt parser that builds an AST from the token stream.
│(parser.rs)│  Binding-power table drives operator precedence and
│           │  associativity.  Supports unary minus, binary ops,
│           │  function calls, broadcast calls, vector/matrix literals,
│           │  transpose postfix, and element-wise operators.
└────┬─────┘
     │ Expr (AST)
     ▼
┌──────────┐
│Evaluator  │  Recursive-descent evaluation of the AST to a Value
│  (eval.rs)│  (Scalar, Vector, or Matrix).  Handles broadcasting,
│           │  matrix multiplication (column-major storage), transpose,
│           │  and element-wise ops.
└────┬─────┘
     │ Value (Scalar / Vector / Matrix)
     ▼
  output
```

## Source map

| File | Role |
|------|------|
| `main.rs` | CLI entry point: `clap` argument parsing, output formatting, CSV export |
| `lexer.rs` | Tokenizes raw text into `Token` stream; recognizes constants (`pi`, `e`, `tau`, `phi`), numbers (scientific notation), identifiers, operators |
| `parser.rs` | Pratt parser: builds `Expr` AST from tokens using binding-power precedence |
| `ast.rs` | AST node definitions (`Expr`) and binary operator enum (`BinOp`) |
| `eval.rs` | Top-level evaluator: dispatches sub-expressions, handles matrix construction, transpose, conjugate-transpose |
| `eval/scalar.rs` | Scalar math: binary ops with broadcasting, unary functions, complex arithmetic |
| `eval/matrix.rs` | Matrix operations: `matmul`, `matrix_pow`, `det`, `inv`, `tr`, `fnorm`, `norm`, `normalize`, `fnormalize`, `eye`, `diag` (and complex variants) |
| `eval/range.rs` | Range expression evaluation (`{start,stop,step}` → column vector) |
| `value.rs` | `Value` enum (`Scalar`, `Complex`, `Matrix`, `ComplexMatrix`) and `Display` impls (compact + pretty) |
| `function.rs` | Function Registry: single source of truth mapping function names to kinds |

## Design Decisions

| Decision | Rationale |
|----------|-----------|
| Only `clap`, `anyhow`, `num-complex`, and `csv` as dependencies | Keeps the binary small and the dependency tree shallow. |
| `pi`, `e`, `tau`, `phi` consumed as `Number` tokens | Avoids ambiguity between constants and function names. |
| `log` = base-10, `ln` = natural log | Matches common calculator conventions. |
| `^` is right-associative | `2^3^2` evaluates as `2^(3^2) = 512`, matching mathematical convention. |
| Single expression per invocation | `-f` reads the whole file as one expression (trimmed). |
| Division by zero → `inf` | Consistent with IEEE 754; no surprise panics. |
| Output format: `expression = result` | Easy to scan when evaluating many expressions. |
| Matrices are row-major in `Value` but column-major in `matmul` | Matrix literals read row-major (`[[1,2],[3,4]]`), but the multiplication algorithm treats inner `Vec`s as columns for cache efficiency. |

## Function Registry

All built-in functions are registered in one place (`function.rs`) with a `FunctionKind`:

| Kind | Functions | Behaviour |
|------|-----------|-----------|
| `MathUnary` | `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `sinh`, `cosh`, `tanh`, `exp`, `ln`, `log`, `sqrt`, `abs`, `floor`, `ceil`, `round` | Accept real/complex scalars; can be broadcast with `.` prefix |
| `Conj` | `conj` | Complex conjugate (no-op on reals); can be broadcast |
| `MatrixOp` | `det`, `tr`, `inv`, `fnorm`, `fnormalize`, `norm`, `normalize`, `eye`, `diag` | Matrix-only; **cannot** be broadcast |

Adding a new function requires one entry in the registry — the parser, evaluator, and broadcast guard all stay in sync automatically.

## Binding-power table

| Operator | Associativity | (left_bp, right_bp) |
|----------|---------------|---------------------|
| `+` `-`  | Left          | (1, 2)              |
| `*` `/` `%` | Left       | (3, 4)              |
| `^`      | Right         | (7, 6)              |
| unary `-`| Prefix        | r_bp = 10           |

## Postfix operators

| Operator | Meaning | Example |
|----------|---------|---------|
| `'`      | Transpose | `[1,2,3]'` |
| `"`      | Conjugate transpose (Hermitian) | `[1+2i, 3-4i]"` |
| `i`      | Imaginary unit postfix | `3i`, `-2i`, `1+2i` |

Postfix operators can be chained (`m'"`). For real matrices, `"` behaves identically to `'`.

