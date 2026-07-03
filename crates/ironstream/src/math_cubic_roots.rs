// FILE: rust/ironstream/crates/ironstream/src/math_cubic_roots.rs

// occt: math_DirectPolynomialRoots / math_TrigonometricEquationFunction

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Real cube root: handles negative arguments correctly.
fn real_cbrt(x: f64) -> f64 {
    x.cbrt()
}

// ---------------------------------------------------------------------------
// solve_quadratic — returns Vec<f64>
// ---------------------------------------------------------------------------

/// Solves `a*x² + b*x + c = 0` and returns all real roots.
///
/// Degenerates to a linear solve when `a ≈ 0`.  Uses a numerically stable
/// formula (avoids catastrophic cancellation).
///
// occt: math_DirectPolynomialRoots (degree-2 case)
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    const ZERO: f64 = 1.0e-30;
    let mut roots = Vec::new();

    if a.abs() <= ZERO {
        // Degenerate: linear.
        if b.abs() > ZERO {
            roots.push(-c / b);
        }
        return roots;
    }

    let p = b / a;
    let q = c / a;

    let eps_d = 3.0 * f64::EPSILON * (p * p + (4.0 * q).abs());
    let mut disc = p * p - 4.0 * q;
    if disc.abs() <= eps_d {
        disc = 0.0;
    }

    if disc < 0.0 {
        // No real roots.
    } else if disc == 0.0 {
        // Double root.
        let r = -0.5 * p;
        roots.push(r);
        roots.push(r);
    } else {
        // Two distinct real roots (numerically stable).
        let sqrt_d = disc.sqrt();
        let r0 = if p >= 0.0 {
            -(p + sqrt_d) * 0.5
        } else {
            -(p - sqrt_d) * 0.5
        };
        let r1 = if r0.abs() > ZERO { q / r0 } else { -(p - sqrt_d) * 0.5 };
        roots.push(r0);
        roots.push(r1);
    }

    roots
}

// ---------------------------------------------------------------------------
// MathCubicRoots
// ---------------------------------------------------------------------------

/// Cubic polynomial root solver.
///
/// Call `perform(a, b, c, d)` to solve `a*x³ + b*x² + c*x + d = 0`.
///
// occt: math_DirectPolynomialRoots (degree-3 case)
#[derive(Debug, Clone)]
pub struct MathCubicRoots {
    roots: Vec<f64>,
    done: bool,
}

impl MathCubicRoots {
    /// Create a new, unperformed solver.
    pub fn new() -> Self {
        Self { roots: Vec::new(), done: false }
    }

    /// Solve `a*x³ + b*x² + c*x + d = 0`.
    ///
    /// Degenerates to a quadratic solver when `a ≈ 0`.
    pub fn perform(&mut self, a: f64, b: f64, c: f64, d: f64) {
        self.roots = solve_cubic_internal(a, b, c, d);
        self.done = true;
    }

    /// Returns `true` once `perform` has been called.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the number of real roots found.
    pub fn nb_roots(&self) -> usize {
        self.roots.len()
    }

    /// Returns the `i`-th root (0-based).
    ///
    /// # Panics
    /// Panics if `i >= nb_roots()` or if `perform` has not been called.
    pub fn root(&self, i: usize) -> f64 {
        assert!(
            self.done,
            "MathCubicRoots: root() called before perform()"
        );
        assert!(
            i < self.roots.len(),
            "MathCubicRoots: index {i} out of range (nb_roots = {})",
            self.roots.len()
        );
        self.roots[i]
    }
}

impl Default for MathCubicRoots {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// MathQuarticRoots
// ---------------------------------------------------------------------------

/// Quartic polynomial root solver (stub).
///
/// When `a ≈ 0`, degenerates to `MathCubicRoots`.
/// For a true degree-4 polynomial, `is_done = true` and `nb_roots = 0` (stub).
///
// occt: math_DirectPolynomialRoots (degree-4 case)
#[derive(Debug, Clone)]
pub struct MathQuarticRoots {
    roots: Vec<f64>,
    done: bool,
}

impl MathQuarticRoots {
    /// Create a new, unperformed solver.
    pub fn new() -> Self {
        Self { roots: Vec::new(), done: false }
    }

    /// Solve `a*x⁴ + b*x³ + c*x² + d*x + e = 0`.
    ///
    /// Degenerates to cubic when `a ≈ 0`; otherwise stub returns 0 roots.
    pub fn perform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64) {
        const ZERO: f64 = 1.0e-30;
        if a.abs() <= ZERO {
            self.roots = solve_cubic_internal(b, c, d, e);
        } else {
            // Stub for true quartic: done but no roots.
            self.roots = Vec::new();
        }
        self.done = true;
    }

    /// Returns `true` once `perform` has been called.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the number of real roots found.
    pub fn nb_roots(&self) -> usize {
        self.roots.len()
    }

    /// Returns the `i`-th root (0-based).
    ///
    /// # Panics
    /// Panics if `i >= nb_roots()` or if `perform` has not been called.
    pub fn root(&self, i: usize) -> f64 {
        assert!(
            self.done,
            "MathQuarticRoots: root() called before perform()"
        );
        assert!(
            i < self.roots.len(),
            "MathQuarticRoots: index {i} out of range (nb_roots = {})",
            self.roots.len()
        );
        self.roots[i]
    }
}

impl Default for MathQuarticRoots {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal cubic solver (returns Vec<f64>)
// ---------------------------------------------------------------------------

fn solve_cubic_internal(a: f64, b: f64, c: f64, d: f64) -> Vec<f64> {
    const ZERO: f64 = 1.0e-30;
    let mut roots = Vec::new();

    if a.abs() <= ZERO {
        return solve_quadratic(b, c, d);
    }

    let inv_a = 1.0 / a;
    let b1 = b * inv_a;
    let c1 = c * inv_a;
    let d1 = d * inv_a;

    let shift = b1 / 3.0;
    let p = c1 - b1 * b1 / 3.0;
    let q = d1 + 2.0 * b1 * b1 * b1 / 27.0 - b1 * c1 / 3.0;

    let disc = (q * q) / 4.0 + (p * p * p) / 27.0;

    const TWO_PI_3: f64 = 2.0 * PI / 3.0;

    if disc > 0.0 {
        let sqrt_d = disc.sqrt();
        let u = real_cbrt(-q / 2.0 + sqrt_d);
        let v = real_cbrt(-q / 2.0 - sqrt_d);
        roots.push(u + v - shift);
    } else if disc < 0.0 {
        let m = 2.0 * (-p / 3.0).sqrt();
        let arg = ((3.0 * q) / (p * m)).clamp(-1.0, 1.0);
        let theta = arg.acos() / 3.0;
        roots.push(m * theta.cos() - shift);
        roots.push(m * (theta - TWO_PI_3).cos() - shift);
        roots.push(m * (theta - 2.0 * TWO_PI_3).cos() - shift);
    } else {
        // disc == 0
        if q.abs() < ZERO {
            let r = -shift;
            roots.push(r);
            roots.push(r);
            roots.push(r);
        } else {
            let r_double = 3.0 * q / p;
            let r_simple = -2.0 * r_double;
            roots.push(r_simple - shift);
            roots.push(r_double - shift);
            roots.push(r_double - shift);
        }
    }

    roots
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1.0e-9;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    #[test]
    fn test_quadratic_two_roots() {
        let roots = solve_quadratic(1.0, -5.0, 6.0);
        assert_eq!(roots.len(), 2);
        let mut r = roots.clone();
        r.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!(approx_eq(r[0], 2.0));
        assert!(approx_eq(r[1], 3.0));
    }

    #[test]
    fn test_cubic_three_roots() {
        let mut solver = MathCubicRoots::new();
        solver.perform(1.0, -6.0, 11.0, -6.0);
        assert!(solver.is_done());
        assert_eq!(solver.nb_roots(), 3);
        let mut roots: Vec<f64> = (0..3).map(|i| solver.root(i)).collect();
        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!(approx_eq(roots[0], 1.0));
        assert!(approx_eq(roots[1], 2.0));
        assert!(approx_eq(roots[2], 3.0));
    }

    #[test]
    fn test_quartic_not_done_before_perform() {
        let s = MathQuarticRoots::new();
        assert!(!s.is_done());
        assert_eq!(s.nb_roots(), 0);
    }

    #[test]
    fn test_quartic_degenerates_to_cubic() {
        let mut s = MathQuarticRoots::new();
        s.perform(0.0, 1.0, -6.0, 11.0, -6.0);
        assert!(s.is_done());
        assert_eq!(s.nb_roots(), 3);
    }
}
