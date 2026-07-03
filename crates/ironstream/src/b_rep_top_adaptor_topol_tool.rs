// FILE: b_rep_top_adaptor_topol_tool.rs
// occt: BRepTopAdaptor_TopolTool

/// Topological tool for BRep faces in surface intersection.
#[derive(Debug, Clone)]
pub struct BRepTopAdaptorTopolTool {
    u0: f64,
    v0: f64,
    du: f64,
    dv: f64,
}

impl BRepTopAdaptorTopolTool {
    pub fn new() -> Self {
        BRepTopAdaptorTopolTool {
            u0: 0.0,
            v0: 0.0,
            du: 1.0,
            dv: 1.0,
        }
    }

    pub fn u0(&self) -> f64 {
        self.u0
    }

    pub fn v0(&self) -> f64 {
        self.v0
    }
}

impl Default for BRepTopAdaptorTopolTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topol_tool() {
        let tool = BRepTopAdaptorTopolTool::new();
        assert_eq!(tool.u0(), 0.0);
        assert_eq!(tool.v0(), 0.0);
    }
}
