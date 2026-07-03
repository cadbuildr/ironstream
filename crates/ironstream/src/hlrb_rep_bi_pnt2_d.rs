// FILE: hlrb_rep_bi_pnt2_d.rs
// occt: HLRBRep_BiPnt2D

/// A segment in 2D space with visibility information.
#[derive(Clone)]
pub struct HlrbrépBiPnt2D {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    flags: i32,
}

impl HlrbrépBiPnt2D {
    pub fn new() -> Self {
        HlrbrépBiPnt2D {
            x1: 0.0, y1: 0.0, x2: 0.0, y2: 0.0, flags: 0,
        }
    }

    pub fn with_points(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        HlrbrépBiPnt2D { x1, y1, x2, y2, flags: 0 }
    }

    pub fn pnt1(&self) -> (f64, f64) { (self.x1, self.y1) }
    pub fn pnt2(&self) -> (f64, f64) { (self.x2, self.y2) }
    pub fn set_flags(&mut self, f: i32) { self.flags = f; }
    pub fn flags(&self) -> i32 { self.flags }
}

impl Default for HlrbrépBiPnt2D {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let bp = HlrbrépBiPnt2D::new();
        assert_eq!(bp.pnt1(), (0.0, 0.0));
        assert_eq!(bp.pnt2(), (0.0, 0.0));
    }
    #[test]
    fn test_with_points() {
        let bp = HlrbrépBiPnt2D::with_points(1.0, 2.0, 3.0, 4.0);
        assert_eq!(bp.pnt1(), (1.0, 2.0));
        assert_eq!(bp.pnt2(), (3.0, 4.0));
    }
}
