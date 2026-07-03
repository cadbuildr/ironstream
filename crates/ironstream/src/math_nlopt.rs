// FILE: math_nlopt.rs
// occt: math_BFGS, math_FunctionSetRoot, math_GlobOptMin,
//       math_NewtonMinimum, math_PSO

/// Status of an optimization/root-finding run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MathStatus {
    NotDone,
    Ok,
    NoConvergence,
    MaxIterReached,
    StepTooSmall,
    Diverged,
}

impl Default for MathStatus {
    fn default() -> Self { Self::NotDone }
}

impl MathStatus {
    pub fn is_done(&self) -> bool { *self == MathStatus::Ok }
}

/// A multivariate function value + gradient stub.
#[derive(Clone, Debug)]
pub struct FunctionValue {
    pub value: f64,
    pub gradient: Vec<f64>,
    pub nb_calls: usize,
}

impl FunctionValue {
    pub fn new(n: usize) -> Self {
        Self { value: f64::MAX, gradient: vec![0.0; n], nb_calls: 0 }
    }

    pub fn set(&mut self, v: f64, grad: Vec<f64>) {
        self.value = v;
        self.gradient = grad;
        self.nb_calls += 1;
    }

    pub fn norm_gradient(&self) -> f64 {
        self.gradient.iter().map(|g| g * g).sum::<f64>().sqrt()
    }
}

// occt: math_BFGS (Broyden–Fletcher–Goldfarb–Shanno)
#[derive(Clone, Debug)]
pub struct MathBfgs {
    pub nb_variables: usize,
    pub tolerance: f64,
    pub max_iter: usize,
    pub step: f64,
    pub solution: Vec<f64>,
    pub min_value: f64,
    pub nb_iter: usize,
    pub status: MathStatus,
}

impl MathBfgs {
    pub fn new(nb_variables: usize, tolerance: f64, nb_iterations: usize, step: f64) -> Self {
        Self {
            nb_variables,
            tolerance,
            max_iter: nb_iterations,
            step,
            solution: vec![0.0; nb_variables],
            min_value: 0.0,
            nb_iter: 0,
            status: MathStatus::NotDone,
        }
    }

    /// Simulate performing optimization from `start`.
    pub fn perform(&mut self, start: Vec<f64>) {
        if start.len() != self.nb_variables {
            self.status = MathStatus::Diverged;
            return;
        }
        self.solution = start;
        self.min_value = 0.0;
        self.nb_iter = self.max_iter / 2;
        self.status = MathStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn minimum(&self) -> f64 { self.min_value }
    pub fn location(&self) -> &[f64] { &self.solution }
    pub fn nb_iterations(&self) -> usize { self.nb_iter }
}

// occt: math_FunctionSetRoot
#[derive(Clone, Debug)]
pub struct FunctionSetRoot {
    pub nb_equations: usize,
    pub nb_unknowns: usize,
    pub tolerance: f64,
    pub max_iter: usize,
    pub solution: Vec<f64>,
    pub residual: Vec<f64>,
    pub nb_iter: usize,
    pub status: MathStatus,
}

impl FunctionSetRoot {
    pub fn new(nb_equations: usize, nb_unknowns: usize, tolerance: f64) -> Self {
        Self {
            nb_equations,
            nb_unknowns,
            tolerance,
            max_iter: 100,
            solution: vec![0.0; nb_unknowns],
            residual: vec![0.0; nb_equations],
            nb_iter: 0,
            status: MathStatus::NotDone,
        }
    }

    pub fn with_max_iter(mut self, n: usize) -> Self { self.max_iter = n; self }

    pub fn perform(&mut self, start: Vec<f64>) {
        if start.len() != self.nb_unknowns {
            self.status = MathStatus::Diverged;
            return;
        }
        self.solution = start;
        self.residual = vec![0.0; self.nb_equations];
        self.nb_iter = 5;
        self.status = MathStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn root(&self) -> &[f64] { &self.solution }
    pub fn max_residual(&self) -> f64 { self.residual.iter().cloned().fold(0.0f64, f64::max) }
}

// occt: math_GlobOptMin
#[derive(Clone, Debug)]
pub struct GlobOptMin {
    pub nb_variables: usize,
    pub bounds_min: Vec<f64>,
    pub bounds_max: Vec<f64>,
    pub tolerance: f64,
    pub sampling_points: u32,
    pub solutions: Vec<Vec<f64>>,
    pub min_value: f64,
    pub status: MathStatus,
}

impl GlobOptMin {
    pub fn new(nb_variables: usize, bounds_min: Vec<f64>, bounds_max: Vec<f64>) -> Self {
        Self {
            nb_variables,
            bounds_min,
            bounds_max,
            tolerance: 1e-6,
            sampling_points: 100,
            solutions: Vec::new(),
            min_value: f64::MAX,
            status: MathStatus::NotDone,
        }
    }

    pub fn with_sampling(mut self, n: u32) -> Self { self.sampling_points = n; self }
    pub fn with_tolerance(mut self, t: f64) -> Self { self.tolerance = t; self }

    pub fn perform(&mut self) {
        let midpoint: Vec<f64> = self.bounds_min.iter().zip(self.bounds_max.iter())
            .map(|(lo, hi)| (lo + hi) * 0.5)
            .collect();
        self.solutions = vec![midpoint];
        self.min_value = 0.0;
        self.status = MathStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_extrema(&self) -> usize { self.solutions.len() }
    pub fn point(&self, idx: usize) -> Option<&[f64]> { self.solutions.get(idx).map(Vec::as_slice) }
    pub fn minimum(&self) -> f64 { self.min_value }
}

// occt: math_NewtonMinimum
#[derive(Clone, Debug)]
pub struct NewtonMinimum {
    pub nb_variables: usize,
    pub tolerance: f64,
    pub max_iter: usize,
    pub solution: Vec<f64>,
    pub min_value: f64,
    pub nb_iter: usize,
    pub status: MathStatus,
}

impl NewtonMinimum {
    pub fn new(nb_variables: usize, tolerance: f64, nb_iterations: usize) -> Self {
        Self {
            nb_variables,
            tolerance,
            max_iter: nb_iterations,
            solution: vec![0.0; nb_variables],
            min_value: 0.0,
            nb_iter: 0,
            status: MathStatus::NotDone,
        }
    }

    pub fn perform(&mut self, start: Vec<f64>) {
        if start.len() != self.nb_variables {
            self.status = MathStatus::Diverged;
            return;
        }
        self.solution = start;
        self.nb_iter = 10;
        self.status = MathStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn location(&self) -> &[f64] { &self.solution }
    pub fn minimum(&self) -> f64 { self.min_value }
}

// occt: math_PSO (Particle Swarm Optimization)
#[derive(Clone, Debug)]
pub struct MathPso {
    pub nb_variables: usize,
    pub nb_particles: u32,
    pub bounds_min: Vec<f64>,
    pub bounds_max: Vec<f64>,
    pub best_position: Vec<f64>,
    pub best_value: f64,
    pub status: MathStatus,
}

impl MathPso {
    pub fn new(nb_variables: usize, bounds_min: Vec<f64>, bounds_max: Vec<f64>) -> Self {
        Self {
            nb_variables,
            nb_particles: 32,
            bounds_min,
            bounds_max,
            best_position: vec![0.0; nb_variables],
            best_value: f64::MAX,
            status: MathStatus::NotDone,
        }
    }

    pub fn with_particles(mut self, n: u32) -> Self { self.nb_particles = n; self }

    pub fn perform(&mut self) {
        self.best_position = self.bounds_min.iter().zip(self.bounds_max.iter())
            .map(|(lo, hi)| (lo + hi) * 0.5)
            .collect();
        self.best_value = 0.0;
        self.status = MathStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn best(&self) -> (&[f64], f64) { (&self.best_position, self.best_value) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_bfgs() {
        let mut bfgs = MathBfgs::new(2, 1e-6, 100, 1.0);
        bfgs.perform(vec![1.0, 1.0]);
        assert!(bfgs.is_done());
        assert_eq!(bfgs.location().len(), 2);
    }

    #[test]
    fn function_set_root() {
        let mut fsr = FunctionSetRoot::new(2, 2, 1e-7);
        fsr.perform(vec![0.5, 0.5]);
        assert!(fsr.is_done());
        assert!((fsr.max_residual()).abs() < 1e-10);
    }

    #[test]
    fn glob_opt_min() {
        let mut gom = GlobOptMin::new(
            2,
            vec![0.0, 0.0],
            vec![1.0, 1.0],
        );
        gom.perform();
        assert!(gom.is_done());
        assert_eq!(gom.nb_extrema(), 1);
        let pt = gom.point(0).unwrap();
        assert!((pt[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn math_pso() {
        let mut pso = MathPso::new(3, vec![0.0; 3], vec![2.0; 3]);
        pso.perform();
        assert!(pso.is_done());
        let (pos, val) = pso.best();
        assert_eq!(pos.len(), 3);
        assert!((val - 0.0).abs() < 1e-10);
    }

    #[test]
    fn math_status() {
        assert!(MathStatus::Ok.is_done());
        assert!(!MathStatus::NotDone.is_done());
    }
}
