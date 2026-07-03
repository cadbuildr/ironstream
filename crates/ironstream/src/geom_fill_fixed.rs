// FILE: geom_fill_fixed.rs
// occt: GeomFill_Fixed

/// Fixed trihedron for sweeping
pub struct Fixed;

impl Fixed {
    pub fn new() -> Self {
        Fixed
    }
}

impl Default for Fixed {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _fixed = Fixed::new();
    }

    #[test]
    fn test_default() {
        let _fixed = Fixed::default();
    }
}
