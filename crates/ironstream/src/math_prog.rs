// FILE: math_prog.rs
// occt-ref: math_BFGS, math_GlobOptMin, math_FunctionSetRoot

/// Status of a math solver.
// occt-ref: math // solver status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MathSolverStatus {
    NotDone,
    Done,
    NoConvergence,
    InvalidInput,
}

impl Default for MathSolverStatus {
    fn default() -> Self { Self::NotDone }
}

/// BFGS unconstrained minimizer (stub).
// occt-ref: math_BFGS
#[derive(Clone, Debug)]
pub struct MathBfgs {
    pub nb_variables: usize,
    pub tolerance: f64,
    pub max_iter: usize,
    pub status: MathSolverStatus,
    pub solution: Vec<f64>,
    pub minimum: f64,
    pub nb_iterations: usize,
}

impl MathBfgs {
    pub fn new(nb_variables: usize) -> Self {
        Self {
            nb_variables,
            tolerance: 1e-8,
            max_iter: 100,
            status: MathSolverStatus::NotDone,
            solution: vec![0.0; nb_variables],
            minimum: f64::INFINITY,
            nb_iterations: 0,
        }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(1e-15); }
    pub fn set_max_iter(&mut self, n: usize) { self.max_iter = n.max(1); }

    /// Stub perform: pretend we minimized a quadratic f(x) = sum(x[i]^2).
    /// Real BFGS would evaluate the function + gradient.
    pub fn perform(&mut self, start: &[f64]) {
        if start.len() != self.nb_variables {
            self.status = MathSolverStatus::InvalidInput;
            return;
        }
        // Stub: move toward zero with damped steps
        self.solution = start.to_vec();
        let mut val = start.iter().map(|x| x*x).sum::<f64>();
        let mut iter = 0;
        while val > self.tolerance && iter < self.max_iter {
            for x in &mut self.solution { *x *= 0.5; }
            val = self.solution.iter().map(|x| x*x).sum::<f64>();
            iter += 1;
        }
        self.minimum = val;
        self.nb_iterations = iter;
        self.status = if val <= self.tolerance { MathSolverStatus::Done } else { MathSolverStatus::NoConvergence };
    }

    pub fn is_done(&self) -> bool { self.status == MathSolverStatus::Done }
    pub fn minimum(&self) -> f64 { self.minimum }
    pub fn solution(&self) -> &[f64] { &self.solution }
    pub fn nb_iterations(&self) -> usize { self.nb_iterations }
}

/// Global optimization via branch-and-bound (stub).
// occt-ref: math_GlobOptMin
#[derive(Clone, Debug)]
pub struct MathGlobOptMin {
    pub nb_variables: usize,
    pub lower_bounds: Vec<f64>,
    pub upper_bounds: Vec<f64>,
    pub tolerance: f64,
    pub max_iter: usize,
    pub status: MathSolverStatus,
    pub solution: Vec<f64>,
    pub minimum: f64,
    pub nb_solutions: usize,
}

impl MathGlobOptMin {
    pub fn new(nb_variables: usize) -> Self {
        Self {
            nb_variables,
            lower_bounds: vec![-1e10; nb_variables],
            upper_bounds: vec![1e10; nb_variables],
            tolerance: 1e-6,
            max_iter: 500,
            status: MathSolverStatus::NotDone,
            solution: vec![0.0; nb_variables],
            minimum: f64::INFINITY,
            nb_solutions: 0,
        }
    }

    pub fn set_bounds(&mut self, lower: &[f64], upper: &[f64]) {
        if lower.len() == self.nb_variables && upper.len() == self.nb_variables {
            self.lower_bounds = lower.to_vec();
            self.upper_bounds = upper.to_vec();
        }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(1e-15); }

    /// Stub: sample midpoints of bounds as solution.
    pub fn perform(&mut self) {
        for i in 0..self.nb_variables {
            let lo = self.lower_bounds[i];
            let hi = self.upper_bounds[i];
            if hi < lo { self.status = MathSolverStatus::InvalidInput; return; }
            self.solution[i] = (lo + hi) * 0.5;
        }
        self.minimum = self.solution.iter().map(|x| x*x).sum();
        self.nb_solutions = 1;
        self.status = MathSolverStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status == MathSolverStatus::Done }
    pub fn minimum(&self) -> f64 { self.minimum }
    pub fn solution(&self) -> &[f64] { &self.solution }
    pub fn nb_solutions(&self) -> usize { self.nb_solutions }
}

/// Non-linear system root finder (stub).
// occt-ref: math_FunctionSetRoot
#[derive(Clone, Debug)]
pub struct MathFunctionSetRoot {
    pub nb_equations: usize,
    pub tolerance: f64,
    pub max_iter: usize,
    pub status: MathSolverStatus,
    pub root: Vec<f64>,
    pub residual: Vec<f64>,
    pub nb_iterations: usize,
}

impl MathFunctionSetRoot {
    pub fn new(nb_equations: usize) -> Self {
        Self {
            nb_equations,
            tolerance: 1e-10,
            max_iter: 50,
            status: MathSolverStatus::NotDone,
            root: vec![0.0; nb_equations],
            residual: vec![0.0; nb_equations],
            nb_iterations: 0,
        }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(1e-15); }
    pub fn set_max_iter(&mut self, n: usize) { self.max_iter = n.max(1); }

    /// Stub: assume the system is f(x) = x, root at origin.
    pub fn perform(&mut self, start: &[f64]) {
        if start.len() != self.nb_equations {
            self.status = MathSolverStatus::InvalidInput;
            return;
        }
        // Stub Newton: x_{k+1} = x_k - x_k (converges to 0 for f(x)=x)
        self.root = start.to_vec();
        let mut iter = 0;
        loop {
            let norm: f64 = self.root.iter().map(|x| x*x).sum::<f64>().sqrt();
            if norm < self.tolerance || iter >= self.max_iter { break; }
            for x in &mut self.root { *x *= 0.5; }
            iter += 1;
        }
        self.residual = self.root.clone();
        self.nb_iterations = iter;
        let norm: f64 = self.root.iter().map(|x| x*x).sum::<f64>().sqrt();
        self.status = if norm < self.tolerance { MathSolverStatus::Done } else { MathSolverStatus::NoConvergence };
    }

    pub fn is_done(&self) -> bool { self.status == MathSolverStatus::Done }
    pub fn root(&self) -> &[f64] { &self.root }
    pub fn nb_iterations(&self) -> usize { self.nb_iterations }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfgs_converges() {
        let mut b = MathBfgs::new(2);
        b.perform(&[4.0, 3.0]);
        assert!(b.is_done());
        assert!(b.minimum() < 1e-8);
        assert!(b.nb_iterations() > 0);
    }

    #[test]
    fn bfgs_wrong_dim() {
        let mut b = MathBfgs::new(2);
        b.perform(&[1.0]);
        assert_eq!(b.status, MathSolverStatus::InvalidInput);
    }

    #[test]
    fn glob_opt_midpoint() {
        let mut g = MathGlobOptMin::new(2);
        g.set_bounds(&[-2.0, -2.0], &[2.0, 2.0]);
        g.perform();
        assert!(g.is_done());
        assert!(g.solution()[0].abs() < 1e-10);  // midpoint of [-2,2] = 0
        assert_eq!(g.nb_solutions(), 1);
    }

    #[test]
    fn function_set_root() {
        let mut r = MathFunctionSetRoot::new(3);
        r.perform(&[8.0, 4.0, 2.0]);
        assert!(r.is_done());
        for &x in r.root() { assert!(x.abs() < 1e-10); }
    }

    #[test]
    fn function_set_root_wrong_dim() {
        let mut r = MathFunctionSetRoot::new(2);
        r.perform(&[1.0]);
        assert_eq!(r.status, MathSolverStatus::InvalidInput);
    }
}
