// FILE: iges_geom_tool_offset_surface.rs
// occt: IGESGeom_ToolOffsetSurface

pub struct ToolOffsetSurface;

impl ToolOffsetSurface {
    pub fn new() -> Self {
        ToolOffsetSurface
    }
}

impl Default for ToolOffsetSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolOffsetSurface::new();
    }
}
