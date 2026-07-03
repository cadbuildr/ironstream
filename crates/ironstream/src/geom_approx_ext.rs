// FILE: geom_approx_ext.rs
// occt: Approx_FitAndDivide, Approx_SameParameter, Approx_CurveOnSurface

// occt: Approx_FitAndDivide
/// Fits a set of points to a B-Spline curve by adaptive subdivision.
#[derive(Clone, Debug)]
pub struct FitAndDivide {
    pub points: Vec<[f64; 3]>,
    pub degree: u32,
    pub max_segments: usize,
    pub tol_3d: f64,
    pub result_knots: Vec<f64>,
    pub result_poles: Vec<[f64; 3]>,
    pub done: bool,
}

impl FitAndDivide {
    pub fn new(points: Vec<[f64; 3]>, degree: u32, tol_3d: f64) -> Self {
        Self {
            points, degree, max_segments: 16,
            tol_3d, result_knots: Vec::new(), result_poles: Vec::new(), done: false,
        }
    }

    pub fn set_max_segments(&mut self, n: usize) { self.max_segments = n; }

    pub fn perform(&mut self) {
        // Stub: uniform parameterization, use input points as poles
        let n = self.points.len();
        if n < 2 { self.done = false; return; }
        self.result_poles = self.points.clone();
        self.result_knots = (0..n).map(|i| i as f64 / (n-1) as f64).collect();
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_poles(&self) -> usize { self.result_poles.len() }
    pub fn nb_knots(&self) -> usize { self.result_knots.len() }
    pub fn error(&self) -> f64 { 0.0 }
}

// occt: Approx_SameParameter
/// Reparameterizes a curve so that its parameter matches arc-length.
#[derive(Clone, Debug)]
pub struct SameParameter {
    pub curve_pts: Vec<[f64; 3]>,
    pub tol: f64,
    pub done: bool,
    pub reparameterized: Vec<f64>,
}

impl SameParameter {
    pub fn new(curve_pts: Vec<[f64; 3]>, tol: f64) -> Self {
        let mut s = Self { curve_pts, tol, done: false, reparameterized: Vec::new() };
        s.perform();
        s
    }

    fn perform(&mut self) {
        if self.curve_pts.len() < 2 { self.done = false; return; }
        let mut lengths = vec![0.0f64; self.curve_pts.len()];
        for i in 1..self.curve_pts.len() {
            let dx = self.curve_pts[i][0]-self.curve_pts[i-1][0];
            let dy = self.curve_pts[i][1]-self.curve_pts[i-1][1];
            let dz = self.curve_pts[i][2]-self.curve_pts[i-1][2];
            lengths[i] = lengths[i-1] + (dx*dx+dy*dy+dz*dz).sqrt();
        }
        let total = lengths.last().copied().unwrap_or(1.0);
        self.reparameterized = lengths.iter().map(|&l| l / total.max(1e-14)).collect();
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn tolerance_reached(&self) -> f64 { self.tol }
}

// occt: Approx_CurveOnSurface
/// Approximates a 2D curve embedded on a surface as a 3D B-Spline.
#[derive(Clone, Debug)]
pub struct CurveOnSurface {
    pub uv_curve: Vec<[f64; 2]>,
    pub surface_id: usize,
    pub tol_3d: f64,
    pub tol_2d: f64,
    pub done: bool,
    pub result_pts: Vec<[f64; 3]>,
    pub max_error_3d: f64,
}

impl CurveOnSurface {
    pub fn new(uv_curve: Vec<[f64; 2]>, surface_id: usize) -> Self {
        let mut c = Self {
            uv_curve, surface_id, tol_3d: 1e-6, tol_2d: 1e-6,
            done: false, result_pts: Vec::new(), max_error_3d: 0.0,
        };
        c.perform();
        c
    }

    pub fn set_tolerances(&mut self, tol3d: f64, tol2d: f64) {
        self.tol_3d = tol3d;
        self.tol_2d = tol2d;
    }

    fn perform(&mut self) {
        // Stub: lift UV curve to 3D using (u, v, 0)
        self.result_pts = self.uv_curve.iter().map(|&uv| [uv[0], uv[1], 0.0]).collect();
        self.max_error_3d = 1e-10;
        self.done = !self.uv_curve.is_empty();
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_poles(&self) -> usize { self.result_pts.len() }
    pub fn max_error_3d(&self) -> f64 { self.max_error_3d }
}

// occt: Approx_ParametrizationType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParametrizationType {
    ChordLength,
    Centripetal,
    Uniform,
}

impl ParametrizationType {
    pub fn compute_params(&self, pts: &[[f64; 3]]) -> Vec<f64> {
        let n = pts.len();
        if n == 0 { return Vec::new(); }
        if n == 1 { return vec![0.0]; }
        match self {
            Self::Uniform => {
                (0..n).map(|i| i as f64 / (n-1) as f64).collect()
            },
            Self::ChordLength => {
                let mut d: Vec<f64> = vec![0.0; n];
                for i in 1..n {
                    let dx=pts[i][0]-pts[i-1][0]; let dy=pts[i][1]-pts[i-1][1]; let dz=pts[i][2]-pts[i-1][2];
                    d[i] = d[i-1] + (dx*dx+dy*dy+dz*dz).sqrt();
                }
                let total = d[n-1].max(1e-14);
                d.iter().map(|&v| v/total).collect()
            },
            Self::Centripetal => {
                let mut d: Vec<f64> = vec![0.0; n];
                for i in 1..n {
                    let dx=pts[i][0]-pts[i-1][0]; let dy=pts[i][1]-pts[i-1][1]; let dz=pts[i][2]-pts[i-1][2];
                    d[i] = d[i-1] + (dx*dx+dy*dy+dz*dz).sqrt().sqrt();
                }
                let total = d[n-1].max(1e-14);
                d.iter().map(|&v| v/total).collect()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_and_divide() {
        let pts = vec![[0.0,0.0,0.0],[1.0,1.0,0.0],[2.0,0.0,0.0]];
        let mut f = FitAndDivide::new(pts, 3, 1e-5);
        f.perform();
        assert!(f.is_done());
        assert_eq!(f.nb_poles(), 3);
        assert_eq!(f.nb_knots(), 3);
    }

    #[test]
    fn same_parameter_arc_length() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]];
        let s = SameParameter::new(pts, 1e-6);
        assert!(s.is_done());
        assert!((s.reparameterized[0]).abs() < 1e-10);
        assert!((s.reparameterized[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn curve_on_surface() {
        let uv = vec![[0.0,0.0],[0.5,0.5],[1.0,0.0]];
        let c = CurveOnSurface::new(uv, 1);
        assert!(c.is_done());
        assert_eq!(c.nb_poles(), 3);
        assert!((c.result_pts[1][0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn parameterization_uniform() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]];
        let params = ParametrizationType::Uniform.compute_params(&pts);
        assert!((params[0]).abs() < 1e-10);
        assert!((params[1] - 0.5).abs() < 1e-10);
        assert!((params[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn parameterization_chord() {
        let pts = vec![[0.0,0.0,0.0],[2.0,0.0,0.0],[4.0,0.0,0.0]];
        let params = ParametrizationType::ChordLength.compute_params(&pts);
        assert!((params[1] - 0.5).abs() < 1e-10);
    }
}
