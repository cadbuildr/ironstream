// FILE: top_ope_b_rep_ds_surface_curve_interference.rs
// occt: TopOpeBRepDS_SurfaceCurveInterference

use crate::top_ope_b_rep_ds_interference::Interference;
use crate::top_ope_b_rep_ds_transition::Transition;
use crate::top_ope_b_rep_ds_kind::Kind;

/// Surface-curve interference (extends Interference with 2D parameters)
#[derive(Debug, Clone)]
pub struct SurfaceCurveInterference {
    /// Base interference
    base: Interference,
    /// Parameter on curve
    param_on_curve: f64,
    /// Parameter U on surface
    param_u: f64,
    /// Parameter V on surface
    param_v: f64,
}

impl SurfaceCurveInterference {
    /// Create a surface-curve interference
    pub fn new(
        transition: Transition,
        support_type: Kind,
        support: i32,
        geometry_type: Kind,
        geometry: i32,
        param_on_curve: f64,
        param_u: f64,
        param_v: f64,
    ) -> Self {
        let base = Interference::with_data(transition, support_type, support, geometry_type, geometry);
        SurfaceCurveInterference {
            base,
            param_on_curve,
            param_u,
            param_v,
        }
    }

    /// Get parameter on curve
    pub fn param_on_curve(&self) -> f64 {
        self.param_on_curve
    }

    /// Set parameter on curve
    pub fn set_param_on_curve(&mut self, p: f64) {
        self.param_on_curve = p;
    }

    /// Get U parameter
    pub fn param_u(&self) -> f64 {
        self.param_u
    }

    /// Set U parameter
    pub fn set_param_u(&mut self, u: f64) {
        self.param_u = u;
    }

    /// Get V parameter
    pub fn param_v(&self) -> f64 {
        self.param_v
    }

    /// Set V parameter
    pub fn set_param_v(&mut self, v: f64) {
        self.param_v = v;
    }

    /// Get base interference
    pub fn base(&self) -> &Interference {
        &self.base
    }

    /// Get mutable base interference
    pub fn base_mut(&mut self) -> &mut Interference {
        &mut self.base
    }

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
    fn test_surface_curve_interference_new() {
        let t = Transition::new();
        let sci = SurfaceCurveInterference::new(
            t,
            Kind::Surface,
            5,
            Kind::Curve,
            10,
            2.5,
            1.0,
            1.5,
        );
        assert_eq!(sci.support_type(), Kind::Surface);
        assert_eq!(sci.support(), 5);
        assert_eq!(sci.geometry_type(), Kind::Curve);
        assert_eq!(sci.geometry(), 10);
        assert_eq!(sci.param_on_curve(), 2.5);
        assert_eq!(sci.param_u(), 1.0);
        assert_eq!(sci.param_v(), 1.5);
    }

    #[test]
    fn test_surface_curve_interference_params() {
        let t = Transition::new();
        let mut sci = SurfaceCurveInterference::new(t, Kind::Surface, 1, Kind::Curve, 2, 0.5, 0.1, 0.2);
        assert_eq!(sci.param_on_curve(), 0.5);
        sci.set_param_on_curve(3.0);
        assert_eq!(sci.param_on_curve(), 3.0);

        sci.set_param_u(2.0);
        sci.set_param_v(3.0);
        assert_eq!(sci.param_u(), 2.0);
        assert_eq!(sci.param_v(), 3.0);
    }
}
