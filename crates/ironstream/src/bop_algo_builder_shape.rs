// FILE: bop_algo_builder_shape.rs
// occt: BOPAlgo_BuilderShape

/// Base class for Boolean operation builders
pub struct BopAlgoBuilderShape {
    error_status: i32,
}

impl BopAlgoBuilderShape {
    pub fn new() -> Self {
        BopAlgoBuilderShape { error_status: 0 }
    }

    pub fn set_error(&mut self, status: i32) {
        self.error_status = status;
    }

    pub fn error_status(&self) -> i32 {
        self.error_status
    }
}

impl Default for BopAlgoBuilderShape {
    fn default() -> Self {
        BopAlgoBuilderShape::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let builder = BopAlgoBuilderShape::new();
        assert_eq!(builder.error_status(), 0);
    }

    #[test]
    fn test_error() {
        let mut builder = BopAlgoBuilderShape::new();
        builder.set_error(1);
        assert_eq!(builder.error_status(), 1);
    }
}
