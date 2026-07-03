// FILE: nl_plate_hpg2_constraint.rs
// occt: NLPlate_HPG2Constraint

/// G2 constraint for Non-Linear Plate surface
#[derive(Clone, Debug)]
pub struct HPG2Constraint {
    u: f64,
    v: f64,
    weight: f64,
}

impl HPG2Constraint {
    /// Create new constraint
    pub fn new() -> Self {
        HPG2Constraint {
            u: 0.0,
            v: 0.0,
            weight: 1.0,
        }
    }

    /// Create with parameters
    pub fn with_params(u: f64, v: f64, weight: f64) -> Self {
        HPG2Constraint { u, v, weight }
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
}

impl Default for HPG2Constraint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = HPG2Constraint::new();
        assert_eq!(c.weight(), 1.0);
    }

    #[test]
    fn test_with_params() {
        let c = HPG2Constraint::with_params(0.25, 0.35, 0.85);
        assert_eq!(c.v(), 0.35);
    }
}
