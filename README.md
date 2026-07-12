# Calcamabot

A small Rust CLI application that evaluates mathematical expressions using a Pratt parser, with support for **complex numbers** and **matrices**.

## Documentation

| **[Architecture](ARCHITECTURE.md)** | **[Function Reference](FUNCTIONS.md)** |
|---|---|
| Pipeline, design decisions, source map | Full syntax and function reference |

## Why?

I wanted to assess some strengths and weaknesses of local models in a greenfield project. This is an extension of a project I did many years ago to learn about Pratt parsers. I jokingly called that project [`Calcamabob`](https://github.com/caseykneale/Calcamabob), it wasn't a serious project. Calcamabob only did basic algebra with support for some transcendental functions. I wanted to see if I could use specification driven agentic development to build a more capable calculator using only local models. I wrote a [report about the experience](EXPERIENCE.md) if anyone is interested. This also isn't a serious project.

### What I think this calculator does right

- It handles complex math reasonably well. For example, `sqrt(-1)` yeilds `i`. `i` is the same as `1i`. Multiplying a real by a complex matrix doesn't require a cast or anything, it just does what I expect. Some transcendentals also just do what I would expect, which is nice. I personally find complex numbers in Julia and Python to be kind of ugly for simple questions.
- I like that it has ranges, pretty print, and csv export. I use these types of things to generate artifical data a lot. This turns 20 lines of code in a lot of languages into 1 mostly readable shell call. So that's nice.
- It's small and fast for small calculations. 1.5MB small. An example run for a small calculation involving matrices used 2.92 MB and completed in less than 0.00 seconds. Thats hundreds of times smaller and faster than Julia equivalent for a one-off. The key is that it has a low overhead.

### What I think it does wrong

- I don't know if I love the decision I made to have `[1,2,3]` be a 3x1 and `[[1,2,3],[4,5,6]]` be a 3x2 matrix, and have that notation be the only way to construct a matrix. Oh well.
- Although it handles complex numbers pretty well, there is a gap in the parser/evaluator where `(1 + 2i) * (3 - 4i)` is not the same thing as `1+2i * 3-4i` and thats confusing. To be fair, Julia handles this the same way so use parenthesis when you have too.
- It does not have good all around performance. It's good enough, you won't feel small matrix multiplications or anything, but please don't feed this into a production API or something that needs a warm cache, does heavy lifting, etc.
- The code isn't great. I tried driving the tools to write cleaner code but I hit my artifical dead-line for the project. See my [experience report](EXPERIENCE.md) for more details.

## Project goals
- Provide a dependency-light command-line tool for evaluating math expressions with a simple syntax.
- Demonstrate a Pratt parser implementation in Rust with clean separation of concerns (lexer → parser → evaluator).
- Support common mathematical functions and constants out of the box.
- Support complex numbers (scalars and matrices) with full arithmetic, transcendental functions, and matrix operations.
- Give clear, actionable error messages when an expression is malformed.


## Quick start

### CLI Arguments
```bash
# Evaluate a string expression
math-cli --expression "2 + 3 * sin(pi / 2)"
# Pretty-print a matrix
math-cli -e "[[1,2],[3,4]]" --pretty
# Export matrices to CSV for plotting, or whatever you want.
math-cli -e "[[1,2],[3,4]] * [[1,2],[3,4]]" --csv output
```

## Capabilities at a glance

| Feature | Example |
|---------|---------|
| Real arithmetic | `2 + 3 * 4` = `14` |
| Complex numbers | `(1 + 2i) * (3 - 4i)` = `11 + 2i` |
| Vectors & matrices | `[[1, 2], [3, 4]]` (2×2 matrix) |
| Matrix multiplication | `[1, 2, 3]' * [1, 2, 3]` = `14` |
| Broadcasting | `2 + [1, 2, 3]` = `[3, 4, 5]` |
| Element-wise ops | `[1, 2, 3] .^ 2` = `[1, 4, 9]` |
| Constants | `pi`, `e`, `tau`, `phi` |
| Math functions | `sin`, `cos`, `ln`, `sqrt`, `abs`, `real`, `imag`, … (auto-return complex values for out-of-domain real inputs) |
| Matrix functions | `det`, `inv`, `tr`, `eye`, `diag`, … |
| Ranges | `{0, pi, pi/4}` = `[0, π/4, π/2, 3π/4, π]` |

Full syntax reference: [`COMMANDS.md`](COMMANDS.md)
How it works under the hood: [`ARCHITECTURE.md`](ARCHITECTURE.md)

### Examples
#### Operator precedence is obeyed
```bash
math-cli -e "8/2*(1+1)-2"
8/2*(1+1)-2 = 6
```

#### Evaluate complex numbers
```bash
math-cli -e "cos(i) + 42 + 1i"
cos(i) + 42 + 1i = 43.54308063481524 + 1i
```
```bash
math-cli -e "abs(1+1i * conj(1+1i))"
abs(1+1i * conj(1+1i)) = 2.23606797749979
```

#### Work with matrices
```bash
math-cli -e "inv(diag(diag([1,2] * [1,2]')))"
diag(diag([1,2] * [1.5,2.3]')) = 
2x2 real matrix
1    0
0 0.25
```
```bash
math-cli -e "[[1,2],[3,4]] * inv([[1,2], [3,4]])" -p
[[1,2],[3,4]] * inv([[1,2], [3,4]]) = 
2x2 real matrix
1 0
0 1
```

#### Use ranges and broadcasting to save effort
Unary functions with a `.` before their argument broadcast over collections.
Ranges are defined by `{start,stop,increment}` where `increment` is optional. 
```bash
math-cli run -- -e "cos.({0,tau,pi/2})"
cos.({0,tau,pi/2}) = [[1, 0.00000000000000006123233995736766, -1, -0.00000000000000018369701987210297, 1]]
```

Ranges act like vectors.
```bash
math-cli run -- -e "{0,5}*{0,-5}'" --pretty
{0,5}*{0,-5}' = 
6x6 real matrix
0  0   0   0   0   0
0 -1  -2  -3  -4  -5
0 -2  -4  -6  -8 -10
0 -3  -6  -9 -12 -15
0 -4  -8 -12 -16 -20
0 -5 -10 -15 -20 -25
```

## Building & Testing

```bash
cargo build
cargo test
```

## Contributing
I likely will not accept or review issues or PR's. This was mostly an experiment. I'll possibly use the tool here and there to reduce my dependence on some other's but I am not trying to draw in even a small crowd.

### Possible ToDo's
- [ ] **Aggregation Operations** Support `sum()`, `mean()`, `median()`, `std()`, `min()`, `max()` for collections, and when broadcasted columnwise.
- [ ] **Multiple expressions per file** (one per line, evaluated independently, allowing for an `ans` keyword or actual variable assignment)
- [ ] **History / REPL mode**: interactive prompt with expression history
- [ ] **Load CSV's as variables**: Could be useful to use this on some real data, but its already grown a bit past what it needs to do. Performance might not be good.


## AI Usage Disclosure
All of the code written in this repository was done so with a LLM guided by a person. The markdown files contain content both written by a person and an agentic workflow. The `EXPERIENCE.md` file is entirely human written, this file is mostly human written and the others are mostly LLM written.
