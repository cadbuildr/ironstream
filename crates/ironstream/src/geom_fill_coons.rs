// FILE: geom_fill_coons.rs
// occt: GeomFill_Coons

/// Coons surface filler
pub struct Coons;

impl Coons {
    pub fn new() -> Self {
        Coons
    }
}

impl Default for Coons {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _coons = Coons::new();
    }

    #[test]
    fn test_default() {
        let _coons = Coons::default();
    }
}
