// FILE: draw_dim.rs
// occt: DrawDim

/// DrawDim implementation
#[derive(Clone)]
pub struct DrawDim;

impl DrawDim {
    /// Creates a new instance
    pub fn new() -> Self {
        DrawDim
    }

    /// Gets operation name
    pub fn operation_name(&self) -> &'static str {
        "DrawDim"
    }

    /// Executes operation
    pub fn execute(&self) -> Result<(), String> {
        Err("DrawDim execution not yet implemented".to_string())
    }
}

impl Default for DrawDim {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let obj = DrawDim::new();
        assert_eq!(obj.operation_name(), "DrawDim");
    }

    #[test]
    fn test_execute_stub() {
        let obj = DrawDim::new();
        assert!(obj.execute().is_err());
    }
}
