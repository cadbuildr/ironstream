// FILE: iges_geom_tool_direction.rs
// occt: IGESGeom_ToolDirection

pub struct ToolDirection;

impl ToolDirection {
    pub fn new() -> Self {
        ToolDirection
    }
}

impl Default for ToolDirection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolDirection::new();
    }
}
