// FILE: prs3d_tool_quadric.rs
// occt: Prs3d_ToolQuadric

/// Stub for Prs3d_ToolQuadric from OCCT.
#[derive(Clone, Debug)]
pub struct Prs3d_ToolQuadric {}

impl Prs3d_ToolQuadric {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Prs3d_ToolQuadric {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_tool_quadric_creation() {
        let _obj = Prs3d_ToolQuadric::new();
        let _def = Prs3d_ToolQuadric::default();
    }
}
