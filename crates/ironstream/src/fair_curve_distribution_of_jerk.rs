// FILE: fair_curve_distribution_of_jerk.rs
// occt: FairCurve_DistributionOfJerk

/// Jerk distribution for fair curve fairing
#[derive(Clone, Debug)]
pub struct FairCurveDistributionOfJerk {
    total_jerk: f64,
}

impl FairCurveDistributionOfJerk {
    pub fn new() -> Self {
        FairCurveDistributionOfJerk { total_jerk: 0.0 }
    }

    pub fn set_jerk(&mut self, jerk: f64) {
        self.total_jerk = jerk;
    }

    pub fn jerk(&self) -> f64 {
        self.total_jerk
    }
}

impl Default for FairCurveDistributionOfJerk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let dist = FairCurveDistributionOfJerk::new();
        assert_eq!(dist.jerk(), 0.0);
    }

    #[test]
    fn test_set_jerk() {
        let mut dist = FairCurveDistributionOfJerk::new();
        dist.set_jerk(5.0);
        assert_eq!(dist.jerk(), 5.0);
    }
}
