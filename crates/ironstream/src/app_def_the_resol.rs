// FILE: app_def_the_resol.rs
// occt: AppDef_TheResol

/// Solver for constrained curve approximation using Uzawa algorithm.
/// Finds the best curve solution approximating a MultiLine with constraints.
pub struct AppDefTheResol {
    done: bool,
    err: f64,
    cont: Vec<Vec<f64>>,           // Constraint matrix
    de_cont: Vec<Vec<f64>>,        // Derivative constraint matrix
    secont: Vec<f64>,              // Secondary constraint
    ct_cinv: Vec<Vec<f64>>,        // Inverse of Cont*Transposed(Cont)
    vardua: Vec<f64>,              // Dual variables
    inc_pass: i32,
    inc_tan: i32,
    inc_curv: i32,
    i_pas: Vec<i32>,
    i_tan: Vec<i32>,
    i_curv: Vec<i32>,
}

impl AppDefTheResol {
    /// Creates a new solver with given parameters.
    pub fn new() -> Self {
        AppDefTheResol {
            done: false,
            err: 0.0,
            cont: Vec::new(),
            de_cont: Vec::new(),
            secont: Vec::new(),
            ct_cinv: Vec::new(),
            vardua: Vec::new(),
            inc_pass: 0,
            inc_tan: 0,
            inc_curv: 0,
            i_pas: Vec::new(),
            i_tan: Vec::new(),
            i_curv: Vec::new(),
        }
    }

    /// Indicates if all computation completed successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the maximum difference value between the curve and given points.
    pub fn error(&self) -> f64 {
        self.err
    }

    /// Returns the constraint matrix.
    pub fn constraint_matrix(&self) -> &[Vec<f64>] {
        &self.cont
    }

    /// Returns the dual variables of the system.
    pub fn duale(&self) -> &[f64] {
        &self.vardua
    }

    /// Returns the Inverse of Cont*Transposed(Cont).
    pub fn inverse_matrix(&self) -> &[Vec<f64>] {
        &self.ct_cinv
    }

    /// Sets the done flag.
    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    /// Sets the error value.
    pub fn set_error(&mut self, err: f64) {
        self.err = err;
    }

    /// Sets the constraint matrix.
    pub fn set_constraint_matrix(&mut self, cont: Vec<Vec<f64>>) {
        self.cont = cont;
    }

    /// Sets the dual variables.
    pub fn set_duale(&mut self, vardua: Vec<f64>) {
        self.vardua = vardua;
    }

    /// Sets the inverse matrix.
    pub fn set_inverse_matrix(&mut self, ct_cinv: Vec<Vec<f64>>) {
        self.ct_cinv = ct_cinv;
    }
}

impl Default for AppDefTheResol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = AppDefTheResol::new();
        assert!(!solver.is_done());
        assert_eq!(solver.error(), 0.0);
        assert!(solver.constraint_matrix().is_empty());
        assert!(solver.duale().is_empty());
    }

    #[test]
    fn test_set_done() {
        let mut solver = AppDefTheResol::new();
        solver.set_done(true);
        assert!(solver.is_done());
    }

    #[test]
    fn test_set_error() {
        let mut solver = AppDefTheResol::new();
        solver.set_error(1.5e-3);
        assert_eq!(solver.error(), 1.5e-3);
    }

    #[test]
    fn test_matrices() {
        let mut solver = AppDefTheResol::new();
        let cont = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        solver.set_constraint_matrix(cont.clone());
        assert_eq!(solver.constraint_matrix(), &cont);
    }
}
