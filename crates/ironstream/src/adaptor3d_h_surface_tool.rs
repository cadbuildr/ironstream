// FILE: adaptor3d_h_surface_tool.rs
// occt: Adaptor3d_HSurfaceTool

pub struct Adaptor3dHSurfaceTool;

impl Adaptor3dHSurfaceTool {
    pub fn new() -> Self {
        Adaptor3dHSurfaceTool
    }

    pub fn first_u(&self) -> f64 {
        0.0
    }

    pub fn last_u(&self) -> f64 {
        1.0
    }

    pub fn first_v(&self) -> f64 {
        0.0
    }

    pub fn last_v(&self) -> f64 {
        1.0
    }
}

impl Default for Adaptor3dHSurfaceTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_surface_tool_new() {
        let tool = Adaptor3dHSurfaceTool::new();
        assert_eq!(tool.first_u(), 0.0);
        assert_eq!(tool.last_u(), 1.0);
    }

    #[test]
    fn test_h_surface_tool_v_range() {
        let tool = Adaptor3dHSurfaceTool::new();
        assert_eq!(tool.first_v(), 0.0);
        assert_eq!(tool.last_v(), 1.0);
    }
}
