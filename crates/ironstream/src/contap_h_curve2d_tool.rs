// FILE: contap_h_curve2d_tool.rs
// occt: Contap_HCurve2dTool

pub struct ContapHCurve2dTool;

impl ContapHCurve2dTool {
    pub fn new() -> Self {
        ContapHCurve2dTool
    }
}

impl Default for ContapHCurve2dTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ContapHCurve2dTool;

    #[test]
    fn test_new() {
        let _ = ContapHCurve2dTool::new();
    }
}
