// FILE: iges_geom_tool_conic_arc.rs
// occt: IGESGeom_ToolConicArc

pub struct ToolConicArc;

impl ToolConicArc {
    pub fn new() -> Self {
        ToolConicArc
    }
}

impl Default for ToolConicArc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolConicArc::new();
    }
}
