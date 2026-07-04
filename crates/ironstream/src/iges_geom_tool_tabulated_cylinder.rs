// FILE: iges_geom_tool_tabulated_cylinder.rs
// occt: IGESGeom_ToolTabulatedCylinder

pub struct ToolTabulatedCylinder;

impl ToolTabulatedCylinder {
    pub fn new() -> Self {
        ToolTabulatedCylinder
    }
}

impl Default for ToolTabulatedCylinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolTabulatedCylinder::new();
    }
}
