// FILE: top_ope_b_rep_p_pnt_on2_s.rs
// occt: TopOpeBRep_PPntOn2S

/// Pointer type alias for PntOn2S (point on 2 surfaces).
pub type PPntOn2S = Option<Box<PntOn2S>>;

/// Point on 2 surfaces structure.
pub struct PntOn2S {
    u1: f64,
    v1: f64,
    u2: f64,
    v2: f64,
}

impl PntOn2S {
    /// Create a new point on 2 surfaces.
    pub fn new(u1: f64, v1: f64, u2: f64, v2: f64) -> Self {
        PntOn2S { u1, v1, u2, v2 }
    }

    /// Get first surface u parameter.
    pub fn u1(&self) -> f64 {
        self.u1
    }

    /// Get first surface v parameter.
    pub fn v1(&self) -> f64 {
        self.v1
    }

    /// Get second surface u parameter.
    pub fn u2(&self) -> f64 {
        self.u2
    }

    /// Get second surface v parameter.
    pub fn v2(&self) -> f64 {
        self.v2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pt = PntOn2S::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(pt.u1(), 1.0);
        assert_eq!(pt.v1(), 2.0);
        assert_eq!(pt.u2(), 3.0);
        assert_eq!(pt.v2(), 4.0);
    }

    #[test]
    fn test_pointer_type() {
        let p: PPntOn2S = Some(Box::new(PntOn2S::new(1.0, 2.0, 3.0, 4.0)));
        assert!(p.is_some());
        if let Some(pt) = p {
            assert_eq!(pt.u1(), 1.0);
        }
    }

    #[test]
    fn test_null_pointer() {
        let p: PPntOn2S = None;
        assert!(p.is_none());
    }
}
