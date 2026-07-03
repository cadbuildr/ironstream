// FILE: l_prop_cl_props3d.rs
// occt: LProp_CLProps3d

/// Type alias for curve local properties using 3D interface.
/// This is a simplified representation of the template-based OCCT class.
pub struct LPropCLProps3d {
    u: f64,
    p: (f64, f64, f64),
    d1: (f64, f64, f64),
    d2: (f64, f64, f64),
    d3: (f64, f64, f64),
}

impl LPropCLProps3d {
    /// Create a new curve local properties object.
    pub fn new(u: f64) -> Self {
        LPropCLProps3d {
            u,
            p: (0.0, 0.0, 0.0),
            d1: (0.0, 0.0, 0.0),
            d2: (0.0, 0.0, 0.0),
            d3: (0.0, 0.0, 0.0),
        }
    }

    /// Get the parameter value.
    pub fn u(&self) -> f64 {
        self.u
    }

    /// Set the position.
    pub fn set_point(&mut self, p: (f64, f64, f64)) {
        self.p = p;
    }

    /// Get the position.
    pub fn point(&self) -> (f64, f64, f64) {
        self.p
    }

    /// Set the first derivative.
    pub fn set_d1(&mut self, d1: (f64, f64, f64)) {
        self.d1 = d1;
    }

    /// Get the first derivative.
    pub fn d1(&self) -> (f64, f64, f64) {
        self.d1
    }

    /// Set the second derivative.
    pub fn set_d2(&mut self, d2: (f64, f64, f64)) {
        self.d2 = d2;
    }

    /// Get the second derivative.
    pub fn d2(&self) -> (f64, f64, f64) {
        self.d2
    }

    /// Set the third derivative.
    pub fn set_d3(&mut self, d3: (f64, f64, f64)) {
        self.d3 = d3;
    }

    /// Get the third derivative.
    pub fn d3(&self) -> (f64, f64, f64) {
        self.d3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_clprops3d() {
        let props = LPropCLProps3d::new(1.5);
        assert_eq!(props.u(), 1.5);
    }

    #[test]
    fn test_set_and_get_point() {
        let mut props = LPropCLProps3d::new(0.0);
        props.set_point((1.0, 2.0, 3.0));
        assert_eq!(props.point(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_set_and_get_derivatives() {
        let mut props = LPropCLProps3d::new(0.0);
        props.set_d1((1.0, 0.0, 0.0));
        props.set_d2((0.0, 1.0, 0.0));
        props.set_d3((0.0, 0.0, 1.0));

        assert_eq!(props.d1(), (1.0, 0.0, 0.0));
        assert_eq!(props.d2(), (0.0, 1.0, 0.0));
        assert_eq!(props.d3(), (0.0, 0.0, 1.0));
    }
}
