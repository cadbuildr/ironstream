// FILE: geom_fill_quasi_angular_convertor.rs
// occt: GeomFill_QuasiAngularConvertor

/// Converts curves using quasi-angular parametrization
pub struct QuasiAngularConvertor;

impl QuasiAngularConvertor {
    pub fn new() -> Self {
        QuasiAngularConvertor
    }
}

impl Default for QuasiAngularConvertor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _conv = QuasiAngularConvertor::new();
    }

    #[test]
    fn test_default() {
        let _conv = QuasiAngularConvertor::default();
    }
}
