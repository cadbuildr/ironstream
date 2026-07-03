// FILE: fair_curve_distribution_of_sagging.rs
// occt: FairCurve_DistributionOfSagging

/// Sagging distribution for fair curve fairing
#[derive(Clone, Debug)]
pub struct FairCurveDistributionOfSagging {
    total_sagging: f64,
}

impl FairCurveDistributionOfSagging {
    pub fn new() -> Self {
        FairCurveDistributionOfSagging { total_sagging: 0.0 }
    }

    pub fn set_sagging(&mut self, sagging: f64) {
        self.total_sagging = sagging;
    }

    pub fn sagging(&self) -> f64 {
        self.total_sagging
    }
}

impl Default for FairCurveDistributionOfSagging {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let dist = FairCurveDistributionOfSagging::new();
        assert_eq!(dist.sagging(), 0.0);
    }

    #[test]
    fn test_set_sagging() {
        let mut dist = FairCurveDistributionOfSagging::new();
        dist.set_sagging(3.0);
        assert_eq!(dist.sagging(), 3.0);
    }
}
