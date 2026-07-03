// FILE: hlrb_rep_curve.rs
// occt: HLRBRep_Curve

pub struct HlrbrépCurve {
    param1: f64,
    param2: f64,
    flags: i32,
}

impl HlrbrépCurve {
    pub fn new() -> Self {
        HlrbrépCurve {
            param1: 0.0,
            param2: 1.0,
            flags: 0,
        }
    }

    pub fn set_range(&mut self, p1: f64, p2: f64) {
        self.param1 = p1;
        self.param2 = p2;
    }

    pub fn parameter1(&self) -> f64 { self.param1 }
    pub fn parameter2(&self) -> f64 { self.param2 }
    pub fn set_flags(&mut self, f: i32) { self.flags = f; }
    pub fn flags(&self) -> i32 { self.flags }
}

impl Default for HlrbrépCurve {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let c = HlrbrépCurve::new();
        assert_eq!(c.parameter1(), 0.0);
        assert_eq!(c.parameter2(), 1.0);
    }
}
