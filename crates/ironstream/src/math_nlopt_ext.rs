// FILE: math_nlopt_ext.rs
// occt: math_NewtonFunctionRoot, math_MultipleVarFunction,
//       math_FunctionRoots, math_GaussSingleIntegration

/// Status of a nonlinear solver run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NloptSolveStatus {
    #[default]
    NotDone,
    Converged,
    MaxIterReached,
    StepTooSmall,
    DivergingCorrectionVector,
}

impl NloptSolveStatus {
    pub fn is_done(&self) -> bool { *self == Self::Converged }
}

/// Univariate function interface for Newton root-finding.
pub trait MathFunction1d {
    fn value(&self, x: f64) -> f64;
    fn derivative(&self, x: f64) -> f64;
}

/// Newton-Raphson root finder for a C1 scalar function on [x_min, x_max].
/// occt: math_NewtonFunctionRoot
#[derive(Clone, Debug)]
pub struct MathNewtonFunctionRoot {
    pub x_min: f64,
    pub x_max: f64,
    pub tolerance: f64,
    pub max_iter: usize,
    pub root: f64,
    pub function_value: f64,
    pub status: NloptSolveStatus,
    pub nb_iterations: usize,
}

impl MathNewtonFunctionRoot {
    pub fn new(x_min: f64, x_max: f64, tolerance: f64) -> Self {
        Self {
            x_min,
            x_max,
            tolerance: tolerance.max(1e-14),
            max_iter: 100,
            root: x_min,
            function_value: f64::NAN,
            status: NloptSolveStatus::NotDone,
            nb_iterations: 0,
        }
    }

    pub fn set_max_iter(&mut self, n: usize) { self.max_iter = n; }

    /// Perform stub Newton iteration on f(x) using derivative.
    pub fn perform<F: MathFunction1d>(&mut self, f: &F, x0: f64) {
        let mut x = x0.clamp(self.x_min, self.x_max);
        self.nb_iterations = 0;
        for _ in 0..self.max_iter {
            let fx = f.value(x);
            let dfx = f.derivative(x);
            if dfx.abs() < 1e-14 { break; }
            let dx = -fx / dfx;
            x += dx;
            x = x.clamp(self.x_min, self.x_max);
            self.nb_iterations += 1;
            if dx.abs() < self.tolerance {
                self.root = x;
                self.function_value = f.value(x);
                self.status = NloptSolveStatus::Converged;
                return;
            }
        }
        self.root = x;
        self.function_value = f.value(x);
        self.status = NloptSolveStatus::MaxIterReached;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn root(&self) -> f64 { self.root }
    pub fn value(&self) -> f64 { self.function_value }
}

/// Finds all roots of a function on [x_min, x_max] by bracketed Newton.
/// occt: math_FunctionRoots
#[derive(Clone, Debug)]
pub struct MathFunctionRoots {
    pub x_min: f64,
    pub x_max: f64,
    pub nb_samples: usize,
    pub tolerance: f64,
    pub roots: Vec<f64>,
    pub is_done: bool,
}

impl MathFunctionRoots {
    pub fn new(x_min: f64, x_max: f64, nb_samples: usize, tolerance: f64) -> Self {
        Self {
            x_min,
            x_max,
            nb_samples: nb_samples.max(2),
            tolerance: tolerance.max(1e-14),
            roots: Vec::new(),
            is_done: false,
        }
    }

    /// Stub: sample linearly and collect sign changes, report midpoint as root.
    pub fn perform<F: Fn(f64) -> f64>(&mut self, f: F) {
        self.roots.clear();
        let n = self.nb_samples;
        let mut prev_x = self.x_min;
        let mut prev_f = f(prev_x);
        for i in 1..=n {
            let x = self.x_min + (self.x_max - self.x_min) * (i as f64 / n as f64);
            let fx = f(x);
            if prev_f * fx <= 0.0 && (prev_f != 0.0 || fx != 0.0) {
                let root = (prev_x + x) * 0.5;
                if !self.roots.iter().any(|r| (r - root).abs() < self.tolerance * 10.0) {
                    self.roots.push(root);
                }
            }
            prev_x = x;
            prev_f = fx;
        }
        self.is_done = true;
    }

    pub fn nb_solutions(&self) -> usize { self.roots.len() }
    pub fn is_done(&self) -> bool { self.is_done }
    pub fn value(&self, i: usize) -> Option<f64> { self.roots.get(i).copied() }
}

/// Gauss quadrature single integration.
/// occt: math_GaussSingleIntegration
#[derive(Clone, Debug)]
pub struct MathGaussSingleIntegration {
    pub x_min: f64,
    pub x_max: f64,
    pub nb_points: usize,
    pub result: f64,
    pub is_done: bool,
}

impl MathGaussSingleIntegration {
    pub fn new(x_min: f64, x_max: f64, nb_points: usize) -> Self {
        Self { x_min, x_max, nb_points: nb_points.max(1), result: 0.0, is_done: false }
    }

    /// Stub: Gauss-Legendre 3-point rule for nb_points.
    pub fn perform<F: Fn(f64) -> f64>(&mut self, f: F) {
        let a = self.x_min; let b = self.x_max;
        let mid = (a + b) / 2.0; let half = (b - a) / 2.0;
        // 3-point Gauss-Legendre nodes and weights on [-1,1]
        let nodes = [-0.7745966692, 0.0, 0.7745966692];
        let weights = [0.5555555556, 0.8888888889, 0.5555555556];
        let mut sum = 0.0;
        for k in 0..3 {
            let x = mid + half * nodes[k];
            sum += weights[k] * f(x);
        }
        self.result = half * sum;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn value(&self) -> f64 { self.result }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct LinearFunc { a: f64, b: f64 } // f(x) = a*x + b, df = a
    impl MathFunction1d for LinearFunc {
        fn value(&self, x: f64) -> f64 { self.a * x + self.b }
        fn derivative(&self, x: f64) -> f64 { let _ = x; self.a }
    }

    #[test]
    fn newton_linear() {
        let f = LinearFunc { a: 2.0, b: -4.0 }; // root at x=2
        let mut n = MathNewtonFunctionRoot::new(0.0, 10.0, 1e-12);
        n.perform(&f, 5.0);
        assert!(n.is_done());
        assert!((n.root() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn function_roots_quadratic() {
        // f(x) = x^2 - 1, roots at +1 and -1
        let mut fr = MathFunctionRoots::new(-2.0, 2.0, 100, 1e-6);
        fr.perform(|x| x * x - 1.0);
        assert!(fr.is_done());
        assert_eq!(fr.nb_solutions(), 2);
    }

    #[test]
    fn gauss_integration_linear() {
        // ∫[0,1] 2x dx = 1
        let mut g = MathGaussSingleIntegration::new(0.0, 1.0, 3);
        g.perform(|x| 2.0 * x);
        assert!(g.is_done());
        assert!((g.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn gauss_integration_constant() {
        // ∫[0,3] 4 dx = 12
        let mut g = MathGaussSingleIntegration::new(0.0, 3.0, 3);
        g.perform(|_| 4.0);
        assert!((g.value() - 12.0).abs() < 1e-10);
    }

    #[test]
    fn function_roots_no_root() {
        // f(x) = x^2 + 1, no real root
        let mut fr = MathFunctionRoots::new(-5.0, 5.0, 50, 1e-6);
        fr.perform(|x| x * x + 1.0);
        assert_eq!(fr.nb_solutions(), 0);
    }
}
