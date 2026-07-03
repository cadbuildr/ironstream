// FILE: hlrb_rep_sl_props.rs
// occt: HLRBRep_SLProps

pub struct HlrbrépSLProps {
    u: f64,
    v: f64,
    normal: [f64; 3],
    curvature1: f64,
    curvature2: f64,
}

impl HlrbrépSLProps {
    pub fn new() -> Self {
        HlrbrépSLProps {
            u: 0.0,
            v: 0.0,
            normal: [0.0, 0.0, 1.0],
            curvature1: 0.0,
            curvature2: 0.0,
        }
    }

    pub fn set_parameters(&mut self, u: f64, v: f64) {
        self.u = u;
        self.v = v;
    }

    pub fn u(&self) -> f64 { self.u }
    pub fn v(&self) -> f64 { self.v }
    pub fn normal(&self) -> [f64; 3] { self.normal }
}

impl Default for HlrbrépSLProps {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let sl = HlrbrépSLProps::new();
        assert_eq!(sl.u(), 0.0);
    }
}
