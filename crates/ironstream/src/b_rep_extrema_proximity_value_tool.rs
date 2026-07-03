// FILE: b_rep_extrema_proximity_value_tool.rs
// occt: BRepExtrema_ProximityValueTool

/// Tool for proximity value calculations
pub struct ProximityValueTool;

impl ProximityValueTool {
    pub fn new() -> Self {
        ProximityValueTool
    }
}

impl Default for ProximityValueTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proximity_value_tool_creation() {
        let _tool = ProximityValueTool::new();
    }
}
