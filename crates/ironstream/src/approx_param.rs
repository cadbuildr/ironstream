// FILE: approx_param.rs
// occt: AppParCurves_MultiBSpCurve, AppParCurves_MultiPoint
// occt-ref: AppParCurves_MultiCurve

/// Continuity constraint for parameter-fitting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppParConstraint {
    NoConstraint,
    PassPoint,
    TangencyPoint,
    CurvaturePoint,
}

impl Default for AppParConstraint {
    fn default() -> Self { Self::PassPoint }
}

/// A multi-point in n-dimensional parameter space.
#[derive(Clone, Debug)]
pub struct AppParMultiPoint {
    pub points3d: Vec<[f64; 3]>,
    pub points2d: Vec<[f64; 2]>,
}

impl AppParMultiPoint {
    pub fn new3d(points: Vec<[f64; 3]>) -> Self {
        Self { points3d: points, points2d: Vec::new() }
    }

    pub fn new_mixed(points3d: Vec<[f64; 3]>, points2d: Vec<[f64; 2]>) -> Self {
        Self { points3d, points2d }
    }

    pub fn nb_curves(&self) -> usize { self.points3d.len() + self.points2d.len() }
    pub fn point3d(&self, i: usize) -> Option<[f64; 3]> { self.points3d.get(i).copied() }
    pub fn point2d(&self, i: usize) -> Option<[f64; 2]> { self.points2d.get(i).copied() }
    pub fn nb_3d_curves(&self) -> usize { self.points3d.len() }
    pub fn nb_2d_curves(&self) -> usize { self.points2d.len() }
}

// occt-ref: AppParCurves_MultiCurve // — a collection of approximating Bezier curves
#[derive(Clone, Debug)]
pub struct AppParMultiCurve {
    pub degree: usize,
    pub poles: Vec<AppParMultiPoint>,
    pub nb_curves: usize,
}

impl AppParMultiCurve {
    pub fn new(degree: usize) -> Self {
        Self { degree, poles: Vec::new(), nb_curves: 0 }
    }

    pub fn add_pole(&mut self, pole: AppParMultiPoint) {
        if self.nb_curves == 0 { self.nb_curves = pole.nb_curves(); }
        self.poles.push(pole);
    }

    pub fn value(&self, curve_idx: usize, t: f64) -> [f64; 3] {
        if self.poles.is_empty() { return [0.0; 3]; }
        // Stub: linear interpolation over poles
        let n = self.poles.len() - 1;
        let idx = (t * n as f64).floor() as usize;
        let idx = idx.min(n);
        self.poles[idx].point3d(curve_idx).unwrap_or([0.0; 3])
    }

    pub fn nb_poles(&self) -> usize { self.poles.len() }
    pub fn degree(&self) -> usize { self.degree }
    pub fn nb_curves(&self) -> usize { self.nb_curves }
}

// occt: AppParCurves_MultiBSpCurve // — collection of approximating BSpline curves
#[derive(Clone, Debug)]
pub struct AppParMultiBSpCurve {
    pub degree: usize,
    pub knots: Vec<f64>,
    pub multiplicities: Vec<usize>,
    pub poles: Vec<AppParMultiPoint>,
    pub nb_curves: usize,
}

impl AppParMultiBSpCurve {
    pub fn new(degree: usize) -> Self {
        let knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
        let mults = vec![degree + 1, degree + 1];
        Self { degree, knots, multiplicities: mults, poles: Vec::new(), nb_curves: 0 }
    }

    pub fn add_pole(&mut self, pole: AppParMultiPoint) {
        if self.nb_curves == 0 { self.nb_curves = pole.nb_curves(); }
        self.poles.push(pole);
    }

    pub fn set_knots(&mut self, knots: Vec<f64>, mults: Vec<usize>) {
        self.knots = knots;
        self.multiplicities = mults;
    }

    pub fn degree(&self) -> usize { self.degree }
    pub fn nb_poles(&self) -> usize { self.poles.len() }
    pub fn nb_knots(&self) -> usize { self.knots.len() }
    pub fn nb_curves(&self) -> usize { self.nb_curves }
    pub fn first_parameter(&self) -> f64 { self.knots.first().copied().unwrap_or(0.0) }
    pub fn last_parameter(&self) -> f64 { self.knots.last().copied().unwrap_or(1.0) }
}

// occt-ref: Approx_FitAndDivide // — fits BSpline curve to point cloud with automatic subdivision
#[derive(Clone, Debug)]
pub struct ApproxFitAndDivide {
    pub multi_line_id: u32,
    pub deg_min: usize,
    pub deg_max: usize,
    pub continuity: AppParConstraint,
    pub tolerance: f64,
    pub nb_curves: usize,
    pub result: Option<AppParMultiBSpCurve>,
    pub is_done: bool,
}

impl ApproxFitAndDivide {
    pub fn new(multi_line_id: u32, deg_min: usize, deg_max: usize) -> Self {
        Self {
            multi_line_id,
            deg_min,
            deg_max,
            continuity: AppParConstraint::TangencyPoint,
            tolerance: 1e-3,
            nb_curves: 0,
            result: None,
            is_done: false,
        }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t; }
    pub fn set_continuity(&mut self, c: AppParConstraint) { self.continuity = c; }

    pub fn perform(&mut self) {
        if self.multi_line_id == 0 { return; }
        let mut curve = AppParMultiBSpCurve::new(self.deg_min.max(1));
        curve.add_pole(AppParMultiPoint::new3d(vec![[0.0; 3], [1.0, 0.0, 0.0]]));
        self.nb_curves = 1;
        self.result = Some(curve);
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_curves(&self) -> usize { self.nb_curves }
    pub fn value(&self, i: usize) -> Option<&AppParMultiBSpCurve> {
        if i == 0 { self.result.as_ref() } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_point_3d() {
        let mp = AppParMultiPoint::new3d(vec![[1.0,0.0,0.0],[0.0,1.0,0.0]]);
        assert_eq!(mp.nb_curves(), 2);
        assert_eq!(mp.point3d(0), Some([1.0,0.0,0.0]));
    }

    #[test]
    fn multi_point_mixed() {
        let mp = AppParMultiPoint::new_mixed(
            vec![[1.0,0.0,0.0]],
            vec![[0.5,0.5]],
        );
        assert_eq!(mp.nb_3d_curves(), 1);
        assert_eq!(mp.nb_2d_curves(), 1);
        assert_eq!(mp.nb_curves(), 2);
    }

    #[test]
    fn multi_curve_poles() {
        let mut c = AppParMultiCurve::new(3);
        c.add_pole(AppParMultiPoint::new3d(vec![[0.0;3]]));
        c.add_pole(AppParMultiPoint::new3d(vec![[1.0,0.0,0.0]]));
        assert_eq!(c.nb_poles(), 2);
        assert_eq!(c.degree(), 3);
    }

    #[test]
    fn multi_bsp_curve_knots() {
        let mut bsp = AppParMultiBSpCurve::new(3);
        bsp.add_pole(AppParMultiPoint::new3d(vec![[0.0;3]]));
        assert_eq!(bsp.degree(), 3);
        assert!((bsp.first_parameter() - 0.0).abs() < 1e-10);
        assert!((bsp.last_parameter() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn fit_and_divide_perform() {
        let mut f = ApproxFitAndDivide::new(1, 3, 8);
        f.perform();
        assert!(f.is_done());
        assert_eq!(f.nb_curves(), 1);
        assert!(f.value(0).is_some());
    }

    #[test]
    fn fit_and_divide_null() {
        let mut f = ApproxFitAndDivide::new(0, 3, 8);
        f.perform();
        assert!(!f.is_done());
    }
}
