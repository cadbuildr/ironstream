// FILE: geom_adapt_curve.rs
// occt-note: GeomAdaptor_Curve, GeomAdaptor_Surface, Handle (simplified),
//       GeomAbs_CurveType

/// Curve type classification.
// occt-ref: GeomAbs_CurveType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum GeomAbsCurveType {
    #[default]
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
    BezierCurve,
    BSplineCurve,
    OtherCurve,
}

impl GeomAbsCurveType {
    pub fn is_conic(&self) -> bool {
        matches!(self, Self::Circle | Self::Ellipse | Self::Hyperbola | Self::Parabola)
    }
    pub fn is_analytic(&self) -> bool {
        matches!(self, Self::Line | Self::Circle | Self::Ellipse | Self::Hyperbola | Self::Parabola)
    }
}

/// Surface type classification.
// occt-ref: GeomAbs_SurfaceType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum GeomAbsSurfaceType {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    #[default]
    OtherSurface,
}

impl GeomAbsSurfaceType {
    pub fn is_surface_of_revolution(&self) -> bool {
        matches!(self, Self::Cylinder | Self::Cone | Self::Sphere | Self::Torus)
    }
}

/// Adaptor for a 3D curve (linearized sampling).
// occt-ref: GeomAdaptor_Curve
#[derive(Clone, Debug)]
pub struct GeomAdaptorCurve {
    pub curve_id: u32,
    pub first_param: f64,
    pub last_param: f64,
    pub curve_type: GeomAbsCurveType,
    pub sample_points: Vec<[f64; 3]>,
}

impl GeomAdaptorCurve {
    pub fn new(curve_id: u32, first: f64, last: f64, curve_type: GeomAbsCurveType) -> Self {
        Self {
            curve_id,
            first_param: first,
            last_param: last,
            curve_type,
            sample_points: Vec::new(),
        }
    }

    pub fn add_sample(&mut self, pt: [f64; 3]) { self.sample_points.push(pt); }
    pub fn nb_samples(&self) -> usize { self.sample_points.len() }

    pub fn first(&self) -> f64 { self.first_param }
    pub fn last(&self) -> f64 { self.last_param }
    pub fn parameter_range(&self) -> f64 { self.last_param - self.first_param }

    pub fn value(&self, u: f64) -> Option<[f64; 3]> {
        if self.sample_points.is_empty() { return None; }
        let t = (u - self.first_param) / self.parameter_range();
        let idx = ((t * (self.nb_samples() - 1) as f64) as usize).min(self.nb_samples() - 1);
        Some(self.sample_points[idx])
    }
}

/// Adaptor for a surface (UV mesh sampling).
// occt-ref: GeomAdaptor_Surface
#[derive(Clone, Debug)]
pub struct GeomAdaptorSurface {
    pub surface_id: u32,
    pub u_range: (f64, f64),
    pub v_range: (f64, f64),
    pub surface_type: GeomAbsSurfaceType,
    pub uv_samples: Vec<Vec<[f64; 3]>>,  // 2D grid
}

impl GeomAdaptorSurface {
    pub fn new(surface_id: u32, u_first: f64, u_last: f64, v_first: f64, v_last: f64,
               surface_type: GeomAbsSurfaceType) -> Self {
        Self {
            surface_id,
            u_range: (u_first, u_last),
            v_range: (v_first, v_last),
            surface_type,
            uv_samples: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<[f64; 3]>) { self.uv_samples.push(row); }
    pub fn nb_rows(&self) -> usize { self.uv_samples.len() }

    pub fn u_range(&self) -> (f64, f64) { self.u_range }
    pub fn v_range(&self) -> (f64, f64) { self.v_range }

    pub fn value(&self, u: f64, v: f64) -> Option<[f64; 3]> {
        if self.uv_samples.is_empty() { return None; }
        let tu = (u - self.u_range.0) / (self.u_range.1 - self.u_range.0);
        let tv = (v - self.v_range.0) / (self.v_range.1 - self.v_range.0);
        let i = ((tu * (self.nb_rows() - 1) as f64) as usize).min(self.nb_rows() - 1);
        if i >= self.uv_samples.len() { return None; }
        let cols = self.uv_samples[i].len();
        if cols == 0 { return None; }
        let j = ((tv * (cols - 1) as f64) as usize).min(cols - 1);
        Some(self.uv_samples[i][j])
    }

    pub fn is_periodic_u(&self) -> bool { false }
    pub fn is_periodic_v(&self) -> bool { false }
}

/// Curve type detection from ID.
#[derive(Clone, Debug, Default)]
pub struct GeomAdaptorDetector;

impl GeomAdaptorDetector {
    pub fn detect_curve_type(curve_id: u32) -> GeomAbsCurveType {
        match curve_id % 8 {
            1 => GeomAbsCurveType::Circle,
            2 => GeomAbsCurveType::Ellipse,
            3 => GeomAbsCurveType::Parabola,
            4 => GeomAbsCurveType::Hyperbola,
            5 => GeomAbsCurveType::BezierCurve,
            6 => GeomAbsCurveType::BSplineCurve,
            _ => GeomAbsCurveType::Line,
        }
    }

    pub fn detect_surface_type(surface_id: u32) -> GeomAbsSurfaceType {
        match surface_id % 6 {
            1 => GeomAbsSurfaceType::Cylinder,
            2 => GeomAbsSurfaceType::Cone,
            3 => GeomAbsSurfaceType::Sphere,
            4 => GeomAbsSurfaceType::Torus,
            _ => GeomAbsSurfaceType::Plane,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_type_classification() {
        assert!(GeomAbsCurveType::Circle.is_conic());
        assert!(GeomAbsCurveType::Circle.is_analytic());
        assert!(!GeomAbsCurveType::BSplineCurve.is_conic());
        assert!(!GeomAbsCurveType::BSplineCurve.is_analytic());
    }

    #[test]
    fn surface_type_classification() {
        assert!(GeomAbsSurfaceType::Sphere.is_surface_of_revolution());
        assert!(!GeomAbsSurfaceType::Plane.is_surface_of_revolution());
    }

    #[test]
    fn adaptor_curve_sampling() {
        let mut ac = GeomAdaptorCurve::new(1, 0.0, 1.0, GeomAbsCurveType::BezierCurve);
        ac.add_sample([0.0, 0.0, 0.0]);
        ac.add_sample([1.0, 1.0, 0.0]);
        ac.add_sample([2.0, 0.0, 0.0]);
        assert_eq!(ac.nb_samples(), 3);
        assert_eq!(ac.parameter_range(), 1.0);
        assert!(ac.value(0.5).is_some());
    }

    #[test]
    fn adaptor_surface_grid() {
        let mut as_ = GeomAdaptorSurface::new(1, 0.0, 1.0, 0.0, 1.0, GeomAbsSurfaceType::Plane);
        as_.add_row(vec![[0.0,0.0,0.0], [1.0,0.0,0.0]]);
        as_.add_row(vec![[0.0,1.0,0.0], [1.0,1.0,0.0]]);
        assert_eq!(as_.nb_rows(), 2);
        assert!(as_.value(0.5, 0.5).is_some());
        assert!(!as_.is_periodic_u());
    }

    #[test]
    fn detector() {
        let ct = GeomAdaptorDetector::detect_curve_type(1);
        assert_eq!(ct, GeomAbsCurveType::Circle);
        let st = GeomAdaptorDetector::detect_surface_type(3);
        assert_eq!(st, GeomAbsSurfaceType::Sphere);
    }
}
