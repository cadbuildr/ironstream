// FILE: iges_geom_tool_ruled_surface.rs
// occt: IGESGeom_ToolRuledSurface

pub struct ToolRuledSurface;

impl ToolRuledSurface {
    pub fn new() -> Self {
        ToolRuledSurface
    }
}

impl Default for ToolRuledSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolRuledSurface::new();
    }
}
