// FILE: approx_sweep_approximation.rs
// occt: Approx_SweepApproximation

/// Approximates a surface defined by section laws.
pub struct ApproxSweepApproximation {
    is_done: bool,
    u_degree: i32,
    v_degree: i32,
    nb_u_poles: i32,
    nb_v_poles: i32,
    nb_u_knots: i32,
    nb_v_knots: i32,
}

impl ApproxSweepApproximation {
    /// Creates a new sweep approximator.
    pub fn new() -> Self {
        ApproxSweepApproximation {
            is_done: false,
            u_degree: 0,
            v_degree: 0,
            nb_u_poles: 0,
            nb_v_poles: 0,
            nb_u_knots: 0,
            nb_v_knots: 0,
        }
    }

    /// Indicates if approximation succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns surface shape parameters.
    pub fn surf_shape(&self) -> (i32, i32, i32, i32, i32, i32) {
        (
            self.u_degree,
            self.v_degree,
            self.nb_u_poles,
            self.nb_v_poles,
            self.nb_u_knots,
            self.nb_v_knots,
        )
    }

    /// Returns U degree.
    pub fn u_degree(&self) -> i32 {
        self.u_degree
    }

    /// Returns V degree.
    pub fn v_degree(&self) -> i32 {
        self.v_degree
    }

    /// Returns number of U poles.
    pub fn nb_u_poles(&self) -> i32 {
        self.nb_u_poles
    }

    /// Returns number of V poles.
    pub fn nb_v_poles(&self) -> i32 {
        self.nb_v_poles
    }

    /// Returns number of U knots.
    pub fn nb_u_knots(&self) -> i32 {
        self.nb_u_knots
    }

    /// Returns number of V knots.
    pub fn nb_v_knots(&self) -> i32 {
        self.nb_v_knots
    }

    /// Sets done flag.
    pub fn set_done(&mut self, done: bool) {
        self.is_done = done;
    }

    /// Sets surface shape.
    pub fn set_surf_shape(
        &mut self,
        u_deg: i32,
        v_deg: i32,
        u_poles: i32,
        v_poles: i32,
        u_knots: i32,
        v_knots: i32,
    ) {
        self.u_degree = u_deg;
        self.v_degree = v_deg;
        self.nb_u_poles = u_poles;
        self.nb_v_poles = v_poles;
        self.nb_u_knots = u_knots;
        self.nb_v_knots = v_knots;
    }
}

impl Default for ApproxSweepApproximation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sweep = ApproxSweepApproximation::new();
        assert!(!sweep.is_done());
        assert_eq!(sweep.u_degree(), 0);
        assert_eq!(sweep.v_degree(), 0);
    }

    #[test]
    fn test_surf_shape() {
        let mut sweep = ApproxSweepApproximation::new();
        sweep.set_surf_shape(3, 2, 10, 8, 7, 5);
        let (ud, vd, up, vp, uk, vk) = sweep.surf_shape();
        assert_eq!(ud, 3);
        assert_eq!(vd, 2);
        assert_eq!(up, 10);
        assert_eq!(vp, 8);
        assert_eq!(uk, 7);
        assert_eq!(vk, 5);
    }

    #[test]
    fn test_done() {
        let mut sweep = ApproxSweepApproximation::new();
        sweep.set_done(true);
        assert!(sweep.is_done());
    }
}
