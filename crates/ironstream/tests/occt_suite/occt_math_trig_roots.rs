//! Ported OpenCascade unit tests — `math_TrigonometricFunctionRoots`.
//!
//! Faithful Rust port of OCCT's `math_TrigonometricFunctionRoots_Test.cxx`
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`). Same
//! numeric inputs, same expected values, same tolerances.
//!
//! `math_TrigonometricFunctionRoots(A,B,C,D,E,inf,sup)` →
//! [`MathTrigonometricFunctionRoots::new`]; the 2- and 3-arg
//! `(D,E,...)` / `(C,D,E,...)` constructors map to
//! [`new_sine`](MathTrigonometricFunctionRoots::new_sine) and
//! [`new_cosine_sine`](MathTrigonometricFunctionRoots::new_cosine_sine).
//! OCCT's 1-indexed `Value(i)` maps to `value(i)`.

use ironstream::math_trig_roots::MathTrigonometricFunctionRoots;

const PI: f64 = std::f64::consts::PI;
const TOLERANCE: f64 = 1.0e-6;

/// EXPECT_NEAR(a, b, tol).
fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol}), diff {}",
        (a - b).abs()
    );
}

// ====================== FullEquationBasic ======================

#[test]
fn full_equation_basic() {
    // cos^2(x) + c*cos(x) = 0 => cos(x)(cos(x) + c) = 0
    let (a, b, c, d, e) = (1.0, 0.0, 1.0, 0.0, 0.0);
    let solver = MathTrigonometricFunctionRoots::new(a, b, c, d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() > 0);
}

// ====================== LinearSineOnly ======================

#[test]
fn linear_sine_only() {
    // sin(x) - 0.5 = 0 => sin(x) = 0.5 => x = PI/6, 5*PI/6
    let (d, e) = (1.0, -0.5);
    let solver = MathTrigonometricFunctionRoots::new_sine(d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() > 0);

    if solver.nb_solutions() >= 1 {
        let x1 = solver.value(1);
        near(x1.sin(), 0.5, TOLERANCE);
    }
}

// ====================== LinearCosineAndSine ======================

#[test]
fn linear_cosine_and_sine() {
    // cos(x) + sin(x) = 0 => tan(x) = -1 => x = 3*PI/4, 7*PI/4
    let (c, d, e) = (1.0, 1.0, 0.0);
    let solver = MathTrigonometricFunctionRoots::new_cosine_sine(c, d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() > 0);

    if solver.nb_solutions() >= 1 {
        let x1 = solver.value(1);
        near(x1.cos() + x1.sin(), 0.0, TOLERANCE);
    }
}

// ====================== PureCosineEquation ======================

#[test]
fn pure_cosine_equation() {
    // cos(x) = 0 => x = PI/2, 3*PI/2
    let (c, d, e) = (1.0, 0.0, 0.0);
    let solver = MathTrigonometricFunctionRoots::new_cosine_sine(c, d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() >= 1);

    if solver.nb_solutions() >= 1 {
        let x1 = solver.value(1);
        near(x1.cos().abs(), 0.0, TOLERANCE);
    }
}

// ====================== PureSineEquation ======================

#[test]
fn pure_sine_equation() {
    // sin(x) = 0 => x = 0, PI, 2*PI
    let (d, e) = (1.0, 0.0);
    let solver = MathTrigonometricFunctionRoots::new_sine(d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() >= 2);

    if solver.nb_solutions() >= 1 {
        let x1 = solver.value(1);
        near(x1.sin().abs(), 0.0, TOLERANCE);
    }
}

// ====================== NoSolution ======================

#[test]
fn no_solution() {
    // sin(x) + 2 = 0 => sin(x) = -2 (impossible)
    let (d, e) = (1.0, 2.0);
    let solver = MathTrigonometricFunctionRoots::new_sine(d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert_eq!(solver.nb_solutions(), 0);
}

// ====================== InfiniteSolutions ======================

#[test]
fn infinite_solutions() {
    // 0*sin(x) + 0 = 0 (always true)
    let (d, e) = (0.0, 0.0);
    let solver = MathTrigonometricFunctionRoots::new_sine(d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(solver.infinite_roots());
}

// ====================== CustomBounds ======================

#[test]
fn custom_bounds() {
    // sin(x) = 0 in range [PI/2, 3*PI/2]
    let (d, e) = (1.0, 0.0);
    let solver = MathTrigonometricFunctionRoots::new_sine(d, e, PI / 2.0, 3.0 * PI / 2.0);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() >= 1);

    for i in 1..=solver.nb_solutions() {
        let x = solver.value(i);
        assert!(x >= PI / 2.0);
        assert!(x <= 3.0 * PI / 2.0);
        near(x.sin().abs(), 0.0, TOLERANCE);
    }
}

// ====================== NarrowBounds ======================

#[test]
fn narrow_bounds() {
    // cos(x) = 0 in range [0, PI/4] => no solutions
    let (c, d, e) = (1.0, 0.0, 0.0);
    let solver = MathTrigonometricFunctionRoots::new_cosine_sine(c, d, e, 0.0, PI / 4.0);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert_eq!(solver.nb_solutions(), 0);
}

// ====================== QuadraticTerms ======================

#[test]
fn quadratic_terms() {
    // cos^2(x) - 0.5 = 0 => cos^2(x) = 0.5
    let (a, b, c, d, e) = (1.0, 0.0, 0.0, 0.0, -0.5);
    let solver = MathTrigonometricFunctionRoots::new(a, b, c, d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() >= 2);

    if solver.nb_solutions() >= 1 {
        let x1 = solver.value(1);
        near(x1.cos() * x1.cos(), 0.5, TOLERANCE);
    }
}

// ====================== MixedTerms ======================

#[test]
fn mixed_terms() {
    // cos(x) + sin(x) - sqrt(2) = 0 => x = PI/4
    let (c, d, e) = (1.0, 1.0, -(2.0f64).sqrt());
    let solver = MathTrigonometricFunctionRoots::new_cosine_sine(c, d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() >= 1);

    if solver.nb_solutions() >= 1 {
        let x1 = solver.value(1);
        near(x1.cos() + x1.sin(), (2.0f64).sqrt(), TOLERANCE);
    }
}

// ====================== AllCoefficients ======================

#[test]
fn all_coefficients() {
    // a*cos^2 + 2b*cos*sin + c*cos + d*sin + e = 0 with all coeffs non-zero
    let (a, b, c, d, e) = (1.0, 0.5, 0.5, 0.5, -0.25);
    let solver = MathTrigonometricFunctionRoots::new(a, b, c, d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());

    for i in 1..=solver.nb_solutions() {
        let x = solver.value(i);
        let result =
            a * x.cos() * x.cos() + 2.0 * b * x.cos() * x.sin() + c * x.cos() + d * x.sin() + e;
        near(result, 0.0, TOLERANCE);
    }
}

// ====================== LargeBounds ======================

#[test]
fn large_bounds() {
    // sin(x) = 0 over multiple periods [0, 4*PI]
    let (d, e) = (1.0, 0.0);
    let solver = MathTrigonometricFunctionRoots::new_sine(d, e, 0.0, 4.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() >= 2);
}

// ====================== NegativeBounds ======================

#[test]
fn negative_bounds() {
    // cos(x) = 0 with negative bounds [-PI, PI]
    let (c, d, e) = (1.0, 0.0, 0.0);
    let solver = MathTrigonometricFunctionRoots::new_cosine_sine(c, d, e, -PI, PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() >= 1);

    for i in 1..=solver.nb_solutions() {
        let x = solver.value(i);
        assert!(x >= -PI);
        assert!(x <= PI);
        near(x.cos().abs(), 0.0, TOLERANCE);
    }
}

// ====================== HighFrequencyTest ======================

#[test]
fn high_frequency_test() {
    // sin(x) - 0.5 = 0 with precise expected solutions PI/6 and 5*PI/6
    let (d, e) = (1.0, -0.5);
    let solver = MathTrigonometricFunctionRoots::new_sine(d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    assert!(!solver.infinite_roots());
    assert!(solver.nb_solutions() >= 2);

    if solver.nb_solutions() >= 2 {
        let mut solutions = Vec::new();
        for i in 1..=solver.nb_solutions() {
            solutions.push(solver.value(i));
        }

        let mut found_first = false;
        let mut found_second = false;
        for &sol in &solutions {
            if (sol - PI / 6.0).abs() < 0.1 {
                found_first = true;
            }
            if (sol - 5.0 * PI / 6.0).abs() < 0.1 {
                found_second = true;
            }
        }
        assert!(found_first || found_second);
    }
}

// ====================== EdgeCaseSmallCoefficients ======================

#[test]
fn edge_case_small_coefficients() {
    // Very small cosine coefficient: should behave ~ like sin(x) = 0
    let (c, d, e) = (1.0e-10, 1.0, 0.0);
    let solver = MathTrigonometricFunctionRoots::new_cosine_sine(c, d, e, 0.0, 2.0 * PI);

    assert!(solver.is_done());
    if !solver.infinite_roots() {
        assert!(solver.nb_solutions() >= 2);
    }
}
