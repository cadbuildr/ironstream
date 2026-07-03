// FILE: geom_fill_degenerated_bound.rs
// occt: GeomFill_DegeneratedBound

/// Degenerated boundary for surface filling
pub struct DegeneratedBound;

impl DegeneratedBound {
    pub fn new() -> Self {
        DegeneratedBound
    }
}

impl Default for DegeneratedBound {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _bound = DegeneratedBound::new();
    }

    #[test]
    fn test_default() {
        let _bound = DegeneratedBound::default();
    }
}
