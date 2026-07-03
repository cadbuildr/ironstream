// FILE: nl_plate_hpg0_g1_constraint.rs
// occt: NLPlate_HPG0G1Constraint

/// G0-G1 constraint for Non-Linear Plate surface
#[derive(Clone, Debug)]
pub struct HPG0G1Constraint {
    u: f64,
    v: f64,
    weight: f64,
}

impl HPG0G1Constraint {
    /// Create new constraint
    pub fn new() -> Self {
        HPG0G1Constraint {
            u: 0.0,
            v: 0.0,
            weight: 1.0,
        }
    }

    /// Create with parameters
    pub fn with_params(u: f64, v: f64, weight: f64) -> Self {
        HPG0G1Constraint { u, v, weight }
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

impl Default for HPG0G1Constraint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = HPG0G1Constraint::new();
        assert_eq!(c.u(), 0.0);
        assert_eq!(c.weight(), 1.0);
    }

    #[test]
    fn test_with_params() {
        let c = HPG0G1Constraint::with_params(0.5, 0.6, 0.8);
        assert_eq!(c.u(), 0.5);
        assert_eq!(c.v(), 0.6);
        assert_eq!(c.weight(), 0.8);
    }
}
