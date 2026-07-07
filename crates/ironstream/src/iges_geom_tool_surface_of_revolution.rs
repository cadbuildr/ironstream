// FILE: iges_geom_tool_surface_of_revolution.rs
// occt: IGESGeom_ToolSurfaceOfRevolution

pub struct ToolSurfaceOfRevolution;

impl ToolSurfaceOfRevolution {
    pub fn new() -> Self {
        ToolSurfaceOfRevolution
    }
}

impl Default for ToolSurfaceOfRevolution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolSurfaceOfRevolution::new();
    }
}
