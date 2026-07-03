// FILE: geom_fill_o.rs
// occt: GeomFill

/// Geometry fill package for surface and sweep operations
pub struct GeomFill;

impl GeomFill {
    pub fn new() -> Self {
        GeomFill
    }
}

impl Default for GeomFill {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _fill = GeomFill::new();
    }

    #[test]
    fn test_default() {
        let _fill = GeomFill::default();
    }
}
