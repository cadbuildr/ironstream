// FILE: bopds_curve.rs
// occt: BOPDS_Curve

/// The class BOPDS_Curve is to store information about intersection curve
#[derive(Clone, Debug)]
pub struct BodsCurve {
    index: i32,
    tangential: bool,
}

impl BodsCurve {
    pub fn new() -> Self {
        BodsCurve {
            index: 0,
            tangential: false,
        }
    }

    pub fn with_index(idx: i32) -> Self {
        BodsCurve {
            index: idx,
            tangential: false,
        }
    }

    pub fn set_index(&mut self, idx: i32) {
        self.index = idx;
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn set_tangential(&mut self, val: bool) {
        self.tangential = val;
    }

    pub fn tangential(&self) -> bool {
        self.tangential
    }
}

impl Default for BodsCurve {
    fn default() -> Self {
        BodsCurve::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_new() {
        let curve = BodsCurve::new();
        assert_eq!(curve.index(), 0);
        assert_eq!(curve.tangential(), false);
    }

    #[test]
    fn test_curve_with_index() {
        let curve = BodsCurve::with_index(5);
        assert_eq!(curve.index(), 5);
    }

    #[test]
    fn test_set_index() {
        let mut curve = BodsCurve::new();
        curve.set_index(3);
        assert_eq!(curve.index(), 3);
    }

    #[test]
    fn test_set_tangential() {
        let mut curve = BodsCurve::new();
        curve.set_tangential(true);
        assert_eq!(curve.tangential(), true);
    }
}
