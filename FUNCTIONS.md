# Function Reference

## Supported syntax

- **Numbers**: integers, decimals, scientific notation (`1.5e-3`)
- **Complex literals**: `3i`, `2+3i`, `2-5i`, `i` (imaginary unit)
- **Operators**: `+`, `-`, `*`, `/`, `^`, `%`
- **Element-wise operators**: `.+`, `.-`, `.*`, `./`, `.^`, `.%`
- **Parentheses**: `(`, `)`
- **Brackets**: `[` (start vector/matrix), `]` (end vector/matrix)
- **Curly braces**: `{` (start range), `}` (end range)
- **Comma**: `,` (separator in vectors/matrices and ranges)
- **Apostrophe**: `'` (transpose postfix, e.g. `[1,2,3]'`)
- **Double-quote**: `"` (conjugate transpose postfix, e.g. `[1+2i, 3-4i]"`)
- **Dot**: `.` (broadcast operator, e.g. `sqrt.(x)`, `.^`, `.*`)
- **Unary minus**: `-3`, `2 * -4`
- **Functions**: `sin`, `cos`, `tan`, `sinh`, `cosh`, `tanh`, `exp`, `ln`, `log` (base 10), `sqrt`, `abs`, `asin`, `acos`, `atan`, `floor`, `ceil`, `round`, `conj`, `real`, `imag`, `det`, `tr`, `inv`, `fnorm`, `norm`, `normalize`, `fnormalize`, `eye`, and `diag`.
- **Broadcast functions**: `sin.(x)`, `sqrt.([1,4,9])`, `conj.([1+2i, 3-4i])`, etc.
- **Constants**: `pi`, `e`, `tau`, `phi`
- **Vector literals**: `[1, 2, 3]` (3×1 column vector)
- **Matrix literals**: `[[1, 2], [3, 4]]` (2×2 matrix)
- **Complex matrix literals**: `[[1+2i, 3-4i]]` (mixed real/complex entries)
- **Transpose**: `[1, 2, 3]'` converts 3×1 vector to 1×3 row matrix
- **Conjugate transpose**: `[1+2i, 3-4i]"` returns the Hermitian transpose

---

## Complex Numbers

math-cli supports full complex number arithmetic using the [`num-complex`](https://docs.rs/num-complex) crate.

### Complex Literals

| Expression | Result |
|------------|--------|
| `i` | `i` (imaginary unit) |
| `3i` | `3i` |
| `2 + 3i` | `2 + 3i` |
| `2 - 5i` | `2 - 5i` |
| `-2i` | `-2i` |
| `1 + i` | `1 + i` |

Complex literals can appear anywhere a real number can: in expressions, inside vectors, and inside matrices.

```math-cli
2 + 3i                    →  2 + 3i
(1 + 2i) * (3 - 4i)      →  11 + 2i
sin(1 + 2i)               →  3.165... + 3.088...i
[1+2i, 3-4i]              →  [[1 + 2i, 3 - 4i]]
```

### Complex Arithmetic

All standard arithmetic operators work on complex numbers:

| Expression | Result |
|------------|--------|
| `(1 + 2i) + (3 - 4i)` | `4 - 2i` |
| `(1 + 2i) - (3 - 4i)` | `-2 + 6i` |
| `(1 + 2i) * (3 - 4i)` | `11 + 2i` |
| `(1 + 2i) / (3 - 4i)` | `-0.2 + 0.2i` |
| `(1 + 2i) ^ 2` | `-3 + 4i` |

Element-wise operators also work on complex numbers:

```
[1+2i, 3-4i] .+ [5, 6+7i]    →  [6 + 2i, 9 + 3i]
[1+2i, 3-4i] .* [5, 6+7i]    →  [5 + 10i, 46 - 3i]
```

### Complex Transcendental Functions

All transcendental functions extend naturally to complex arguments via `num-complex` trait implementations:

| Function | Example | Result |
|----------|---------|--------|
| `sin` | `sin(1 + 2i)` | `3.165... + 3.088...i` |
| `cos` | `cos(1 + 2i)` | `-1.033... - 2.472...i` |
| `tan` | `tan(1 + 2i)` | `0.035... + 1.018...i` |
| `exp` | `exp(1 + 2i)` | `-1.131... + 2.472...i` |
| `ln` | `ln(1 + 2i)` | `1.150... + 1.107...i` |
| `sqrt` | `sqrt(-1)` | `i` |
| `abs` | `abs(3 + 4i)` | `5` (returns a real scalar — the modulus) |
| `sinh` | `sinh(1 + 2i)` | `3.051... + 3.649...i` |
| `cosh` | `cosh(1 + 2i)` | `-1.042... - 2.040...i` |
| `tanh` | `tanh(1 + 2i)` | `1.036... + 0.054...i` |
| `log` | `log(1 + 2i)` | `0.206... + 0.480...i` (base-10 log) |
| `floor` | `floor(1.7 + 2.3i)` | `1 + 2i` (floor applied to real and imaginary parts) |
| `ceil` | `ceil(1.2 + 2.7i)` | `2 + 3i` |
| `round` | `round(1.4 + 2.6i)` | `1 + 3i` |

### The `conj` Function

Returns the complex conjugate. For real inputs, returns the input unchanged.

```
conj(3 + 4i)        →  3 - 4i
conj(2 - 5i)        →  2 + 5i
conj(3i)            →  -3i
conj(5)             →  5
conj.([1+2i, 3-4i]) →  [1-2i, 3+4i]
```

### The `real` and `imag` Functions

Extract the real or imaginary part of a complex number. Both are broadcastable via the dot prefix.

| Function | Description | Example |
|----------|-------------|---------|
| `real` | Returns the real part as a real scalar | `real(3 + 4i)` → `3` |
| `imag` | Returns the imaginary part as a pure-imaginary complex number | `imag(3 + 4i)` → `4i` |

For real inputs, `real` returns the input unchanged and `imag` returns `0i`.

```
real(3 + 4i)        →  3
imag(3 + 4i)        →  4i
real(5)             →  5
imag(5)             →  0i
real.([1+2i, 3-4i]) →  [1, 3]
imag.([1+2i, 3-4i]) →  [2i, -4i]
```

Note: `real` and `imag` cannot be applied directly to matrices — they must be broadcast with the dot prefix:

```
real([[1,2],[3,4]])    →  Error (use real.([[1,2],[3,4]]) instead)
real.([[1+2i, 3-4i]])  →  [[1, 3]]
```

### Conjugate Transpose (`"`)

The double-quote postfix operator computes the **conjugate transpose** (Hermitian transpose) of a matrix or vector. For real matrices, it behaves identically to the regular transpose (`'`).

```
[1+2i, 3-4i]"    →  [[1-2i], [3+4i]]   (2×1 column → 1×2 row, conjugated)
[[1+2i, 3-4i]]"  →  [[1-2i], [3+4i]]   (2×1 → 1×2)
[[1, 2], [3, 4]]" → [[1, 3], [2, 4]]   (real matrix: same as transpose)
```

This is especially useful for computing inner products of complex vectors:

```
[1+2i, 3-4i]" * [1+2i, 3-4i]    →  30
```

### Complex Matrix Operations

Complex matrices (containing `Complex` entries) are automatically detected when any element is complex. All matrix operations extend to the complex domain:

#### Complex Matrix Multiplication

```
[[1+2i]] * [[3-4i]]    →  [[11 + 2i]]
[[1+2i, 3-4i]] * [[5], [6+7i]]  = [[5 + 10i, 15 - 20i], [-8 + 19i, 46 - 3i]]
```

Mixed real and complex matrices multiply correctly — a real matrix times a complex matrix yields a complex matrix.

#### Complex Matrix Functions

| Function | Description | Example |
|----------|-------------|---------|
| `tr` | Trace (sum of diagonal, returns real part) | `tr([[1+2i, 0], [0, 3-4i]])` → `4` |
| `fnorm` | Frobenius norm (real, non-negative) | `fnorm([[3+4i]])` → `5` |
| `norm` | Column L2 norms (returns real matrix) | `norm([[3+4i, 0]])` → `[[5]]` |
| `normalize` | Normalize each column to unit L2 norm | `normalize([[3+4i]])` → `[[0.6+0.8i]]` |
| `fnormalize` | Divide all elements by Frobenius norm | `fnormalize([[3+4i]])` → `[[0.6+0.8i]]` |

Note: `det`, `tr`, and `inv` on complex matrices return the real part of the result (since the determinant or trace of a complex matrix may be complex).

### Mixed Real/Complex Operations

When a real operand is combined with a complex operand, the result is complex:

```
1 + [1+2i, 3-4i]              →  [2+2i, 4-4i]
[1+2i, 3-4i] * 2              →  [2+4i, 6-8i]
[1+2i, 3-4i] / 2              →  [0.5+1i, 1.5-2i]
[[1, 2], [3, 4]] * [[1+2i, 5]] = [[16 + 2i, 22 + 4i]]
```

---

## Vectors, Matrices & Broadcasting

### Vector & Matrix Literals

```
[1, 2, 3]          → 3×1 column vector
[[1, 2], [3, 4]]   → 2×2 matrix
```

### Transpose Operator (`'`)

```
[1, 2, 3]'    →  [[1, 2, 3]]   (1×3 row matrix)
[[1, 2], [3, 4]]'  →  [[1, 3], [2, 4]]
```

### Broadcasting (automatic)

Scalar operations on vectors/matrices apply element-wise:

```
2 + [1, 2, 3]       →  [3, 4, 5]
[1, 2, 3] * 2       →  [2, 4, 6]
[[1, 2], [3, 4]] + 10  →  [[11, 12], [13, 14]]
```

### Element-wise Operators

```
[1, 2, 3] .+ [4, 5, 6]    →  [5, 7, 9]
[1, 2, 3] .^ 2             →  [1, 4, 9]
2 .^ [1, 2, 3]             →  [2, 4, 8]
[1, 2, 3] .* [4, 5, 6]     →  [4, 10, 18]
```

### Matrix Multiplication (`*`)

```
[[1, 2], [3, 4]] * [[5, 6], [7, 8]]  →  [[19, 22], [43, 50]]
[1, 2, 3]' * [1, 2, 3]               →  14  (dot product)
```

Dimension checking: `A (m×n) * B (n×p)` requires A's column count to equal B's row count.

### Matrix Power (`^`)

```
[[1, 1], [0, 1]] ^ 3  →  [[1, 3], [0, 1]]   (repeated matrix multiplication)
```

### Matrix Functions

Matrix-specific operations that return a scalar or matrix. These **cannot** be broadcast with the dot prefix — they operate on the entire matrix.

#### Determinant (`det`)

Returns the determinant of a square matrix.

```
det([[1, 2], [3, 4]])     →  -2
det([[1, 0], [0, 1]])     →  1
det([[2, 0], [0, 3]])     →  6
```

#### Trace (`tr`)

Returns the trace (sum of diagonal elements) of a square matrix.

```
tr([[1, 2], [3, 4]])     →  5
tr([[1, 0, 0], [0, 2, 0], [0, 0, 3]])  →  6
```

#### Inverse (`inv`)

Returns the inverse of a square matrix. Errors on singular or non-square matrices.

```
inv([[1, 2], [3, 4]])    →  [[-2, 1], [1.5, -0.5]]
inv([[1, 0], [0, 1]])    →  [[1, 0], [0, 1]]   (identity)
[[1, 2], [3, 4]] * inv([[1, 2], [3, 4]]) = [[1, 0], [0, 1]]
```

Verify: `A * inv(A)` always yields the identity matrix (for invertible `A`).

#### Frobenius Norm (`fnorm`)

Returns the Frobenius norm (square root of the sum of squared elements) of any matrix.

```
fnorm([[3, 4], [0, 0]])   →  5
fnorm([[1, 2], [3, 4]])   →  5.477
```

#### Euclidean Column Normalization (`norm`)

Obtain the L2 norm of each column of a matrix.

```
norm([[3, 0], [0, 4]]) = [[3, 4]]
norm([[1, 2], [3, 4]]) = [[2.23606797749979, 5]]
```

### Identity Matrix (`eye`)

Creates an n×n identity matrix. The argument must be a non-negative integer (or a complex number with zero imaginary part and integer real part).

```
eye(2)    →  [[1, 0], [0, 1]]
eye(3)    →  [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
eye(1)    →  [[1]]
eye(0)    →  []   (empty matrix)
```

Useful for constructing identity matrices in expressions:

```
eye(3) * [1, 2, 3]    →  [1, 2, 3]   (identity leaves vectors unchanged)
```

### Diagonal Matrix / Extract Diagonal (`diag`)

A dual-purpose function that behaves differently depending on its input:

- **Scalar input** → returns a 1×1 matrix containing that scalar.
- **Vector input** → creates a diagonal matrix with the vector values on the diagonal.
- **Matrix input** → extracts the diagonal elements as a vector.

```
diag(5)                        →  [[5]]
diag([1, 2, 3])                →  [[1, 0, 0], [0, 2, 0], [0, 0, 3]]
diag([[1, 2], [3, 4]])         →  [1, 4]   (extracts diagonal)
diag([[2, 0], [0, 3]])         →  [2, 3]
```

Works with complex values:

```
diag(2+3i)                     →  [[2+3i]]
diag([1+2i, 3-4i])             →  [[1+2i, 0], [0, 3-4i]]
diag([[1+2i, 0], [0, 3-4i]])   →  [1+2i, 3-4i]
```

### Broadcast Functions

Apply a function element-wise to a vector or matrix:

```
sqrt.([4, 9, 16])              →  [2, 3, 4]
abs.([-3, 5, -2])              →  [3, 5, 2]
exp.([0, 1, 2])                →  [1, 2.718..., 7.389...]
```

### Range Expressions `{start,stop}` / `{start,stop,step}`

Curly-brace ranges generate a vector (1×n column) from `start` to `stop` with an optional `step`. The step defaults to `+1` (ascending) or `-1` (descending) when omitted.

```
{0, 3}              →  [0, 1, 2, 3]
{0, 3, 2}           →  [0, 2]
{3, 0}              →  [3, 2, 1, 0]
{0, pi, pi/4}       →  [0, π/4, π/2, 3π/4, π]
{0, 1, 0.3}         →  [0, 0.3, 0.6, 0.9]
```

Ranges work seamlessly with matrix multiplication. For example, a 5×5 Hilbert matrix can be built from a range:

```
{0,4} * {0,4}'      →  [[0, 0, 0, 0, 0],
                        [0, 1, 2, 3, 4],
                        [0, 2, 4, 6, 8],
                        [0, 3, 6, 9, 12],
                        [0, 4, 8, 12, 16]]
```

This is the outer product of `[0,1,2,3,4]` with itself — a 5×5 matrix where each element `M[i,j] = i * j`.
