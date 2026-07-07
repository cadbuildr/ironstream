// FILE: iges_geom_tool_trimmed_surface.rs
// occt: IGESGeom_ToolTrimmedSurface

pub struct ToolTrimmedSurface;

impl ToolTrimmedSurface {
    pub fn new() -> Self {
        ToolTrimmedSurface
    }
}

impl Default for ToolTrimmedSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolTrimmedSurface::new();
    }
}
