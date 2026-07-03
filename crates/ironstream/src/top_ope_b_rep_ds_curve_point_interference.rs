// FILE: top_ope_b_rep_ds_curve_point_interference.rs
// occt: TopOpeBRepDS_CurvePointInterference

use crate::top_ope_b_rep_ds_interference::Interference;
use crate::top_ope_b_rep_ds_transition::Transition;
use crate::top_ope_b_rep_ds_kind::Kind;

/// Curve-point interference (extends Interference with a parameter)
#[derive(Debug, Clone)]
pub struct CurvePointInterference {
    /// Base interference
    base: Interference,
    /// Parameter on the curve
    param: f64,
}

impl CurvePointInterference {
    /// Create a curve-point interference
    pub fn new(
        transition: Transition,
        support_type: Kind,
        support: i32,
        geometry_type: Kind,
        geometry: i32,
        param: f64,
    ) -> Self {
        let base = Interference::with_data(transition, support_type, support, geometry_type, geometry);
        CurvePointInterference { base, param }
    }

    /// Get the parameter
    pub fn parameter(&self) -> f64 {
        self.param
    }

    /// Set the parameter
    pub fn set_parameter(&mut self, p: f64) {
        self.param = p;
    }

    /// Get base interference
    pub fn base(&self) -> &Interference {
        &self.base
    }

    /// Get mutable base interference
    pub fn base_mut(&mut self) -> &mut Interference {
        &mut self.base
    }

    // Delegate methods
    pub fn support_type(&self) -> Kind {
        self.base.support_type()
    }

    pub fn support(&self) -> i32 {
        self.base.support()
    }

    pub fn geometry_type(&self) -> Kind {
        self.base.geometry_type()
    }

    pub fn geometry(&self) -> i32 {
        self.base.geometry()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_point_interference_new() {
        let t = Transition::new();
        let cpi = CurvePointInterference::new(
            t,
            Kind::Curve,
            5,
            Kind::Point,
            10,
            2.5,
        );
        assert_eq!(cpi.support_type(), Kind::Curve);
        assert_eq!(cpi.support(), 5);
        assert_eq!(cpi.geometry_type(), Kind::Point);
        assert_eq!(cpi.geometry(), 10);
        assert_eq!(cpi.parameter(), 2.5);
    }

    #[test]
    fn test_curve_point_interference_parameter() {
        let t = Transition::new();
        let mut cpi = CurvePointInterference::new(t, Kind::Curve, 1, Kind::Point, 2, 0.5);
        assert_eq!(cpi.parameter(), 0.5);
        cpi.set_parameter(3.7);
        assert_eq!(cpi.parameter(), 3.7);
    }
}
