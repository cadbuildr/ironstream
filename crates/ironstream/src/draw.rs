// FILE: draw.rs
// occt: Draw

/// Draw implementation
#[derive(Clone)]
pub struct Draw;

impl Draw {
    /// Creates a new instance
    pub fn new() -> Self {
        Draw
    }

    /// Gets operation name
    pub fn operation_name(&self) -> &'static str {
        "Draw"
    }

    /// Executes operation
    pub fn execute(&self) -> Result<(), String> {
        Err("Draw execution not yet implemented".to_string())
    }
}

impl Default for Draw {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let obj = Draw::new();
        assert_eq!(obj.operation_name(), "Draw");
    }

    #[test]
    fn test_execute_stub() {
        let obj = Draw::new();
        assert!(obj.execute().is_err());
    }
}
