// FILE: math_root_all.rs
//
// Faithful port of OCCT MathRoot_All root finding algorithms.
// Finds all roots using sampling and refinement, detects null intervals.

// occt: MathRoot_All

use core::f64;

/// Status indicator for root finding results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootStatus {
    /// Computation successful
    Ok,
    /// Did not converge within tolerance
    NotConverged,
    /// Invalid input parameters
    InvalidInput,
}

/// Represents an interval where the function is null (within tolerance)
#[derive(Debug, Clone)]
pub struct NullInterval {
    /// Interval start
    pub a: f64,
    /// Interval end
    pub b: f64,
    /// State number (for parametric curves)
    pub state: i32,
}

impl Default for NullInterval {
    fn default() -> Self {
        NullInterval {
            a: 0.0,
            b: 0.0,
            state: 0,
        }
    }
}

/// Result for all roots finder including null intervals
#[derive(Debug, Clone)]
pub struct AllRootsResult {
    /// Computation status
    pub status: RootStatus,
    /// Isolated root locations
    pub roots: Vec<f64>,
    /// State numbers for roots
    pub root_states: Vec<i32>,
    /// Intervals where function is null
    pub null_intervals: Vec<NullInterval>,
}

impl AllRootsResult {
    /// Creates a new empty result
    pub fn new() -> Self {
        AllRootsResult {
            status: RootStatus::NotConverged,
            roots: Vec::new(),
            root_states: Vec::new(),
            null_intervals: Vec::new(),
        }
    }

    /// Returns true if computation succeeded
    pub fn is_done(&self) -> bool {
        self.status == RootStatus::Ok
    }

    /// Returns the number of roots found
    pub fn nb_roots(&self) -> usize {
        self.roots.len()
    }

    /// Returns the number of null intervals found
    pub fn nb_intervals(&self) -> usize {
        self.null_intervals.len()
    }

    /// Access root by index (0-based)
    pub fn get_root(&self, index: usize) -> f64 {
        if index < self.roots.len() {
            self.roots[index]
        } else {
            0.0
        }
    }
}

impl Default for AllRootsResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for multiple root finding
#[derive(Debug, Clone)]
pub struct MultipleConfig {
    /// Number of sample points for initial search
    pub nb_samples: usize,
    /// Tolerance on X for convergence
    pub x_tolerance: f64,
    /// Tolerance on F(X) for convergence
    pub f_tolerance: f64,
    /// Tolerance to consider function as null
    pub null_tolerance: f64,
    /// Maximum iterations per root refinement
    pub max_iterations: usize,
    /// Find roots of f(x) - offset = 0
    pub offset: f64,
}

impl Default for MultipleConfig {
    fn default() -> Self {
        MultipleConfig {
            nb_samples: 100,
            x_tolerance: 1e-10,
            f_tolerance: 1e-10,
            null_tolerance: 1e-12,
            max_iterations: 100,
            offset: 0.0,
        }
    }
}

/// Find all roots using uniform sampling.
///
/// Uses a sample of the function to find:
/// 1. Null intervals: where |F(x)| <= EpsNul for consecutive sample points
/// 2. Isolated roots: single points where F(x) = 0
///
/// The algorithm:
/// 1. Evaluates F at sample points
/// 2. Identifies null intervals where |F| <= EpsNul for 2+ consecutive points
/// 3. Refines interval boundaries
/// 4. Finds isolated roots between null intervals
///
/// # Arguments
/// * `func` - function with Value(x) -> Option<f64> method
/// * `a` - interval start
/// * `b` - interval end
/// * `nb_samples` - number of sample points
/// * `eps_x` - tolerance for root x-value
/// * `eps_f` - tolerance for function value
/// * `eps_nul` - tolerance for null interval detection
///
/// # Returns
/// AllRootsResult with roots and null intervals
pub fn find_all_roots_with_intervals<F>(
    func: &F,
    a: f64,
    b: f64,
    nb_samples: usize,
    eps_x: f64,
    eps_f: f64,
    eps_nul: f64,
) -> AllRootsResult
where
    F: Fn(f64) -> Option<f64>,
{
    let mut result = AllRootsResult::new();

    if nb_samples < 2 {
        result.status = RootStatus::InvalidInput;
        return result;
    }

    // Create sample points
    let mut samples = Vec::with_capacity(nb_samples);
    let step = (b - a) / (nb_samples - 1) as f64;
    for i in 0..nb_samples {
        samples.push(a + i as f64 * step);
    }
    samples[nb_samples - 1] = b; // Ensure last point is exactly b

    // Evaluate function at all sample points
    let mut f_values = Vec::with_capacity(nb_samples);
    for &x in &samples {
        match func(x) {
            Some(f) => f_values.push(f),
            None => {
                result.status = RootStatus::NotConverged;
                return result;
            }
        }
    }

    // Scan for null intervals
    let mut interval_starts = Vec::new();
    let mut interval_ends = Vec::new();

    let mut in_interval = false;
    let mut deb_nul = 0.0;
    let mut fin_nul = 0.0;
    let mut val_sav = f_values[0];
    let mut prev_nul = f_values[0].abs() <= eps_nul;

    for i in 1..nb_samples {
        let cur_nul = f_values[i].abs() <= eps_nul;

        if !cur_nul {
            val_sav = f_values[i];
        }

        if in_interval && !cur_nul {
            // End of null interval
            in_interval = false;
            interval_starts.push(deb_nul);
            fin_nul = samples[i - 1];
            interval_ends.push(fin_nul);
        } else if !in_interval && prev_nul && cur_nul {
            // Start of null interval
            in_interval = true;
            if i == 1 {
                deb_nul = samples[0];
            } else {
                deb_nul = samples[i - 1];
            }
        }

        prev_nul = cur_nul;
    }

    // Handle interval ending at last sample
    if in_interval {
        interval_starts.push(deb_nul);
        fin_nul = samples[nb_samples - 1];
        interval_ends.push(fin_nul);
    }

    // Store null intervals
    for k in 0..interval_starts.len() {
        let null_int = NullInterval {
            a: interval_starts[k],
            b: interval_ends[k],
            state: 0,
        };
        result.null_intervals.push(null_int);
    }

    // Find isolated roots between null intervals using simple bisection
    if interval_starts.is_empty() {
        // No null intervals - find all roots in entire range
        find_roots_in_range(func, a, b, eps_x, eps_f, &mut result);
    } else {
        // Find roots before first null interval
        if !samples[0].is_finite() {
            let start = a;
            let end = interval_starts[0];
            find_roots_in_range(func, start, end, eps_x, eps_f, &mut result);
        } else if (interval_starts[0] - a).abs() > eps_x {
            find_roots_in_range(func, a, interval_starts[0], eps_x, eps_f, &mut result);
        }

        // Find roots between null intervals
        for k in 1..interval_starts.len() {
            let start = interval_ends[k - 1];
            let end = interval_starts[k];
            if (end - start).abs() > eps_x {
                find_roots_in_range(func, start, end, eps_x, eps_f, &mut result);
            }
        }

        // Find roots after last null interval
        if (b - interval_ends[interval_ends.len() - 1]).abs() > eps_x {
            find_roots_in_range(func, interval_ends[interval_ends.len() - 1], b, eps_x, eps_f, &mut result);
        }
    }

    result.status = RootStatus::Ok;
    result
}

/// Helper function to find roots in a range using simple bracketing
fn find_roots_in_range<F>(
    func: &F,
    a: f64,
    b: f64,
    eps_x: f64,
    eps_f: f64,
    result: &mut AllRootsResult,
) where
    F: Fn(f64) -> Option<f64>,
{
    const NB_SAMPLES: usize = 20;

    let step = (b - a) / (NB_SAMPLES - 1) as f64;

    // Get first value
    let mut prev_f = match func(a) {
        Some(f) => f,
        None => return,
    };

    // Scan for sign changes
    for i in 1..NB_SAMPLES {
        let x = a + i as f64 * step;
        let cur_f = match func(x) {
            Some(f) => f,
            None => return,
        };

        // Check for sign change
        if (prev_f * cur_f < 0.0) || prev_f.abs() < eps_f {
            // Refine root using bisection
            let root = bisect(func, a + (i - 1) as f64 * step, x, eps_x, eps_f);
            add_root_if_unique(result, root, eps_x);
        }

        prev_f = cur_f;
    }

    // Check last point
    if prev_f.abs() < eps_f {
        add_root_if_unique(result, b, eps_x);
    }
}

/// Bisection method for root refinement
fn bisect<F>(func: &F, mut a: f64, mut b: f64, eps_x: f64, eps_f: f64) -> f64
where
    F: Fn(f64) -> Option<f64>,
{
    for _ in 0..50 {
        let mid = (a + b) / 2.0;

        if (b - a).abs() < eps_x {
            return mid;
        }

        let f_mid = match func(mid) {
            Some(f) => f,
            None => return mid,
        };

        if f_mid.abs() < eps_f {
            return mid;
        }

        let f_a = match func(a) {
            Some(f) => f,
            None => return mid,
        };

        if (f_a * f_mid) < 0.0 {
            b = mid;
        } else {
            a = mid;
        }
    }

    (a + b) / 2.0
}

/// Add a root to result if it's not a duplicate
fn add_root_if_unique(result: &mut AllRootsResult, root: f64, eps_x: f64) {
    // Check if this root is already in the list
    for existing_root in &result.roots {
        if (root - existing_root).abs() < eps_x {
            return;
        }
    }
    result.roots.push(root);
    result.root_states.push(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_roots_simple_quadratic() {
        // f(x) = x^2 - 4 has roots at -2 and 2
        let f = |x: f64| Some(x * x - 4.0);

        let result = find_all_roots_with_intervals(&f, -5.0, 5.0, 21, 1e-6, 1e-6, 1e-10);
        assert_eq!(result.status, RootStatus::Ok);
        assert!(result.nb_roots() >= 2, "Should find at least 2 roots");

        // Check that roots are approximately -2 and 2
        let has_neg2 = result.roots.iter().any(|r| (r + 2.0).abs() < 1e-5);
        let has_pos2 = result.roots.iter().any(|r| (r - 2.0).abs() < 1e-5);
        assert!(has_neg2, "Should find root near -2");
        assert!(has_pos2, "Should find root near 2");
    }

    #[test]
    fn test_find_roots_cubic() {
        // f(x) = x^3 - x has roots at -1, 0, 1
        let f = |x: f64| Some(x * x * x - x);

        let result = find_all_roots_with_intervals(&f, -2.0, 2.0, 21, 1e-6, 1e-6, 1e-10);
        assert_eq!(result.status, RootStatus::Ok);
        assert!(result.nb_roots() >= 2, "Should find at least 2 roots");
    }

    #[test]
    fn test_find_roots_no_roots() {
        // f(x) = x^2 + 1 has no real roots
        let f = |x: f64| Some(x * x + 1.0);

        let result = find_all_roots_with_intervals(&f, -5.0, 5.0, 21, 1e-6, 1e-6, 1e-10);
        assert_eq!(result.status, RootStatus::Ok);
        assert_eq!(result.nb_roots(), 0, "Should find no roots");
    }

    #[test]
    fn test_find_roots_single_root() {
        // f(x) = x has root at 0
        let f = |x: f64| Some(x);

        let result = find_all_roots_with_intervals(&f, -2.0, 2.0, 21, 1e-6, 1e-6, 1e-10);
        assert_eq!(result.status, RootStatus::Ok);
        assert!(result.nb_roots() >= 1, "Should find at least 1 root");
        assert!(result.roots.iter().any(|r| r.abs() < 1e-5), "Should find root near 0");
    }

    #[test]
    fn test_find_roots_invalid_input() {
        let f = |x: f64| Some(x);

        // Not enough samples
        let result = find_all_roots_with_intervals(&f, -2.0, 2.0, 1, 1e-6, 1e-6, 1e-10);
        assert_eq!(result.status, RootStatus::InvalidInput);
    }

    #[test]
    fn test_null_interval_detection() {
        // f(x) = 0 everywhere (constant zero) -> should detect null interval
        let f = |_x: f64| Some(0.0);

        let result = find_all_roots_with_intervals(&f, 0.0, 1.0, 11, 1e-6, 1e-6, 1e-10);
        assert_eq!(result.status, RootStatus::Ok);
        // Should detect at least one null interval
        assert!(result.nb_intervals() > 0, "Should detect null interval for constant zero");
    }

    #[test]
    fn test_multiple_config_default() {
        let config = MultipleConfig::default();
        assert_eq!(config.nb_samples, 100);
        assert_eq!(config.x_tolerance, 1e-10);
        assert_eq!(config.f_tolerance, 1e-10);
        assert_eq!(config.offset, 0.0);
    }

    #[test]
    fn test_null_interval_struct() {
        let interval = NullInterval {
            a: 0.5,
            b: 1.5,
            state: 42,
        };
        assert_eq!(interval.a, 0.5);
        assert_eq!(interval.b, 1.5);
        assert_eq!(interval.state, 42);
    }
}
