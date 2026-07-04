// FILE: iges_geom_tool_point.rs
// occt: IGESGeom_ToolPoint

pub struct ToolPoint;

impl ToolPoint {
    pub fn new() -> Self {
        ToolPoint
    }
}

impl Default for ToolPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolPoint::new();
    }
}
