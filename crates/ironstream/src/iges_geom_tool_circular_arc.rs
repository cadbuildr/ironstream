// FILE: iges_geom_tool_circular_arc.rs
// occt: IGESGeom_ToolCircularArc

pub struct ToolCircularArc;

impl ToolCircularArc {
    pub fn new() -> Self {
        ToolCircularArc
    }
}

impl Default for ToolCircularArc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolCircularArc::new();
    }
}
