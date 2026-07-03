// FILE: hlrb_rep_surface.rs
// occt: HLRBRep_Surface

/// Adapts a TopoDS_Face surface for HLR processing.
/// Stores surface data and provides query methods for properties and evaluations.
#[derive(Clone, Debug)]
pub struct HLRBRepSurface {
    first_u: f64,
    last_u: f64,
    first_v: f64,
    last_v: f64,
    is_u_closed: bool,
    is_v_closed: bool,
    is_u_periodic: bool,
    is_v_periodic: bool,
    u_period: f64,
    v_period: f64,
}

impl HLRBRepSurface {
    /// Creates an undefined surface with no face loaded.
    pub fn new() -> Self {
        HLRBRepSurface {
            first_u: 0.0,
            last_u: 0.0,
            first_v: 0.0,
            last_v: 0.0,
            is_u_closed: false,
            is_v_closed: false,
            is_u_periodic: false,
            is_v_periodic: false,
            u_period: 0.0,
            v_period: 0.0,
        }
    }

    pub fn set_first_u(&mut self, v: f64) {
        self.first_u = v;
    }

    pub fn set_last_u(&mut self, v: f64) {
        self.last_u = v;
    }

    pub fn set_first_v(&mut self, v: f64) {
        self.first_v = v;
    }

    pub fn set_last_v(&mut self, v: f64) {
        self.last_v = v;
    }

    pub fn first_u_parameter(&self) -> f64 {
        self.first_u
    }

    pub fn last_u_parameter(&self) -> f64 {
        self.last_u
    }

    pub fn first_v_parameter(&self) -> f64 {
        self.first_v
    }

    pub fn last_v_parameter(&self) -> f64 {
        self.last_v
    }

    pub fn set_u_closed(&mut self, b: bool) {
        self.is_u_closed = b;
    }

    pub fn set_v_closed(&mut self, b: bool) {
        self.is_v_closed = b;
    }

    pub fn is_u_closed(&self) -> bool {
        self.is_u_closed
    }

    pub fn is_v_closed(&self) -> bool {
        self.is_v_closed
    }

    pub fn set_u_periodic(&mut self, b: bool) {
        self.is_u_periodic = b;
    }

    pub fn set_v_periodic(&mut self, b: bool) {
        self.is_v_periodic = b;
    }

    pub fn is_u_periodic(&self) -> bool {
        self.is_u_periodic
    }

    pub fn is_v_periodic(&self) -> bool {
        self.is_v_periodic
    }

    pub fn set_u_period(&mut self, p: f64) {
        self.u_period = p;
    }

    pub fn set_v_period(&mut self, p: f64) {
        self.v_period = p;
    }

    pub fn u_period(&self) -> f64 {
        self.u_period
    }

    pub fn v_period(&self) -> f64 {
        self.v_period
    }

    /// Returns (x, y, z) for parameters u, v on surface.
    pub fn value(&self, _u: f64, _v: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0) // placeholder
    }

    /// Computes point at (u,v).
    pub fn d0(&self, u: f64, v: f64) -> (f64, f64, f64) {
        self.value(u, v)
    }

    /// Computes point and first derivatives at (u,v).
    pub fn d1(&self, u: f64, v: f64) -> ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) {
        let p = self.value(u, v);
        (p, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0))
    }

    /// Returns surface type: 0=Plane, 1=Cylinder, 2=Cone, 3=Sphere, 4=Torus, etc.
    pub fn get_type(&self) -> i32 {
        0
    }

    /// Returns number of U intervals.
    pub fn nb_u_intervals(&self, _shape: i32) -> i32 {
        1
    }

    /// Returns number of V intervals.
    pub fn nb_v_intervals(&self, _shape: i32) -> i32 {
        1
    }
}

impl Default for HLRBRepSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let s = HLRBRepSurface::new();
        assert_eq!(s.first_u_parameter(), 0.0);
        assert_eq!(s.first_v_parameter(), 0.0);
    }

    #[test]
    fn test_set_bounds() {
        let mut s = HLRBRepSurface::new();
        s.set_first_u(0.0);
        s.set_last_u(1.0);
        s.set_first_v(0.0);
        s.set_last_v(2.0);
        assert_eq!(s.first_u_parameter(), 0.0);
        assert_eq!(s.last_u_parameter(), 1.0);
        assert_eq!(s.last_v_parameter(), 2.0);
    }

    #[test]
    fn test_closedness() {
        let mut s = HLRBRepSurface::new();
        s.set_u_closed(true);
        s.set_v_closed(false);
        assert!(s.is_u_closed());
        assert!(!s.is_v_closed());
    }

    #[test]
    fn test_periodicity() {
        let mut s = HLRBRepSurface::new();
        s.set_u_periodic(true);
        s.set_u_period(6.283185307);
        assert!(s.is_u_periodic());
        assert!((s.u_period() - 6.283185307).abs() < 1e-6);
    }
}
