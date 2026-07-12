use core::f64;

use crate::parse_and_eval;

use crate::tests::common::{expect_scalar, expect_vector};

// Scalar tests

#[test]
fn test_basic_addition() {
    expect_scalar(&parse_and_eval("2 + 3").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_basic_subtraction() {
    expect_scalar(&parse_and_eval("10 - 4").unwrap(), 6.0, 1e-9);
}

#[test]
fn test_basic_multiplication() {
    expect_scalar(&parse_and_eval("3 * 7").unwrap(), 21.0, 1e-9);
}

#[test]
fn test_basic_division() {
    expect_scalar(&parse_and_eval("15 / 3").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_operator_precedence_mul_over_add() {
    expect_scalar(&parse_and_eval("2 + 3 * 4").unwrap(), 14.0, 1e-9);
}

#[test]
fn test_operator_precedence_div_over_add() {
    expect_scalar(&parse_and_eval("10 + 6 / 2").unwrap(), 13.0, 1e-9);
}

#[test]
fn test_parentheses() {
    expect_scalar(&parse_and_eval("(2 + 3) * 4").unwrap(), 20.0, 1e-9);
}

#[test]
fn test_nested_parentheses() {
    expect_scalar(&parse_and_eval("((2 + 3) * (4 + 1))").unwrap(), 25.0, 1e-9);
}

#[test]
fn test_unary_minus() {
    expect_scalar(&parse_and_eval("-3 + 5").unwrap(), 2.0, 1e-9);
}

#[test]
fn test_unary_minus_in_expression() {
    expect_scalar(&parse_and_eval("2 * -4").unwrap(), -8.0, 1e-9);
}

#[test]
fn test_exponentiation_right_associative() {
    expect_scalar(&parse_and_eval("2 ^ 3 ^ 2").unwrap(), 512.0, 1e-9);
}

#[test]
fn test_exponentiation_basic() {
    expect_scalar(&parse_and_eval("2 ^ 10").unwrap(), 1024.0, 1e-9);
}

#[test]
fn test_sin_zero() {
    expect_scalar(&parse_and_eval("sin(0)").unwrap(), 0.0, 1e-9);
}

#[test]
fn test_cos_zero() {
    expect_scalar(&parse_and_eval("cos(0)").unwrap(), 1.0, 1e-9);
}

#[test]
fn test_tan_zero() {
    expect_scalar(&parse_and_eval("tan(0)").unwrap(), 0.0, 1e-9);
}

#[test]
fn test_sin_pi_over_2() {
    expect_scalar(&parse_and_eval("sin(pi / 2)").unwrap(), 1.0, 1e-9);
}

#[test]
fn test_exp_zero() {
    expect_scalar(&parse_and_eval("exp(0)").unwrap(), 1.0, 1e-9);
}

#[test]
fn test_exp_one() {
    expect_scalar(
        &parse_and_eval("exp(1)").unwrap(),
        std::f64::consts::E,
        1e-9,
    );
}

#[test]
fn test_ln_exp_identity() {
    expect_scalar(&parse_and_eval("ln(exp(1))").unwrap(), 1.0, 1e-9);
}

#[test]
fn test_log_base10() {
    expect_scalar(&parse_and_eval("log(100)").unwrap(), 2.0, 1e-9);
}

#[test]
fn test_log_base10_1000() {
    expect_scalar(&parse_and_eval("log(1000)").unwrap(), 3.0, 1e-9);
}

#[test]
fn test_pi_constant() {
    expect_scalar(&parse_and_eval("pi").unwrap(), std::f64::consts::PI, 1e-9);
}

#[test]
fn test_e_constant() {
    expect_scalar(&parse_and_eval("e").unwrap(), std::f64::consts::E, 1e-9);
}

#[test]
fn test_tau_constant() {
    expect_scalar(&parse_and_eval("tau").unwrap(), std::f64::consts::TAU, 1e-9);
}

#[test]
fn test_phi_constant() {
    expect_scalar(&parse_and_eval("phi").unwrap(), 1.618_033_988_749_895, 1e-9);
}

#[test]
fn test_tau_equals_2pi() {
    expect_scalar(
        &parse_and_eval("tau / 2").unwrap(),
        std::f64::consts::PI,
        1e-9,
    );
}

#[test]
fn test_phi_squared_minus_phi_equals_one() {
    let result = parse_and_eval("phi * phi - phi").unwrap();
    expect_scalar(&result, 1.0, 1e-9);
}

#[test]
fn test_2_times_pi() {
    expect_scalar(
        &parse_and_eval("2 * pi").unwrap(),
        2.0 * std::f64::consts::PI,
        1e-9,
    );
}

#[test]
fn test_decimal_numbers() {
    expect_scalar(&parse_and_eval("1.5 + 2.5").unwrap(), 4.0, 1e-9);
}

#[test]
fn test_scientific_notation() {
    expect_scalar(&parse_and_eval("1e2").unwrap(), 100.0, 1e-9);
}

#[test]
fn test_scientific_notation_negative_exp() {
    expect_scalar(&parse_and_eval("1e-3").unwrap(), 0.001, 1e-12);
}

#[test]
fn test_complex_expression() {
    expect_scalar(
        &parse_and_eval("2 * sin(pi / 2) + 3 * cos(0)").unwrap(),
        5.0,
        1e-9,
    );
}

#[test]
fn test_error_unknown_function() {
    assert!(parse_and_eval("foo(2)").is_err());
}

#[test]
fn test_error_missing_rparen() {
    assert!(parse_and_eval("sin(2").is_err());
}

#[test]
fn test_error_trailing_tokens() {
    assert!(parse_and_eval("2 +").is_err());
}

#[test]
fn test_error_empty_input() {
    assert!(parse_and_eval("").is_err());
}

#[test]
fn test_error_invalid_char() {
    assert!(parse_and_eval("2 @ 3").is_err());
}

#[test]
fn test_division_by_zero_returns_inf() {
    let result = parse_and_eval("1 / 0").unwrap();
    expect_scalar(&result, f64::INFINITY, 1e-9);
}

#[test]
fn test_nested_functions() {
    expect_scalar(&parse_and_eval("exp(ln(5))").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_mixed_ops() {
    expect_scalar(&parse_and_eval("2 * 3 + 4 * 5").unwrap(), 26.0, 1e-9);
}

// Phase 1: sqrt, abs, asin, acos, atan, floor, ceil, round

#[test]
fn test_sqrt_4() {
    expect_scalar(&parse_and_eval("sqrt(4)").unwrap(), 2.0, 1e-9);
}

#[test]
fn test_sqrt_2() {
    expect_scalar(
        &parse_and_eval("sqrt(2)").unwrap(),
        f64::consts::SQRT_2,
        1e-9,
    );
}

#[test]
fn test_sqrt_nested() {
    expect_scalar(&parse_and_eval("sqrt(sqrt(16))").unwrap(), 2.0, 1e-9);
}

#[test]
fn test_abs_positive() {
    expect_scalar(&parse_and_eval("abs(5)").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_abs_negative() {
    expect_scalar(&parse_and_eval("abs(-5)").unwrap(), 5.0, 1e-9);
}

#[test]
fn test_abs_zero() {
    expect_scalar(&parse_and_eval("abs(0)").unwrap(), 0.0, 1e-9);
}

#[test]
fn test_asin_zero() {
    expect_scalar(&parse_and_eval("asin(0)").unwrap(), 0.0, 1e-9);
}

#[test]
fn test_asin_half() {
    expect_scalar(
        &parse_and_eval("asin(0.5)").unwrap(),
        std::f64::consts::FRAC_PI_6,
        1e-9,
    );
}

#[test]
fn test_acos_zero() {
    expect_scalar(
        &parse_and_eval("acos(0)").unwrap(),
        std::f64::consts::FRAC_PI_2,
        1e-9,
    );
}

#[test]
fn test_acos_one() {
    expect_scalar(&parse_and_eval("acos(1)").unwrap(), 0.0, 1e-9);
}

#[test]
fn test_atan_zero() {
    expect_scalar(&parse_and_eval("atan(0)").unwrap(), 0.0, 1e-9);
}

#[test]
fn test_atan_one() {
    expect_scalar(
        &parse_and_eval("atan(1)").unwrap(),
        std::f64::consts::FRAC_PI_4,
        1e-9,
    );
}

#[test]
fn test_floor_positive() {
    expect_scalar(&parse_and_eval("floor(3.7)").unwrap(), 3.0, 1e-9);
}

#[test]
fn test_floor_negative() {
    expect_scalar(&parse_and_eval("floor(-3.2)").unwrap(), -4.0, 1e-9);
}

#[test]
fn test_ceil_positive() {
    expect_scalar(&parse_and_eval("ceil(3.2)").unwrap(), 4.0, 1e-9);
}

#[test]
fn test_ceil_negative() {
    expect_scalar(&parse_and_eval("ceil(-3.7)").unwrap(), -3.0, 1e-9);
}

#[test]
fn test_round_up() {
    expect_scalar(&parse_and_eval("round(3.5)").unwrap(), 4.0, 1e-9);
}

#[test]
fn test_round_down() {
    expect_scalar(&parse_and_eval("round(3.4)").unwrap(), 3.0, 1e-9);
}

#[test]
fn test_round_negative() {
    expect_scalar(&parse_and_eval("round(-2.5)").unwrap(), -3.0, 1e-9);
}

#[test]
fn test_nested_function_composition() {
    expect_scalar(&parse_and_eval("abs(floor(-2.7))").unwrap(), 3.0, 1e-9);
}

#[test]
fn test_sqrt_in_expression() {
    expect_scalar(&parse_and_eval("sqrt(9) + 1").unwrap(), 4.0, 1e-9);
}

#[test]
fn test_round_in_expression() {
    expect_scalar(&parse_and_eval("round(3.14 * 100)").unwrap(), 314.0, 1e-9);
}

// Phase 1b: Hyperbolic functions

#[test]
fn test_sinh_zero() {
    expect_scalar(&parse_and_eval("sinh(0)").unwrap(), 0.0, 1e-9);
}

#[test]
fn test_cosh_zero() {
    expect_scalar(&parse_and_eval("cosh(0)").unwrap(), 1.0, 1e-9);
}

#[test]
fn test_tanh_zero() {
    expect_scalar(&parse_and_eval("tanh(0)").unwrap(), 0.0, 1e-9);
}

#[test]
fn test_sinh_one() {
    // sinh(1) = (e - 1/e) / 2
    let e = std::f64::consts::E;
    let expected = (e - 1.0 / e) / 2.0;
    expect_scalar(&parse_and_eval("sinh(1)").unwrap(), expected, 1e-9);
}

#[test]
fn test_cosh_one() {
    // cosh(1) = (e + 1/e) / 2
    let e = std::f64::consts::E;
    let expected = (e + 1.0 / e) / 2.0;
    expect_scalar(&parse_and_eval("cosh(1)").unwrap(), expected, 1e-9);
}

#[test]
fn test_tanh_one() {
    // tanh(1) = sinh(1) / cosh(1)
    let e = std::f64::consts::E;
    let sinh1 = (e - 1.0 / e) / 2.0;
    let cosh1 = (e + 1.0 / e) / 2.0;
    expect_scalar(&parse_and_eval("tanh(1)").unwrap(), sinh1 / cosh1, 1e-9);
}

// Phase 3: Modulo operator

#[test]
fn test_modulo_scalar() {
    expect_scalar(&parse_and_eval("10 % 3").unwrap(), 1.0, 1e-9);
}

#[test]
fn test_modulo_zero_remainder() {
    expect_scalar(&parse_and_eval("12 % 4").unwrap(), 0.0, 1e-9);
}

#[test]
fn test_modulo_negative_dividend() {
    // rem_euclid: (-7) mod 3 = 2
    expect_scalar(&parse_and_eval("-7 % 3").unwrap(), 2.0, 1e-9);
}

#[test]
fn test_modulo_negative_divisor() {
    // rem_euclid returns non-negative result: 7 mod (-3) = 1
    expect_scalar(&parse_and_eval("7 % -3").unwrap(), 1.0, 1e-9);
}

#[test]
fn test_modulo_elementwise() {
    expect_vector(
        &parse_and_eval("[10, 11, 12] .% 3").unwrap(),
        &[1.0, 2.0, 0.0],
        1e-9,
    );
}

#[test]
fn test_modulo_in_expression() {
    // 10 % 3 + 5 = 6
    expect_scalar(&parse_and_eval("10 % 3 + 5").unwrap(), 6.0, 1e-9);
}

#[test]
fn test_modulo_precedence_over_add() {
    // 2 + 10 % 3 should be 2 + 1 = 3, not 12 % 3 = 0
    expect_scalar(&parse_and_eval("2 + 10 % 3").unwrap(), 3.0, 1e-9);
}

#[test]
fn test_modulo_precedence_with_mul() {
    // 2 * 10 % 3 should be (2*10) % 3 = 20 % 3 = 2
    expect_scalar(&parse_and_eval("2 * 10 % 3").unwrap(), 2.0, 1e-9);
}

#[test]
fn test_modulo_division_by_zero() {
    // Division by zero in modulo should produce an error or NaN
    let result = parse_and_eval("5 % 0");
    assert!(result.is_err() || result.unwrap().as_scalar().is_some_and(f64::is_nan));
}

// Phase 5: Range expressions {start,stop} / {start,stop,step}

#[test]
fn test_range_basic() {
    expect_vector(
        &parse_and_eval("{0,3}").unwrap(),
        &[0.0, 1.0, 2.0, 3.0],
        1e-9,
    );
}

#[test]
fn test_range_with_step() {
    expect_vector(&parse_and_eval("{0,3,2}").unwrap(), &[0.0, 2.0], 1e-9);
}

#[test]
fn test_range_descending() {
    expect_vector(
        &parse_and_eval("{3,0}").unwrap(),
        &[3.0, 2.0, 1.0, 0.0],
        1e-9,
    );
}

#[test]
fn test_range_explicit_negative_step() {
    expect_vector(
        &parse_and_eval("{5,0,-1}").unwrap(),
        &[5.0, 4.0, 3.0, 2.0, 1.0, 0.0],
        1e-9,
    );
}

#[test]
fn test_range_float() {
    expect_vector(
        &parse_and_eval("{0,1,0.3}").unwrap(),
        &[0.0, 0.3, 0.6, 0.9],
        1e-9,
    );
}

#[test]
fn test_range_pi() {
    let expected: Vec<f64> = (0..=4)
        .map(|i| i as f64 * std::f64::consts::PI / 4.0)
        .collect();
    expect_vector(&parse_and_eval("{0,pi,pi/4}").unwrap(), &expected, 1e-9);
}

#[test]
fn test_range_empty() {
    expect_vector(&parse_and_eval("{3,0,1}").unwrap(), &[], 1e-9);
}

#[test]
fn test_range_step_zero_error() {
    assert!(parse_and_eval("{0,3,0}").is_err());
}

#[test]
fn test_range_in_expression() {
    expect_vector(
        &parse_and_eval("{0,2} + 10").unwrap(),
        &[10.0, 11.0, 12.0],
        1e-9,
    );
}

#[test]
fn test_range_display() {
    let result = parse_and_eval("{0,2}").unwrap();
    assert_eq!(format!("{}", result), "[[0, 1, 2]]");
}

// Scalar display

#[test]
fn test_display_scalar() {
    let result = parse_and_eval("42").unwrap();
    assert_eq!(format!("{}", result), "42");
}

// Scalar error tests

#[test]
fn test_error_unary_on_vector() {
    assert!(parse_and_eval("sin([1,2])").is_err());
}

#[test]
fn test_dot_after_non_function() {
    expect_vector(
        &parse_and_eval("2 .+ [1, 2, 3]").unwrap(),
        &[3.0, 4.0, 5.0],
        1e-9,
    );
}
