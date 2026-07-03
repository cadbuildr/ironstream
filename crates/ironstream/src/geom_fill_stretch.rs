// FILE: geom_fill_stretch.rs
// occt: GeomFill_Stretch

/// Stretch surface filler
pub struct Stretch;

impl Stretch {
    pub fn new() -> Self {
        Stretch
    }
}

impl Default for Stretch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _stretch = Stretch::new();
    }

    #[test]
    fn test_default() {
        let _stretch = Stretch::default();
    }
}
