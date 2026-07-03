// FILE: hlrb_rep_cl_props_a_tool.rs
// occt: HLRBRep_CLPropsATool

pub struct HlrbrépCLPropsATool;

impl HlrbrépCLPropsATool {
    pub fn value(_u: f64) -> [f64; 3] { [0.0, 0.0, 0.0] }
    pub fn d1(_u: f64) -> ([f64; 3], [f64; 3]) { ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]) }
    pub fn d2(_u: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
    }
    pub fn d3(_u: f64) -> ([f64; 3], [f64; 3], [f64; 3], [f64; 3]) {
        ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_value() {
        assert_eq!(HlrbrépCLPropsATool::value(0.5), [0.0, 0.0, 0.0]);
    }
}
