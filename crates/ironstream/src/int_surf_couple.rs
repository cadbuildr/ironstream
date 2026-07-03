// FILE: int_surf_couple.rs
// occt: IntSurf_Couple

/// Couple of parameter values on two surfaces
#[derive(Clone, Debug, Copy)]
pub struct Couple {
    u1: f64,
    v1: f64,
    u2: f64,
    v2: f64,
}

impl Couple {
    /// Create a new couple
    pub fn new() -> Self {
        Couple {
            u1: 0.0,
            v1: 0.0,
            u2: 0.0,
            v2: 0.0,
        }
    }

    /// Create with values
    pub fn with_params(u1: f64, v1: f64, u2: f64, v2: f64) -> Self {
        Couple { u1, v1, u2, v2 }
    }

    pub fn u1(&self) -> f64 {
        self.u1
    }

    pub fn v1(&self) -> f64 {
        self.v1
    }

    pub fn u2(&self) -> f64 {
        self.u2
    }

    pub fn v2(&self) -> f64 {
        self.v2
    }

    pub fn set_values(&mut self, u1: f64, v1: f64, u2: f64, v2: f64) {
        self.u1 = u1;
        self.v1 = v1;
        self.u2 = u2;
        self.v2 = v2;
    }
}

impl Default for Couple {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Couple::new();
        assert_eq!(c.u1(), 0.0);
        assert_eq!(c.v1(), 0.0);
    }

    #[test]
    fn test_with_params() {
        let c = Couple::with_params(0.5, 0.6, 0.7, 0.8);
        assert_eq!(c.u1(), 0.5);
        assert_eq!(c.v1(), 0.6);
        assert_eq!(c.u2(), 0.7);
        assert_eq!(c.v2(), 0.8);
    }
}
