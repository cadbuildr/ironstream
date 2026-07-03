// FILE: geom_fill_darboux.rs
// occt: GeomFill_Darboux

/// Darboux trihedron for sweeping
pub struct Darboux;

impl Darboux {
    pub fn new() -> Self {
        Darboux
    }
}

impl Default for Darboux {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _darboux = Darboux::new();
    }

    #[test]
    fn test_default() {
        let _darboux = Darboux::default();
    }
}
