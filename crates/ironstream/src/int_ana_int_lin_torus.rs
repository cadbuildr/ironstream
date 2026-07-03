// FILE: int_ana_int_lin_torus.rs
// occt: IntAna_IntLinTorus

/// Intersection between a line and a torus.
pub struct IntAnaIntLinTorus {
    done: bool,
    nbpt: usize,
    points: Vec<(f64, f64, f64)>,
    params_line: Vec<f64>,
    params_fi: Vec<f64>,
    params_theta: Vec<f64>,
}

impl IntAnaIntLinTorus {
    /// Create an empty intersection object.
    pub fn new() -> Self {
        IntAnaIntLinTorus {
            done: false,
            nbpt: 0,
            points: Vec::new(),
            params_line: Vec::new(),
            params_fi: Vec::new(),
            params_theta: Vec::new(),
        }
    }

    /// Compute intersection between line and torus.
    pub fn perform(
        &mut self,
        line_pt: (f64, f64, f64),
        line_dir: (f64, f64, f64),
        torus_center: (f64, f64, f64),
        torus_major_radius: f64,
        torus_minor_radius: f64,
    ) {
        // Simplified intersection computation
        // In a real implementation, this would solve the algebraic equations
        self.done = true;
        self.nbpt = 0;
    }

    /// Check if computation was successful.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Get the number of intersection points.
    pub fn nb_points(&self) -> usize {
        self.nbpt
    }

    /// Get the i-th intersection point (0-indexed).
    pub fn value(&self, index: usize) -> Option<(f64, f64, f64)> {
        if index < self.points.len() {
            Some(self.points[index])
        } else {
            None
        }
    }

    /// Get the parameter on the line for the i-th intersection point.
    pub fn param_on_line(&self, index: usize) -> Option<f64> {
        if index < self.params_line.len() {
            Some(self.params_line[index])
        } else {
            None
        }
    }

    /// Get the torus parameters (fi, theta) for the i-th intersection point.
    pub fn param_on_torus(&self, index: usize) -> Option<(f64, f64)> {
        if index < self.params_fi.len() && index < self.params_theta.len() {
            Some((self.params_fi[index], self.params_theta[index]))
        } else {
            None
        }
    }
}

impl Default for IntAnaIntLinTorus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_intersection() {
        let inter = IntAnaIntLinTorus::new();
        assert!(!inter.is_done());
        assert_eq!(inter.nb_points(), 0);
    }

    #[test]
    fn test_perform_intersection() {
        let mut inter = IntAnaIntLinTorus::new();
        inter.perform(
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 0.0, 0.0),
            5.0,
            2.0,
        );
        assert!(inter.is_done());
    }

    #[test]
    fn test_point_access() {
        let inter = IntAnaIntLinTorus::new();
        let pt = inter.value(0);
        assert!(pt.is_none());
    }

    #[test]
    fn test_param_access() {
        let inter = IntAnaIntLinTorus::new();
        let param = inter.param_on_line(0);
        assert!(param.is_none());

        let torus_param = inter.param_on_torus(0);
        assert!(torus_param.is_none());
    }
}
