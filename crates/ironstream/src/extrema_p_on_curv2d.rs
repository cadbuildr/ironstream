// FILE: extrema_p_on_curv2d.rs
// occt: Extrema_POnCurv2d

/// Point on 2D curve definition.
pub struct ExtremaPOnCurv2d {
    param: f64,
    x: f64,
    y: f64,
}

impl ExtremaPOnCurv2d {
    pub fn new() -> Self {
        ExtremaPOnCurv2d {
            param: 0.0,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn set_values(&mut self, param: f64, x: f64, y: f64) {
        self.param = param;
        self.x = x;
        self.y = y;
    }

    pub fn parameter(&self) -> f64 {
        self.param
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = ExtremaPOnCurv2d::new();
        assert_eq!(p.parameter(), 0.0);
    }

    #[test]
    fn test_set_values() {
        let mut p = ExtremaPOnCurv2d::new();
        p.set_values(1.5, 2.0, 3.0);
        assert_eq!(p.parameter(), 1.5);
        assert_eq!(p.x(), 2.0);
        assert_eq!(p.y(), 3.0);
    }
}
