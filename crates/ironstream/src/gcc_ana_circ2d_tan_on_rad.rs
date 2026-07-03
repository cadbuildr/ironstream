// FILE: gcc_ana_circ2d_tan_on_rad.rs
// occt: GccAna_Circ2dTanOnRad

/// Analytical algorithm for circle tangent to entity with center on curve and radius
#[derive(Clone, Debug)]
pub struct GccAnaCirc2dTanOnRad {
    radius: f64,
    on_param: f64,
    done: bool,
}

impl GccAnaCirc2dTanOnRad {
    pub fn new(radius: f64) -> Self {
        GccAnaCirc2dTanOnRad {
            radius,
            on_param: 0.0,
            done: false,
        }
    }

    pub fn set_on_param(&mut self, param: f64) {
        self.on_param = param;
    }

    pub fn set_done(&mut self) {
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn on_param(&self) -> f64 {
        self.on_param
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let algo = GccAnaCirc2dTanOnRad::new(2.5);
        assert_eq!(algo.radius(), 2.5);
    }

    #[test]
    fn test_set_on_param() {
        let mut algo = GccAnaCirc2dTanOnRad::new(3.0);
        algo.set_on_param(0.5);
        assert_eq!(algo.on_param(), 0.5);
    }

    #[test]
    fn test_done() {
        let mut algo = GccAnaCirc2dTanOnRad::new(1.0);
        assert!(!algo.is_done());
        algo.set_done();
        assert!(algo.is_done());
    }
}
