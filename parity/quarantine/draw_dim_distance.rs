// FILE: draw_dim_distance.rs
// occt: DrawDim_Distance

/// DrawDim_Distance implementation
#[derive(Clone)]
pub struct DrawDim_Distance;

impl DrawDim_Distance {
    /// Creates a new instance
    pub fn new() -> Self {
        DrawDim_Distance
    }

    /// Gets operation name
    pub fn operation_name(&self) -> &'static str {
        "DrawDimDistance"
    }

    /// Executes operation
    pub fn execute(&self) -> Result<(), String> {
        Err("DrawDimDistance execution not yet implemented".to_string())
    }
}

impl Default for DrawDim_Distance {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let obj = DrawDim_Distance::new();
        assert_eq!(obj.operation_name(), "DrawDimDistance");
    }

    #[test]
    fn test_execute_stub() {
        let obj = DrawDim_Distance::new();
        assert!(obj.execute().is_err());
    }
}
