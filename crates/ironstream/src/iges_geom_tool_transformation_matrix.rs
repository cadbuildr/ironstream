// FILE: iges_geom_tool_transformation_matrix.rs
// occt: IGESGeom_ToolTransformationMatrix

pub struct ToolTransformationMatrix;

impl ToolTransformationMatrix {
    pub fn new() -> Self {
        ToolTransformationMatrix
    }
}

impl Default for ToolTransformationMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolTransformationMatrix::new();
    }
}
