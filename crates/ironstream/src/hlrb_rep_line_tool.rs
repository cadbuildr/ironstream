// FILE: hlrb_rep_line_tool.rs
// occt: HLRBRep_LineTool

pub struct HlrbrépLineTool;

impl HlrbrépLineTool {
    pub fn first_parameter() -> f64 { 0.0 }
    pub fn last_parameter() -> f64 { 1.0 }
    pub fn is_closed() -> bool { false }
    pub fn is_periodic() -> bool { false }
    pub fn period() -> f64 { 0.0 }
    pub fn resolution(_r3d: f64) -> f64 { 0.01 }
    pub fn continuity() -> usize { 3 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parameters() {
        assert_eq!(HlrbrépLineTool::first_parameter(), 0.0);
    }
}
