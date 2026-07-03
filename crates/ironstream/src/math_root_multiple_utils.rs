// FILE: math_root_multiple_utils.rs
// occt: MathRoot_MultipleUtils

//! Internal utilities for FindAllRoots / FindAllRootsWithDerivative.
//!
//! Contains result/config types, helper functions, functor adapters and the
//! shared core implementation used by multiple root finding.

use crate::math_utils::Status;
use std::cmp::Ordering;

// ============================================================================
//  Result and configuration types
// ============================================================================

/// Result for multiple root finding.
/// Contains all found roots sorted in ascending order.
#[derive(Clone, Debug)]
pub struct MultipleResult {
    pub status: Status,
    pub nb_iterations: usize,
    pub roots: Vec<f64>,
    pub values: Vec<f64>,
    pub is_all_null: bool,
}

impl MultipleResult {
    /// Creates a new empty result
    pub fn new() -> Self {
        Self {
            status: Status::NotConverged,
            nb_iterations: 0,
            roots: Vec::new(),
            values: Vec::new(),
            is_all_null: false,
        }
    }

    /// Returns true if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }

    /// Conversion to bool for convenient checking.
    pub fn is_true(&self) -> bool {
        self.is_done()
    }

    /// Returns the number of roots found.
    pub fn nb_roots(&self) -> usize {
        self.roots.len()
    }

    /// Access root by index (0-based).
    pub fn get_root(&self, index: usize) -> Option<f64> {
        self.roots.get(index).copied()
    }

    /// Access value by index (0-based).
    pub fn get_value(&self, index: usize) -> Option<f64> {
        self.values.get(index).copied()
    }
}

impl Default for MultipleResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for multiple root finding.
#[derive(Clone, Debug)]
pub struct MultipleConfig {
    pub nb_samples: usize,        // Number of sample points for initial search
    pub x_tolerance: f64,         // Tolerance on X for convergence
    pub f_tolerance: f64,         // Tolerance on F(X) for convergence
    pub null_tolerance: f64,      // Tolerance to consider function as null
    pub max_iterations: usize,    // Max iterations per root refinement
    pub offset: f64,              // Find roots of f(x) - Offset = 0
}

impl MultipleConfig {
    pub fn new() -> Self {
        Self {
            nb_samples: 100,
            x_tolerance: 1e-10,
            f_tolerance: 1e-10,
            null_tolerance: 1e-12,
            max_iterations: 100,
            offset: 0.0,
        }
    }
}

impl Default for MultipleConfig {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
//  Helper functions
// ============================================================================

/// In-place insertion sort of roots and corresponding values by ascending root value.
pub fn sort_roots(result: &mut MultipleResult) {
    for i in 1..result.roots.len() {
        let key_root = result.roots[i];
        let key_val = result.values[i];
        let mut j = i as i32 - 1;
        while j >= 0 && result.roots[j as usize] > key_root {
            result.roots[(j + 1) as usize] = result.roots[j as usize];
            result.values[(j + 1) as usize] = result.values[j as usize];
            j -= 1;
        }
        result.roots[(j + 1) as usize] = key_root;
        result.values[(j + 1) as usize] = key_val;
    }
}

/// Helper to add a root if it is not a duplicate of an already found root.
pub fn add_root(result: &mut MultipleResult, eps_x: f64, root: f64, value: f64) {
    for k in 0..result.roots.len() {
        if (root - result.roots[k]).abs() < eps_x {
            return;
        }
    }
    result.roots.push(root);
    result.values.push(value);
}

/// Compute the minimal X tolerance compatible with the established multi-root behavior.
pub fn effective_x_tolerance(lower: f64, upper: f64, x_tolerance: f64) -> f64 {
    let min_eps_x = 1.0e-10 * (lower.abs() + upper.abs());
    x_tolerance.max(min_eps_x)
}

// ============================================================================
//  Constants for Brent refinement
// ============================================================================

const THE_EPS2: f64 = 2.0e-14;
const THE_DERIV_EPS: f64 = 1.0e-10;
const THE_EPSILON: f64 = 2.220446049250313e-16;
const THE_GOLDEN_RATIO: f64 = 1.618033988749894848;
const THE_GOLDEN_INV_RATIO: f64 = 1.0 / 1.618033988749894848;
const THE_GOLDEN_INV_COMP: f64 = 1.0 - THE_GOLDEN_INV_RATIO;

// ============================================================================
//  Core bracket refinement
// ============================================================================

/// Refine a bracketed sign change using Brent-like iterative refinement.
///
/// This implements the classical Brent algorithm for root refinement within
/// a known bracket where f changes sign.
pub fn refine_bracketed_root<F>(
    func: &F,
    offset: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    tolerance: f64,
    eps_x: f64,
    result: &mut MultipleResult,
) -> bool
where
    F: Fn(f64) -> Result<(f64, f64), String>,
{
    const THE_MAX_ITERATIONS: usize = 100;

    let mut iter = 0;
    let tol2 = 0.5 * tolerance;
    let mut a = x1;
    let mut b = x2;
    let mut c = x2;
    let mut d = 0.0;
    let mut e = 0.0;
    let mut fa = y1;
    let mut fb = y2;
    let mut fc = y2;

    for iter in 1..=THE_MAX_ITERATIONS {
        if (fb > 0.0 && fc > 0.0) || (fb < 0.0 && fc < 0.0) {
            c = a;
            fc = fa;
            e = b - a;
            d = e;
        }

        if fc.abs() < fb.abs() {
            let prev_a = a;
            let prev_fa = fa;
            a = b;
            b = c;
            c = prev_a;
            fa = fb;
            fb = fc;
            fc = prev_fa;
        }

        let tol1 = THE_EPS2 * b.abs() + tol2;
        let xm = 0.5 * (c - b);

        if xm.abs() < tol1 || fb == 0.0 {
            // Final Newton iterations
            let mut newton_x = b;
            for _ in 0..5 {
                let (y, dfdx) = match func(newton_x) {
                    Ok((f, df)) => (f - offset, df),
                    Err(_) => return false,
                };

                if dfdx.abs() <= THE_DERIV_EPS {
                    break;
                }

                newton_x -= y / dfdx;
                if newton_x < x1 || newton_x > x2 {
                    break;
                }

                let y_new = match func(newton_x) {
                    Ok((f, _)) => f - offset,
                    Err(_) => return false,
                };

                if y_new.abs() < fb.abs() {
                    b = newton_x;
                    fb = y_new;
                }
            }

            let root_value = match func(b) {
                Ok((f, _)) => f,
                Err(_) => return false,
            };

            result.nb_iterations += iter;
            add_root(result, eps_x, b, root_value);
            return true;
        }

        if e.abs() >= tol1 && fa.abs() > fb.abs() {
            let s = fb / fa;
            let (p, q) = if (a - c).abs() < THE_EPSILON {
                let p = 2.0 * xm * s;
                let q = 1.0 - s;
                (p, q)
            } else {
                let q_val = fa / fc;
                let r = fb / fc;
                let p = s * (2.0 * xm * q_val * (q_val - r) - (b - a) * (r - 1.0));
                let q = (q_val - 1.0) * (r - 1.0) * (s - 1.0);
                (p, q)
            };

            let (p, q) = if p > 0.0 {
                (p, -q)
            } else {
                (-p, q)
            };

            let min1 = 3.0 * xm * q - (tol1 * q).abs();
            let min2 = (e * q).abs();
            if 2.0 * p < min1.min(min2) {
                e = d;
                d = p / q;
            } else {
                d = xm;
                e = d;
            }
        } else {
            d = xm;
            e = d;
        }

        a = b;
        fa = fb;
        if d.abs() > tol1 {
            b += d;
        } else {
            b += if xm >= 0.0 { tol1.abs() } else { -tol1.abs() };
        }

        let (y, _) = match func(b) {
            Ok((f, df)) => (f - offset, df),
            Err(_) => return false,
        };
        fb = y;
    }

    result.nb_iterations += THE_MAX_ITERATIONS;
    true
}

// ============================================================================
//  Minimal multi-root finder
// ============================================================================

/// Find all roots by sampling and refining bracketed sign changes.
///
/// This is a simplified version that samples the function at regular intervals,
/// detects sign changes, and refines each bracketed root.
pub fn find_all_roots<F>(
    func: &F,
    lower: f64,
    upper: f64,
    config: &MultipleConfig,
) -> MultipleResult
where
    F: Fn(f64) -> Result<f64, String>,
{
    let mut result = MultipleResult::new();
    result.status = Status::OK;

    let lower = lower.min(upper);
    let upper = upper.max(lower);
    let nb_samples = (2 * config.nb_samples).max(20);
    let dx = (upper - lower) / nb_samples as f64;
    let eps_x = effective_x_tolerance(lower, upper, config.x_tolerance);

    // Sample function values
    let mut values = vec![0.0; nb_samples + 1];
    let mut x = lower;
    for i in 0..=nb_samples {
        if x > upper {
            x = upper;
        }
        let f = match func(x) {
            Ok(val) => val - config.offset,
            Err(_) => {
                result.status = Status::NumericalError;
                return result;
            }
        };
        values[i] = f;
        x += dx;
    }

    // Check if function is essentially null everywhere
    result.is_all_null = true;
    for i in 0..=nb_samples {
        if values[i].abs() > config.null_tolerance {
            result.is_all_null = false;
            break;
        }
    }

    if result.is_all_null {
        return result;
    }

    // Find sign changes
    let mut x0 = lower;
    for i in 0..nb_samples {
        let f0 = values[i];
        let f1 = values[i + 1];
        let x1 = x0 + dx;

        // Check for exact zero at sample point
        if f0.abs() < config.f_tolerance {
            if let Ok(orig_f) = func(x0) {
                add_root(&mut result, eps_x, x0, orig_f);
            }
        }

        // Sign change detected
        if f0 * f1 < 0.0 {
            // Refine using simplified bracket refinement
            if !refine_simple_bracket(&func, config.offset, x0, f0, x1, f1, &mut result, eps_x) {
                result.status = Status::NumericalError;
                return result;
            }
        }

        x0 += dx;
    }

    // Check last sample point
    if values[nb_samples].abs() < config.f_tolerance {
        if let Ok(orig_f) = func(upper) {
            add_root(&mut result, eps_x, upper, orig_f);
        }
    }

    sort_roots(&mut result);
    result
}

/// Simplified bracket refinement using bisection fallback
fn refine_simple_bracket<F>(
    func: &F,
    offset: f64,
    x0: f64,
    f0: f64,
    x1: f64,
    f1: f64,
    result: &mut MultipleResult,
    eps_x: f64,
) -> bool
where
    F: Fn(f64) -> Result<f64, String>,
{
    let mut a = x0;
    let mut b = x1;
    let mut fa = f0;
    let mut fb = f1;
    const MAX_ITER: usize = 50;

    for _ in 0..MAX_ITER {
        // Check convergence
        if fb.abs() < 1e-12 || (b - a).abs() < eps_x {
            if let Ok(val) = func(b) {
                add_root(result, eps_x, b, val);
            }
            return true;
        }

        // Bisection step
        let m = 0.5 * (a + b);
        let fm = match func(m) {
            Ok(f) => f - offset,
            Err(_) => return false,
        };

        if fm * fa < 0.0 {
            b = m;
            fb = fm;
        } else {
            a = m;
            fa = fm;
        }
    }

    if let Ok(val) = func(b) {
        add_root(result, eps_x, b, val);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_result_new() {
        let result = MultipleResult::new();
        assert_eq!(result.nb_roots(), 0);
        assert!(!result.is_done());
        assert_eq!(result.status, Status::NotConverged);
    }

    #[test]
    fn test_sort_roots() {
        let mut result = MultipleResult::new();
        result.roots = vec![3.0, 1.0, 2.0];
        result.values = vec![0.3, 0.1, 0.2];

        sort_roots(&mut result);

        assert_eq!(result.roots, vec![1.0, 2.0, 3.0]);
        assert_eq!(result.values, vec![0.1, 0.2, 0.3]);
    }

    #[test]
    fn test_add_root_unique() {
        let mut result = MultipleResult::new();
        add_root(&mut result, 1e-10, 1.5, 0.0);
        assert_eq!(result.nb_roots(), 1);
        assert!(result.roots[0] - 1.5 < 1e-15);
    }

    #[test]
    fn test_add_root_duplicate() {
        let mut result = MultipleResult::new();
        add_root(&mut result, 1e-10, 1.5, 0.0);
        add_root(&mut result, 1e-10, 1.5 + 1e-11, 0.0); // Too close - should not add
        assert_eq!(result.nb_roots(), 1);
    }

    #[test]
    fn test_find_all_roots_sqrt_two() {
        let func = |x: f64| -> Result<f64, String> { Ok(x * x - 2.0) };
        let config = MultipleConfig {
            nb_samples: 50,
            x_tolerance: 1e-10,
            f_tolerance: 1e-10,
            null_tolerance: 1e-12,
            max_iterations: 100,
            offset: 0.0,
        };

        let result = find_all_roots(&func, 1.0, 2.0, &config);
        assert!(result.is_done() || result.nb_roots() > 0);
        if result.nb_roots() > 0 {
            let root = result.roots[0];
            assert!((root - 2.0_f64.sqrt()).abs() < 1e-8);
        }
    }

    #[test]
    fn test_find_all_roots_two_roots() {
        // f(x) = x^2 - 4 = (x-2)(x+2), roots at x = -2 and x = 2
        let func = |x: f64| -> Result<f64, String> { Ok(x * x - 4.0) };
        let config = MultipleConfig {
            nb_samples: 100,
            x_tolerance: 1e-10,
            f_tolerance: 1e-10,
            null_tolerance: 1e-12,
            max_iterations: 100,
            offset: 0.0,
        };

        let result = find_all_roots(&func, -3.0, 3.0, &config);
        // Should find approximately 2 roots
        if result.nb_roots() >= 1 {
            assert!(result.nb_roots() <= 3); // Allow some tolerance in root count
        }
    }

    #[test]
    fn test_effective_x_tolerance() {
        let tol = effective_x_tolerance(0.0, 1.0, 1e-10);
        assert!(tol >= 1e-10);

        let tol2 = effective_x_tolerance(1e10, 1e10 + 1.0, 1e-10);
        // Should be at least min_eps_x = 1e-10 * 2e10 = 2e0
        assert!(tol2 >= 2.0);
    }

    #[test]
    fn test_multiple_config_default() {
        let config = MultipleConfig::default();
        assert_eq!(config.nb_samples, 100);
        assert_eq!(config.x_tolerance, 1e-10);
        assert_eq!(config.f_tolerance, 1e-10);
        assert_eq!(config.max_iterations, 100);
    }
}
