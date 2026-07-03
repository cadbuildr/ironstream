// FILE: app_def_variational.rs
// occt: AppDef_Variational

/// Variational optimization for curve approximation.
/// Smooths N points with constraints by minimization of quadratic/variational criteria.
pub struct AppDefVariational {
    is_created: bool,
    is_done: bool,
    is_over_constrained: bool,
    max_error: f64,
    max_error_index: i32,
    average_error: f64,
    max_degree: i32,
    max_segment: i32,
    nb_iterations: i32,
    tolerance: f64,
    with_min_max: bool,
    with_cutting: bool,
    criterium: [f64; 4],
    percent: [f64; 3],
}

impl AppDefVariational {
    /// Creates a new variational approximation solver.
    pub fn new() -> Self {
        AppDefVariational {
            is_created: false,
            is_done: false,
            is_over_constrained: false,
            max_error: 0.0,
            max_error_index: 0,
            average_error: 0.0,
            max_degree: 14,
            max_segment: 100,
            nb_iterations: 2,
            tolerance: 1.0,
            with_min_max: false,
            with_cutting: true,
            criterium: [0.0; 4],
            percent: [0.0; 3],
        }
    }

    /// Returns True if the creation is done and corresponds to current fields.
    pub fn is_created(&self) -> bool {
        self.is_created
    }

    /// Returns True if the approximation succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns True if the problem is overconstrained.
    pub fn is_over_constrained(&self) -> bool {
        self.is_over_constrained
    }

    /// Returns the maximum error between approximation and data points.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    /// Returns the index of the MultiPoint with ErrorMax.
    pub fn max_error_index(&self) -> i32 {
        self.max_error_index
    }

    /// Returns the quadratic average error.
    pub fn quadratic_error(&self) -> f64 {
        self.criterium[0]
    }

    /// Returns the average error between MultiLine and approximation.
    pub fn average_error(&self) -> f64 {
        self.average_error
    }

    /// Returns the maximum degree used in approximation.
    pub fn max_degree(&self) -> i32 {
        self.max_degree
    }

    /// Returns the maximum number of segments used.
    pub fn max_segment(&self) -> i32 {
        self.max_segment
    }

    /// Returns if approximation searches to minimize maximum error.
    pub fn with_min_max(&self) -> bool {
        self.with_min_max
    }

    /// Returns if approximation can insert new knots.
    pub fn with_cutting(&self) -> bool {
        self.with_cutting
    }

    /// Returns the tolerance used in approximation.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Returns the number of iterations.
    pub fn nb_iterations(&self) -> i32 {
        self.nb_iterations
    }

    /// Sets the created flag.
    pub fn set_created(&mut self, val: bool) {
        self.is_created = val;
    }

    /// Sets the done flag.
    pub fn set_done(&mut self, val: bool) {
        self.is_done = val;
    }

    /// Sets the over-constrained flag.
    pub fn set_over_constrained(&mut self, val: bool) {
        self.is_over_constrained = val;
    }

    /// Sets the maximum error.
    pub fn set_max_error(&mut self, err: f64) {
        self.max_error = err;
    }

    /// Sets the index of maximum error.
    pub fn set_max_error_index(&mut self, idx: i32) {
        self.max_error_index = idx;
    }

    /// Sets the average error.
    pub fn set_average_error(&mut self, err: f64) {
        self.average_error = err;
    }

    /// Sets maximum degree.
    pub fn set_max_degree(&mut self, deg: i32) {
        self.max_degree = deg;
    }

    /// Sets maximum segments.
    pub fn set_max_segment(&mut self, seg: i32) {
        self.max_segment = seg;
    }

    /// Sets with_min_max flag.
    pub fn set_with_min_max(&mut self, val: bool) {
        self.with_min_max = val;
    }

    /// Sets with_cutting flag.
    pub fn set_with_cutting(&mut self, val: bool) {
        self.with_cutting = val;
    }

    /// Sets tolerance.
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    /// Sets number of iterations.
    pub fn set_nb_iterations(&mut self, nb: i32) {
        self.nb_iterations = nb;
    }

    /// Sets criterium values.
    pub fn set_criterium(&mut self, idx: usize, val: f64) {
        if idx < 4 {
            self.criterium[idx] = val;
        }
    }

    /// Sets percent values.
    pub fn set_percent(&mut self, idx: usize, val: f64) {
        if idx < 3 {
            self.percent[idx] = val;
        }
    }
}

impl Default for AppDefVariational {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let var = AppDefVariational::new();
        assert!(!var.is_created());
        assert!(!var.is_done());
        assert!(!var.is_over_constrained());
        assert_eq!(var.max_degree(), 14);
        assert_eq!(var.max_segment(), 100);
    }

    #[test]
    fn test_flags() {
        let mut var = AppDefVariational::new();
        var.set_created(true);
        var.set_done(true);
        var.set_over_constrained(true);
        assert!(var.is_created());
        assert!(var.is_done());
        assert!(var.is_over_constrained());
    }

    #[test]
    fn test_parameters() {
        let mut var = AppDefVariational::new();
        var.set_max_error(0.001);
        var.set_max_error_index(5);
        var.set_average_error(0.0005);
        assert_eq!(var.max_error(), 0.001);
        assert_eq!(var.max_error_index(), 5);
        assert_eq!(var.average_error(), 0.0005);
    }

    #[test]
    fn test_settings() {
        let mut var = AppDefVariational::new();
        var.set_max_degree(12);
        var.set_max_segment(50);
        var.set_tolerance(1.0e-6);
        var.set_nb_iterations(3);
        assert_eq!(var.max_degree(), 12);
        assert_eq!(var.max_segment(), 50);
        assert_eq!(var.tolerance(), 1.0e-6);
        assert_eq!(var.nb_iterations(), 3);
    }
}
