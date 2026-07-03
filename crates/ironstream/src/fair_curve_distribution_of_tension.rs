// FILE: fair_curve_distribution_of_tension.rs
// occt: FairCurve_DistributionOfTension

/// Tension distribution for fair curve fairing
#[derive(Clone, Debug)]
pub struct FairCurveDistributionOfTension {
    total_tension: f64,
}

impl FairCurveDistributionOfTension {
    pub fn new() -> Self {
        FairCurveDistributionOfTension { total_tension: 0.0 }
    }

    pub fn set_tension(&mut self, tension: f64) {
        self.total_tension = tension;
    }

    pub fn tension(&self) -> f64 {
        self.total_tension
    }
}

impl Default for FairCurveDistributionOfTension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let dist = FairCurveDistributionOfTension::new();
        assert_eq!(dist.tension(), 0.0);
    }

    #[test]
    fn test_set_tension() {
        let mut dist = FairCurveDistributionOfTension::new();
        dist.set_tension(2.0);
        assert_eq!(dist.tension(), 2.0);
    }
}
