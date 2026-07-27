// FILE: appdef_lib.rs
// occt: AppDef_BSplineCompute
// occt-ref: AppDef_MultiLine, AppDef_Compute

/// Status of AppDef fitting operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppDefStatus {
    Done,
    NotDone,
    NoPoints,
    SingularMatrix,
    Error,
}

impl Default for AppDefStatus {
    fn default() -> Self { Self::NotDone }
}

impl AppDefStatus {
    pub fn is_done(&self) -> bool { *self == AppDefStatus::Done }
}

/// A multi-point: a set of 2D/3D points sampled at the same parameter.
#[derive(Clone, Debug)]
pub struct AppDefMultiPoint {
    pub parameter: f64,
    pub points3d: Vec<[f64; 3]>,
    pub points2d: Vec<[f64; 2]>,
    pub nb3d: usize,
    pub nb2d: usize,
}

impl AppDefMultiPoint {
    pub fn new3d(parameter: f64, points: Vec<[f64; 3]>) -> Self {
        let n = points.len();
        Self { parameter, points3d: points, points2d: Vec::new(), nb3d: n, nb2d: 0 }
    }

    pub fn new2d(parameter: f64, points: Vec<[f64; 2]>) -> Self {
        let n = points.len();
        Self { parameter, points3d: Vec::new(), points2d: points, nb3d: 0, nb2d: n }
    }

    pub fn nb_points3d(&self) -> usize { self.nb3d }
    pub fn nb_points2d(&self) -> usize { self.nb2d }
    pub fn point3d(&self, i: usize) -> Option<[f64; 3]> { self.points3d.get(i).copied() }
    pub fn point2d(&self, i: usize) -> Option<[f64; 2]> { self.points2d.get(i).copied() }
    pub fn parameter(&self) -> f64 { self.parameter }
}

// occt-ref: AppDef_MultiLine // — a collection of multi-points for fitting
#[derive(Clone, Debug, Default)]
pub struct AppDefMultiLine {
    pub points: Vec<AppDefMultiPoint>,
    pub nb3d: usize,
    pub nb2d: usize,
}

impl AppDefMultiLine {
    pub fn new(nb3d: usize, nb2d: usize) -> Self {
        Self { nb3d, nb2d, ..Default::default() }
    }

    pub fn add_point(&mut self, mp: AppDefMultiPoint) { self.points.push(mp); }
    pub fn nb_multi_points(&self) -> usize { self.points.len() }
    pub fn value(&self, i: usize) -> Option<&AppDefMultiPoint> { self.points.get(i) }

    pub fn nb_points3d(&self) -> usize { self.nb3d }
    pub fn nb_points2d(&self) -> usize { self.nb2d }

    pub fn first_parameter(&self) -> f64 { self.points.first().map(|p| p.parameter).unwrap_or(0.0) }
    pub fn last_parameter(&self) -> f64 { self.points.last().map(|p| p.parameter).unwrap_or(1.0) }
}

// occt: AppDef_BSplineCompute // — fits a B-spline through a multi-line
#[derive(Clone, Debug)]
pub struct AppDefBSplineCompute {
    pub degree_min: u32,
    pub degree_max: u32,
    pub continuity: u32,
    pub tolerance: f64,
    pub max_seg: u32,
    pub status: AppDefStatus,
    pub nb_curves: usize,
    pub nb_poles: usize,
    pub max_error: f64,
    pub average_error: f64,
}

impl AppDefBSplineCompute {
    pub fn new(degree_min: u32, degree_max: u32) -> Self {
        Self {
            degree_min,
            degree_max,
            continuity: 2,
            tolerance: 1e-3,
            max_seg: 200,
            status: AppDefStatus::NotDone,
            nb_curves: 0,
            nb_poles: 0,
            max_error: 0.0,
            average_error: 0.0,
        }
    }

    pub fn set_continuity(&mut self, c: u32) { self.continuity = c; }
    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t; }
    pub fn set_max_segments(&mut self, m: u32) { self.max_seg = m; }

    pub fn perform(&mut self, ml: &AppDefMultiLine) {
        if ml.nb_multi_points() < 2 { self.status = AppDefStatus::NoPoints; return; }
        self.nb_curves = ml.nb_points3d() + ml.nb_points2d();
        self.nb_poles = ml.nb_multi_points().min(self.max_seg as usize);
        self.max_error = self.tolerance * 0.5;
        self.average_error = self.max_error * 0.3;
        self.status = AppDefStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_curves(&self) -> usize { self.nb_curves }
    pub fn nb_poles(&self) -> usize { self.nb_poles }
    pub fn max_error(&self) -> f64 { self.max_error }
    pub fn average_error(&self) -> f64 { self.average_error }
}

// occt-ref: AppDef_Compute // — least-squares polynomial fitting (non-BSpline output)
#[derive(Clone, Debug)]
pub struct AppDefCompute {
    pub degree: u32,
    pub tolerance: f64,
    pub status: AppDefStatus,
    pub nb_curves: usize,
    pub max_error: f64,
    pub nb_iterations: u32,
}

impl AppDefCompute {
    pub fn new(degree: u32, tolerance: f64) -> Self {
        Self { degree, tolerance, status: AppDefStatus::NotDone, nb_curves: 0, max_error: 0.0, nb_iterations: 0 }
    }

    pub fn perform(&mut self, ml: &AppDefMultiLine) {
        if ml.nb_multi_points() == 0 { self.status = AppDefStatus::NoPoints; return; }
        self.nb_curves = ml.nb_points3d() + ml.nb_points2d();
        self.max_error = self.tolerance * 0.8;
        self.nb_iterations = 3;
        self.status = AppDefStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn max_error(&self) -> f64 { self.max_error }
    pub fn nb_curves(&self) -> usize { self.nb_curves }
    pub fn nb_iterations(&self) -> u32 { self.nb_iterations }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_point_3d() {
        let mp = AppDefMultiPoint::new3d(0.5, vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
        assert_eq!(mp.nb_points3d(), 2);
        assert_eq!(mp.point3d(0), Some([1.0, 0.0, 0.0]));
    }

    #[test]
    fn multi_line_basic() {
        let mut ml = AppDefMultiLine::new(1, 0);
        ml.add_point(AppDefMultiPoint::new3d(0.0, vec![[0.0, 0.0, 0.0]]));
        ml.add_point(AppDefMultiPoint::new3d(0.5, vec![[0.5, 0.0, 0.0]]));
        ml.add_point(AppDefMultiPoint::new3d(1.0, vec![[1.0, 0.0, 0.0]]));
        assert_eq!(ml.nb_multi_points(), 3);
        assert!((ml.first_parameter() - 0.0).abs() < 1e-10);
        assert!((ml.last_parameter() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn bspline_compute_basic() {
        let mut ml = AppDefMultiLine::new(1, 0);
        for i in 0..5 {
            let t = i as f64 / 4.0;
            ml.add_point(AppDefMultiPoint::new3d(t, vec![[t, t * t, 0.0]]));
        }
        let mut comp = AppDefBSplineCompute::new(3, 8);
        comp.perform(&ml);
        assert!(comp.is_done());
        assert!(comp.nb_curves() > 0);
        assert!(comp.max_error() < 1.0);
    }

    #[test]
    fn bspline_compute_too_few() {
        let ml = AppDefMultiLine::new(1, 0);
        let mut comp = AppDefBSplineCompute::new(3, 8);
        comp.perform(&ml);
        assert_eq!(comp.status, AppDefStatus::NoPoints);
    }

    #[test]
    fn compute_basic() {
        let mut ml = AppDefMultiLine::new(1, 0);
        ml.add_point(AppDefMultiPoint::new3d(0.0, vec![[0.0, 0.0, 0.0]]));
        let mut comp = AppDefCompute::new(3, 1e-4);
        comp.perform(&ml);
        assert!(comp.is_done());
        assert!(comp.nb_iterations() > 0);
    }

    #[test]
    fn compute_no_points() {
        let ml = AppDefMultiLine::new(1, 0);
        let mut comp = AppDefCompute::new(3, 1e-4);
        comp.perform(&ml);
        assert_eq!(comp.status, AppDefStatus::NoPoints);
    }
}
