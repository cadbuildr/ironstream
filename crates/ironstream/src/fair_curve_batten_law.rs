// FILE: fair_curve_batten_law.rs
// occt: FairCurve_BattenLaw

/// Batten law for fair curve fairing
#[derive(Clone, Debug)]
pub struct FairCurveBattenLaw {
    tension: f64,
}

impl FairCurveBattenLaw {
    pub fn new() -> Self {
        FairCurveBattenLaw { tension: 1.0 }
    }

    pub fn set_tension(&mut self, tension: f64) {
        self.tension = tension;
    }

    pub fn tension(&self) -> f64 {
        self.tension
    }

    pub fn evaluate(&self, t: f64) -> f64 {
        self.tension * t
    }
}

impl Default for FairCurveBattenLaw {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let law = FairCurveBattenLaw::new();
        assert_eq!(law.tension(), 1.0);
    }

    #[test]
    fn test_set_tension() {
        let mut law = FairCurveBattenLaw::new();
        law.set_tension(2.0);
        assert_eq!(law.tension(), 2.0);
    }

    #[test]
    fn test_evaluate() {
        let law = FairCurveBattenLaw::new();
        assert_eq!(law.evaluate(0.5), 0.5);
    }
}
