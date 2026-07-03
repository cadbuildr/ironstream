// FILE: geom_fill_polynomial_convertor.rs
// occt: GeomFill_PolynomialConvertor

/// Converts polynomial curves
pub struct PolynomialConvertor;

impl PolynomialConvertor {
    pub fn new() -> Self {
        PolynomialConvertor
    }
}

impl Default for PolynomialConvertor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _conv = PolynomialConvertor::new();
    }

    #[test]
    fn test_default() {
        let _conv = PolynomialConvertor::default();
    }
}
