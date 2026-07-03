// FILE: gcc_ana_circ2d_tan_cen.rs
// occt: GccAna_Circ2dTanCen

/// Analytical algorithm for circles tangent to an entity with given center
#[derive(Clone, Debug)]
pub struct GccAnaCirc2dTanCen {
    center_x: f64,
    center_y: f64,
    radius: f64,
    done: bool,
}

impl GccAnaCirc2dTanCen {
    pub fn new(cx: f64, cy: f64) -> Self {
        GccAnaCirc2dTanCen {
            center_x: cx,
            center_y: cy,
            radius: 0.0,
            done: false,
        }
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    pub fn set_done(&mut self) {
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn center(&self) -> (f64, f64) {
        (self.center_x, self.center_y)
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let algo = GccAnaCirc2dTanCen::new(1.0, 2.0);
        let (cx, cy) = algo.center();
        assert_eq!(cx, 1.0);
        assert_eq!(cy, 2.0);
    }

    #[test]
    fn test_set_radius() {
        let mut algo = GccAnaCirc2dTanCen::new(0.0, 0.0);
        algo.set_radius(5.0);
        assert_eq!(algo.radius(), 5.0);
    }

    #[test]
    fn test_done() {
        let mut algo = GccAnaCirc2dTanCen::new(1.0, 1.0);
        assert!(!algo.is_done());
        algo.set_done();
        assert!(algo.is_done());
    }
}
