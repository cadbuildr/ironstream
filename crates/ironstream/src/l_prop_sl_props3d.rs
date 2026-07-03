// FILE: l_prop_sl_props3d.rs
// occt: LProp_SLProps3d

/// Surface local properties in 3D.
pub struct LPropSLProps3d {
    u: f64,
    v: f64,
    p: (f64, f64, f64),
    d1u: (f64, f64, f64),
    d1v: (f64, f64, f64),
    d2u: (f64, f64, f64),
    d2v: (f64, f64, f64),
    duv: (f64, f64, f64),
}

impl LPropSLProps3d {
    /// Create a new surface local properties object.
    pub fn new(u: f64, v: f64) -> Self {
        LPropSLProps3d {
            u,
            v,
            p: (0.0, 0.0, 0.0),
            d1u: (0.0, 0.0, 0.0),
            d1v: (0.0, 0.0, 0.0),
            d2u: (0.0, 0.0, 0.0),
            d2v: (0.0, 0.0, 0.0),
            duv: (0.0, 0.0, 0.0),
        }
    }

    /// Get the U parameter.
    pub fn u(&self) -> f64 {
        self.u
    }

    /// Get the V parameter.
    pub fn v(&self) -> f64 {
        self.v
    }

    /// Set the position.
    pub fn set_point(&mut self, p: (f64, f64, f64)) {
        self.p = p;
    }

    /// Get the position.
    pub fn point(&self) -> (f64, f64, f64) {
        self.p
    }

    /// Set the first U derivative.
    pub fn set_d1u(&mut self, d1u: (f64, f64, f64)) {
        self.d1u = d1u;
    }

    /// Get the first U derivative.
    pub fn d1u(&self) -> (f64, f64, f64) {
        self.d1u
    }

    /// Set the first V derivative.
    pub fn set_d1v(&mut self, d1v: (f64, f64, f64)) {
        self.d1v = d1v;
    }

    /// Get the first V derivative.
    pub fn d1v(&self) -> (f64, f64, f64) {
        self.d1v
    }

    /// Set the second U derivative.
    pub fn set_d2u(&mut self, d2u: (f64, f64, f64)) {
        self.d2u = d2u;
    }

    /// Get the second U derivative.
    pub fn d2u(&self) -> (f64, f64, f64) {
        self.d2u
    }

    /// Set the second V derivative.
    pub fn set_d2v(&mut self, d2v: (f64, f64, f64)) {
        self.d2v = d2v;
    }

    /// Get the second V derivative.
    pub fn d2v(&self) -> (f64, f64, f64) {
        self.d2v
    }

    /// Set the UV derivative.
    pub fn set_duv(&mut self, duv: (f64, f64, f64)) {
        self.duv = duv;
    }

    /// Get the UV derivative.
    pub fn duv(&self) -> (f64, f64, f64) {
        self.duv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_slprops3d() {
        let props = LPropSLProps3d::new(0.5, 0.5);
        assert_eq!(props.u(), 0.5);
        assert_eq!(props.v(), 0.5);
    }

    #[test]
    fn test_set_and_get_all_derivatives() {
        let mut props = LPropSLProps3d::new(0.0, 0.0);
        props.set_d1u((1.0, 0.0, 0.0));
        props.set_d1v((0.0, 1.0, 0.0));
        props.set_d2u((0.0, 1.0, 0.0));
        props.set_d2v((0.0, 0.0, 1.0));
        props.set_duv((0.0, 0.0, 0.0));

        assert_eq!(props.d1u(), (1.0, 0.0, 0.0));
        assert_eq!(props.d1v(), (0.0, 1.0, 0.0));
        assert_eq!(props.duv(), (0.0, 0.0, 0.0));
    }
}
