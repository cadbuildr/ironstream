// FILE: draw_number.rs
// occt: Draw_Number

//! Stores numeric values in Draw variables.

/// Represents a drawable number for storing numeric values
pub struct DrawNumber {
    value: f64,
}

impl DrawNumber {
    /// Create a new Draw number
    pub fn new(value: f64) -> Self {
        DrawNumber { value }
    }

    /// Get the numeric value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Set the numeric value
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    /// Check if drawable is displayable (always false for numbers)
    pub fn is_displayable(&self) -> bool {
        false
    }

    /// Dump the number as a string
    pub fn dump(&self) -> String {
        format!("{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_number_creation() {
        let num = DrawNumber::new(42.0);
        assert_eq!(num.value(), 42.0);
    }

    #[test]
    fn test_draw_number_set_value() {
        let mut num = DrawNumber::new(0.0);
        num.set_value(3.14159);
        assert_eq!(num.value(), 3.14159);
    }

    #[test]
    fn test_draw_number_not_displayable() {
        let num = DrawNumber::new(100.0);
        assert!(!num.is_displayable());
    }

    #[test]
    fn test_draw_number_dump() {
        let num = DrawNumber::new(123.456);
        let dumped = num.dump();
        assert!(dumped.contains("123"));
    }
}
