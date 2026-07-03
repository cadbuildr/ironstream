// FILE: geom_fill_curve_and_trihedron.rs
// occt: GeomFill_CurveAndTrihedron

/// Base class for curve with trihedron generation
pub struct CurveAndTrihedron;

impl CurveAndTrihedron {
    pub fn new() -> Self {
        CurveAndTrihedron
    }
}

impl Default for CurveAndTrihedron {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _trh = CurveAndTrihedron::new();
    }

    #[test]
    fn test_default() {
        let _trh = CurveAndTrihedron::default();
    }
}
