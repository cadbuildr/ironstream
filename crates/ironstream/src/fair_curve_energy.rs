// FILE: fair_curve_energy.rs
// occt: FairCurve_Energy

/// Energy functional for fair curve fairing
#[derive(Clone, Debug)]
pub struct FairCurveEnergy {
    value: f64,
    is_minimized: bool,
}

impl FairCurveEnergy {
    pub fn new() -> Self {
        FairCurveEnergy { value: 0.0, is_minimized: false }
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_minimized(&mut self, minimized: bool) {
        self.is_minimized = minimized;
    }

    pub fn is_minimized(&self) -> bool {
        self.is_minimized
    }
}

impl Default for FairCurveEnergy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let energy = FairCurveEnergy::new();
        assert_eq!(energy.value(), 0.0);
        assert!(!energy.is_minimized());
    }

    #[test]
    fn test_set_value() {
        let mut energy = FairCurveEnergy::new();
        energy.set_value(15.0);
        assert_eq!(energy.value(), 15.0);
    }

    #[test]
    fn test_set_minimized() {
        let mut energy = FairCurveEnergy::new();
        energy.set_minimized(true);
        assert!(energy.is_minimized());
    }
}
