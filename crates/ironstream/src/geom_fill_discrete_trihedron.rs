// FILE: geom_fill_discrete_trihedron.rs
// occt: GeomFill_DiscreteTrihedron

/// Discrete trihedron for sweeping
pub struct DiscreteTrihedron;

impl DiscreteTrihedron {
    pub fn new() -> Self {
        DiscreteTrihedron
    }
}

impl Default for DiscreteTrihedron {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _tri = DiscreteTrihedron::new();
    }

    #[test]
    fn test_default() {
        let _tri = DiscreteTrihedron::default();
    }
}
