// FILE: geom_surface_quality.rs
// occt: GeomLib_CheckBSplineCurve
// occt-ref: ShapeAnalysis_Surface, GeomLib_IsBoundedCurve

// occt-ref: ShapeAnalysis_Surface
/// Analyzes parametric surface quality and find UV bounds.
#[derive(Clone, Debug)]
pub struct SurfaceAnalysis {
    pub surface_id: usize,
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
    pub uv_resolution: f64,
}

impl SurfaceAnalysis {
    pub fn new(surface_id: usize, u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        Self { surface_id, u_min, u_max, v_min, v_max, uv_resolution: 1e-7 }
    }

    pub fn set_uv_resolution(&mut self, r: f64) { self.uv_resolution = r; }

    pub fn u_resolution(&self, distance: f64) -> f64 {
        let span = (self.u_max - self.u_min).max(1e-14);
        distance / span
    }

    pub fn v_resolution(&self, distance: f64) -> f64 {
        let span = (self.v_max - self.v_min).max(1e-14);
        distance / span
    }

    pub fn project_uv(&self, u: f64, v: f64) -> Option<[f64; 2]> {
        if u >= self.u_min && u <= self.u_max && v >= self.v_min && v <= self.v_max {
            Some([u, v])
        } else { None }
    }

    pub fn is_u_closed(&self, tol: f64) -> bool {
        (self.u_max - self.u_min - 2.0 * std::f64::consts::PI).abs() < tol
    }

    pub fn parametric_area(&self) -> f64 {
        (self.u_max - self.u_min) * (self.v_max - self.v_min)
    }
}

// occt: GeomLib_CheckBSplineCurve
#[derive(Clone, Debug)]
pub struct CheckBSplineCurve {
    pub poles: Vec<[f64; 3]>,
    pub knots: Vec<f64>,
    pub degree: u32,
    pub is_c1: bool,
    pub is_g1: bool,
    pub max_error: f64,
}

impl CheckBSplineCurve {
    pub fn new(poles: Vec<[f64; 3]>, knots: Vec<f64>, degree: u32) -> Self {
        let mut c = Self { poles, knots, degree, is_c1: true, is_g1: true, max_error: 0.0 };
        c.analyze();
        c
    }

    fn analyze(&mut self) {
        // Stub: mark as C1 unless there are duplicate knots at an interior point
        let n = self.knots.len();
        for i in 1..n.saturating_sub(1) {
            if i + 1 < n && (self.knots[i] - self.knots[i+1]).abs() < 1e-14 {
                self.is_c1 = false;
                break;
            }
        }
        self.is_g1 = self.is_c1; // simplified
    }

    pub fn is_c1_continuous(&self) -> bool { self.is_c1 }
    pub fn is_g1_continuous(&self) -> bool { self.is_g1 }
    pub fn max_deviation(&self) -> f64 { self.max_error }
}

// occt-ref: GeomLib_IsBoundedCurve
/// Returns whether a curve has finite parameter domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveBoundedness {
    Bounded,
    BoundedOnLeft,
    BoundedOnRight,
    Unbounded,
}

pub fn is_bounded(t_min: f64, t_max: f64) -> CurveBoundedness {
    let inf = f64::INFINITY;
    match (t_min.is_finite(), t_max.is_finite()) {
        (true, true) => CurveBoundedness::Bounded,
        (true, false) => CurveBoundedness::BoundedOnLeft,
        (false, true) => CurveBoundedness::BoundedOnRight,
        (false, false) => CurveBoundedness::Unbounded,
    }
}

// occt-ref: ShapeAnalysis_Curve
#[derive(Clone, Debug)]
pub struct CurveAnalysis {
    pub pts: Vec<[f64; 3]>,
}

impl CurveAnalysis {
    pub fn new(pts: Vec<[f64; 3]>) -> Self { Self { pts } }

    pub fn length(&self) -> f64 {
        self.pts.windows(2).map(|w| {
            let dx=w[1][0]-w[0][0]; let dy=w[1][1]-w[0][1]; let dz=w[1][2]-w[0][2];
            (dx*dx+dy*dy+dz*dz).sqrt()
        }).sum()
    }

    pub fn max_deviation_from_line(&self) -> f64 {
        if self.pts.len() < 3 { return 0.0; }
        let start = self.pts[0];
        let end = *self.pts.last().unwrap();
        let dx = end[0]-start[0]; let dy = end[1]-start[1]; let dz = end[2]-start[2];
        let len = (dx*dx+dy*dy+dz*dz).sqrt();
        if len < 1e-14 { return 0.0; }
        self.pts.iter().map(|&p| {
            let t = ((p[0]-start[0])*dx + (p[1]-start[1])*dy + (p[2]-start[2])*dz) / (len*len);
            let proj = [start[0]+t*dx, start[1]+t*dy, start[2]+t*dz];
            let ex=p[0]-proj[0]; let ey=p[1]-proj[1]; let ez=p[2]-proj[2];
            (ex*ex+ey*ey+ez*ez).sqrt()
        }).fold(0.0_f64, f64::max)
    }

    pub fn is_closed(&self, tol: f64) -> bool {
        if self.pts.len() < 3 { return false; }
        let a = self.pts[0]; let b = *self.pts.last().unwrap();
        let dx=a[0]-b[0]; let dy=a[1]-b[1]; let dz=a[2]-b[2];
        (dx*dx+dy*dy+dz*dz).sqrt() < tol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_analysis() {
        let s = SurfaceAnalysis::new(1, 0.0, 2.0*std::f64::consts::PI, -1.0, 1.0);
        assert!(s.is_u_closed(1e-6));
        assert!((s.parametric_area() - 4.0*std::f64::consts::PI).abs() < 1e-8);
    }

    #[test]
    fn surface_project_uv() {
        let s = SurfaceAnalysis::new(1, 0.0, 1.0, 0.0, 1.0);
        assert!(s.project_uv(0.5, 0.5).is_some());
        assert!(s.project_uv(2.0, 0.5).is_none());
    }

    #[test]
    fn check_bspline_curve() {
        let poles = vec![[0.0,0.0,0.0],[1.0,1.0,0.0],[2.0,0.0,0.0]];
        let knots = vec![0.0, 0.5, 1.0];
        let c = CheckBSplineCurve::new(poles, knots, 2);
        assert!(c.is_c1_continuous());
    }

    #[test]
    fn is_bounded_fn() {
        assert_eq!(is_bounded(0.0, 1.0), CurveBoundedness::Bounded);
        assert_eq!(is_bounded(0.0, f64::INFINITY), CurveBoundedness::BoundedOnLeft);
        assert_eq!(is_bounded(f64::NEG_INFINITY, f64::INFINITY), CurveBoundedness::Unbounded);
    }

    #[test]
    fn curve_analysis() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.5,0.0],[2.0,0.0,0.0]];
        let a = CurveAnalysis::new(pts);
        assert!(a.length() > 2.0);
        assert!(a.max_deviation_from_line() > 0.1);
        assert!(!a.is_closed(1e-6));
    }
}
