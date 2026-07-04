// FILE: iges_geom_tool_plane.rs
// occt: IGESGeom_ToolPlane

pub struct ToolPlane;

impl ToolPlane {
    pub fn new() -> Self {
        ToolPlane
    }
}

impl Default for ToolPlane {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolPlane::new();
    }
}
