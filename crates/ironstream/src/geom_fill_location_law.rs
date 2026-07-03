// FILE: geom_fill_location_law.rs
// occt: GeomFill_LocationLaw

/// Base class for location law in sweep surfaces
pub struct LocationLaw;

impl LocationLaw {
    pub fn new() -> Self {
        LocationLaw
    }
}

impl Default for LocationLaw {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _law = LocationLaw::new();
    }

    #[test]
    fn test_default() {
        let _law = LocationLaw::default();
    }
}
