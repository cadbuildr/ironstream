// FILE: geom_fill_curved.rs
// occt: GeomFill_Curved

/// Curved surface filling
pub struct Curved;

impl Curved {
    pub fn new() -> Self {
        Curved
    }
}

impl Default for Curved {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _curved = Curved::new();
    }

    #[test]
    fn test_default() {
        let _curved = Curved::default();
    }
}
