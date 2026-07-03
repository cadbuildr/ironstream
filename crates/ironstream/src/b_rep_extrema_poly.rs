// FILE: b_rep_extrema_poly.rs
// occt: BRepExtrema_Poly

/// Polygon processing utilities
pub struct Poly;

impl Poly {
    pub fn new() -> Self {
        Poly
    }
}

impl Default for Poly {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_creation() {
        let _poly = Poly::new();
    }
}
