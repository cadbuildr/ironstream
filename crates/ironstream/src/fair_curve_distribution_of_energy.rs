// FILE: fair_curve_distribution_of_energy.rs
// occt: FairCurve_DistributionOfEnergy

/// Energy distribution for fair curve fairing
#[derive(Clone, Debug)]
pub struct FairCurveDistributionOfEnergy {
    total_energy: f64,
}

impl FairCurveDistributionOfEnergy {
    pub fn new() -> Self {
        FairCurveDistributionOfEnergy { total_energy: 0.0 }
    }

    pub fn set_energy(&mut self, energy: f64) {
        self.total_energy = energy;
    }

    pub fn energy(&self) -> f64 {
        self.total_energy
    }

    pub fn is_valid(&self) -> bool {
        self.total_energy >= 0.0
    }
}

impl Default for FairCurveDistributionOfEnergy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let dist = FairCurveDistributionOfEnergy::new();
        assert_eq!(dist.energy(), 0.0);
    }

    #[test]
    fn test_set_energy() {
        let mut dist = FairCurveDistributionOfEnergy::new();
        dist.set_energy(10.0);
        assert_eq!(dist.energy(), 10.0);
    }

    #[test]
    fn test_is_valid() {
        let mut dist = FairCurveDistributionOfEnergy::new();
        assert!(dist.is_valid());
        dist.set_energy(-1.0);
        assert!(!dist.is_valid());
    }
}
