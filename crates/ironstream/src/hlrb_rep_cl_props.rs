// FILE: hlrb_rep_cl_props.rs
// occt: HLRBRep_CLProps

/// Curve line properties (tangent, curvature, etc).
pub struct HlrbrépCLProps {
    param: f64,
    is_singular: bool,
    tangent: [f64; 3],
    curvature: f64,
}

impl HlrbrépCLProps {
    pub fn new() -> Self {
        HlrbrépCLProps {
            param: 0.0,
            is_singular: false,
            tangent: [0.0, 0.0, 1.0],
            curvature: 0.0,
        }
    }

    pub fn set_parameter(&mut self, p: f64) { self.param = p; }
    pub fn parameter(&self) -> f64 { self.param }
    pub fn set_singular(&mut self, s: bool) { self.is_singular = s; }
    pub fn is_singular(&self) -> bool { self.is_singular }
    pub fn set_tangent(&mut self, t: [f64; 3]) { self.tangent = t; }
    pub fn tangent(&self) -> [f64; 3] { self.tangent }
    pub fn set_curvature(&mut self, c: f64) { self.curvature = c; }
    pub fn curvature(&self) -> f64 { self.curvature }
}

impl Default for HlrbrépCLProps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let cp = HlrbrépCLProps::new();
        assert_eq!(cp.parameter(), 0.0);
        assert!(!cp.is_singular());
    }
}
