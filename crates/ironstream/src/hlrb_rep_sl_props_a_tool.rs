// FILE: hlrb_rep_sl_props_a_tool.rs
// occt: HLRBRep_SLPropsATool

pub struct HlrbrépSLPropsATool;

impl HlrbrépSLPropsATool {
    pub fn value(_u: f64, _v: f64) -> [f64; 3] { [0.0, 0.0, 0.0] }
    pub fn d1(_u: f64, _v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
    }
    pub fn normal(_u: f64, _v: f64) -> [f64; 3] { [0.0, 0.0, 1.0] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_value() {
        assert_eq!(HlrbrépSLPropsATool::value(0.5, 0.5), [0.0, 0.0, 0.0]);
    }
}
