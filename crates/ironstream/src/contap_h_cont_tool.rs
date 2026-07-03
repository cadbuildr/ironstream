// FILE: contap_h_cont_tool.rs
// occt: Contap_HContTool

pub struct ContapHContTool;

impl ContapHContTool {
    pub fn new() -> Self {
        ContapHContTool
    }
}

impl Default for ContapHContTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ContapHContTool;

    #[test]
    fn test_new() {
        let _ = ContapHContTool::new();
    }
}
