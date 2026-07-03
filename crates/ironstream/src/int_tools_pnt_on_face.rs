// FILE: int_tools_pnt_on_face.rs
// occt: IntTools_PntOnFace

/// Point on a face with UV parameters.
#[derive(Clone, Copy, Debug)]
pub struct IntToolsPntOnFace {
    is_valid: bool,
    u: f64,
    v: f64,
}

impl IntToolsPntOnFace {
    /// Create an empty point on face.
    pub fn new() -> Self {
        IntToolsPntOnFace {
            is_valid: false,
            u: 0.0,
            v: 0.0,
        }
    }

    /// Initialize with face and parameters.
    pub fn init(u: f64, v: f64) -> Self {
        IntToolsPntOnFace {
            is_valid: true,
            u,
            v,
        }
    }

    /// Set the parameters.
    pub fn set_parameters(&mut self, u: f64, v: f64) {
        self.u = u;
        self.v = v;
    }

    /// Set the validity flag.
    pub fn set_valid(&mut self, valid: bool) {
        self.is_valid = valid;
    }

    /// Get the validity flag.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Get the validity flag (alternative name).
    pub fn valid(&self) -> bool {
        self.is_valid
    }

    /// Get the parameters.
    pub fn parameters(&self) -> (f64, f64) {
        (self.u, self.v)
    }

    /// Get U parameter.
    pub fn u(&self) -> f64 {
        self.u
    }

    /// Get V parameter.
    pub fn v(&self) -> f64 {
        self.v
    }
}

impl Default for IntToolsPntOnFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pnt = IntToolsPntOnFace::new();
        assert!(!pnt.is_valid());
        assert_eq!(pnt.u(), 0.0);
        assert_eq!(pnt.v(), 0.0);
    }

    #[test]
    fn test_init() {
        let pnt = IntToolsPntOnFace::init(1.5, 2.5);
        assert!(pnt.is_valid());
        assert_eq!(pnt.u(), 1.5);
        assert_eq!(pnt.v(), 2.5);
    }

    #[test]
    fn test_set_parameters() {
        let mut pnt = IntToolsPntOnFace::new();
        pnt.set_parameters(3.0, 4.0);
        let (u, v) = pnt.parameters();
        assert_eq!(u, 3.0);
        assert_eq!(v, 4.0);
    }

    #[test]
    fn test_set_valid() {
        let mut pnt = IntToolsPntOnFace::new();
        pnt.set_valid(true);
        assert!(pnt.is_valid());
        assert!(pnt.valid());
    }
}
