// FILE: nl_plate_hpg0_g2_constraint.rs
// occt: NLPlate_HPG0G2Constraint

/// G0-G2 constraint for Non-Linear Plate surface
#[derive(Clone, Debug)]
pub struct HPG0G2Constraint {
    u: f64,
    v: f64,
    weight: f64,
}

impl HPG0G2Constraint {
    /// Create new constraint
    pub fn new() -> Self {
        HPG0G2Constraint {
            u: 0.0,
            v: 0.0,
            weight: 1.0,
        }
    }

    /// Create with parameters
    pub fn with_params(u: f64, v: f64, weight: f64) -> Self {
        HPG0G2Constraint { u, v, weight }
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

impl Default for HPG0G2Constraint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = HPG0G2Constraint::new();
        assert_eq!(c.weight(), 1.0);
    }

    #[test]
    fn test_with_params() {
        let c = HPG0G2Constraint::with_params(0.3, 0.4, 0.9);
        assert_eq!(c.u(), 0.3);
        assert_eq!(c.v(), 0.4);
    }
}
