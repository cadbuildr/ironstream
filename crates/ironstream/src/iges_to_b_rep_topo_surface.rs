// FILE: iges_to_b_rep_topo_surface.rs
// occt: IGESToBRep_TopoSurface

/// Provides methods to transfer topologic surfaces entities from IGES to CASCADE.
/// This is a conversion tool for handling various IGES surface types.
pub struct IGESToBRepTopoSurface {
    /// Unit length factor for conversion
    the_u_length: f64,
    /// Epsilon tolerance
    eps: f64,
    /// Geometric epsilon
    eps_geom: f64,
    /// Coefficient epsilon
    eps_coeff: f64,
    /// Topological mode flag
    mode_topo: bool,
    /// Approximation mode flag
    mode_approx: bool,
    /// Optimization flag
    optimized: bool,
}

impl IGESToBRepTopoSurface {
    /// Creates a tool ready to run, with epsilons set to 1.E-04, TheModeTopo to True,
    /// the optimization of the continuity to False.
    pub fn new() -> Self {
        Self {
            the_u_length: 1.0,
            eps: 1e-4,
            eps_geom: 1e-4,
            eps_coeff: 1e-4,
            mode_topo: true,
            mode_approx: false,
            optimized: false,
        }
    }

    /// Creates a tool with explicit epsilon values and mode flags.
    pub fn with_params(
        eps: f64,
        eps_geom: f64,
        eps_coeff: f64,
        mode_topo: bool,
        mode_approx: bool,
        optimized: bool,
    ) -> Self {
        Self {
            the_u_length: 1.0,
            eps,
            eps_geom,
            eps_coeff,
            mode_topo,
            mode_approx,
            optimized,
        }
    }

    /// Returns the unit length factor
    pub fn u_length(&self) -> f64 {
        self.the_u_length
    }

    /// Sets the unit length factor
    pub fn set_u_length(&mut self, length: f64) {
        self.the_u_length = length;
    }

    /// Returns the epsilon tolerance
    pub fn epsilon(&self) -> f64 {
        self.eps
    }

    /// Sets the epsilon tolerance
    pub fn set_epsilon(&mut self, eps: f64) {
        self.eps = eps;
    }

    /// Returns the geometric epsilon
    pub fn eps_geom(&self) -> f64 {
        self.eps_geom
    }

    /// Sets the geometric epsilon
    pub fn set_eps_geom(&mut self, eps_geom: f64) {
        self.eps_geom = eps_geom;
    }

    /// Returns the coefficient epsilon
    pub fn eps_coeff(&self) -> f64 {
        self.eps_coeff
    }

    /// Sets the coefficient epsilon
    pub fn set_eps_coeff(&mut self, eps_coeff: f64) {
        self.eps_coeff = eps_coeff;
    }

    /// Returns the topological mode flag
    pub fn mode_topo(&self) -> bool {
        self.mode_topo
    }

    /// Sets the topological mode flag
    pub fn set_mode_topo(&mut self, mode: bool) {
        self.mode_topo = mode;
    }

    /// Returns the approximation mode flag
    pub fn mode_approx(&self) -> bool {
        self.mode_approx
    }

    /// Sets the approximation mode flag
    pub fn set_mode_approx(&mut self, mode: bool) {
        self.mode_approx = mode;
    }

    /// Returns the optimization flag
    pub fn optimized(&self) -> bool {
        self.optimized
    }

    /// Sets the optimization flag
    pub fn set_optimized(&mut self, opt: bool) {
        self.optimized = opt;
    }
}

impl Default for IGESToBRepTopoSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_defaults() {
        let ts = IGESToBRepTopoSurface::new();
        assert_eq!(ts.epsilon(), 1e-4);
        assert_eq!(ts.eps_geom(), 1e-4);
        assert_eq!(ts.eps_coeff(), 1e-4);
        assert!(ts.mode_topo());
        assert!(!ts.mode_approx());
        assert!(!ts.optimized());
        assert_eq!(ts.u_length(), 1.0);
    }

    #[test]
    fn test_with_params() {
        let ts = IGESToBRepTopoSurface::with_params(0.001, 0.002, 0.003, false, true, true);
        assert_eq!(ts.epsilon(), 0.001);
        assert_eq!(ts.eps_geom(), 0.002);
        assert_eq!(ts.eps_coeff(), 0.003);
        assert!(!ts.mode_topo());
        assert!(ts.mode_approx());
        assert!(ts.optimized());
    }

    #[test]
    fn test_setters() {
        let mut ts = IGESToBRepTopoSurface::new();

        ts.set_epsilon(0.005);
        assert_eq!(ts.epsilon(), 0.005);

        ts.set_eps_geom(0.006);
        assert_eq!(ts.eps_geom(), 0.006);

        ts.set_eps_coeff(0.007);
        assert_eq!(ts.eps_coeff(), 0.007);

        ts.set_u_length(2.5);
        assert_eq!(ts.u_length(), 2.5);

        ts.set_mode_topo(false);
        assert!(!ts.mode_topo());

        ts.set_mode_approx(true);
        assert!(ts.mode_approx());

        ts.set_optimized(true);
        assert!(ts.optimized());
    }

    #[test]
    fn test_default_trait() {
        let ts = IGESToBRepTopoSurface::default();
        assert_eq!(ts.epsilon(), 1e-4);
        assert!(ts.mode_topo());
    }
}
