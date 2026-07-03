// FILE: nl_plate_hpg0_g3_constraint.rs
// occt: NLPlate_HPG0G3Constraint

/// G0-G3 constraint for Non-Linear Plate surface
#[derive(Clone, Debug)]
pub struct HPG0G3Constraint {
    u: f64,
    v: f64,
    weight: f64,
}

impl HPG0G3Constraint {
    /// Create new constraint
    pub fn new() -> Self {
        HPG0G3Constraint {
            u: 0.0,
            v: 0.0,
            weight: 1.0,
        }
    }

    /// Create with parameters
    pub fn with_params(u: f64, v: f64, weight: f64) -> Self {
        HPG0G3Constraint { u, v, weight }
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

impl Default for HPG0G3Constraint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = HPG0G3Constraint::new();
        assert_eq!(c.weight(), 1.0);
    }

    #[test]
    fn test_with_params() {
        let c = HPG0G3Constraint::with_params(0.2, 0.3, 0.7);
        assert_eq!(c.u(), 0.2);
    }
}
