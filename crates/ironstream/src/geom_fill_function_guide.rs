// FILE: geom_fill_function_guide.rs
// occt: GeomFill_FunctionGuide

/// Function for guided surface generation
pub struct FunctionGuide;

impl FunctionGuide {
    pub fn new() -> Self {
        FunctionGuide
    }
}

impl Default for FunctionGuide {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _func = FunctionGuide::new();
    }

    #[test]
    fn test_default() {
        let _func = FunctionGuide::default();
    }
}
