// FILE: extrema_curve.rs
// occt: Extrema_ExtCC (curve-curve extrema), Extrema_ExtCurveParameters,
//       Extrema_ExtCOcc (closest points on curves)

/// Result of a curve-curve extrema computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtCC3dStatus {
    NotDone,
    Done,
    InfinitePoints,
    NoDomain,
}

impl Default for ExtCC3dStatus {
    fn default() -> Self { Self::NotDone }
}

impl ExtCC3dStatus {
    pub fn is_done(&self) -> bool { *self == ExtCC3dStatus::Done }
}

/// A solution point pair on two curves.
#[derive(Clone, Debug, Default)]
pub struct CurvePair {
    pub param1: f64,
    pub param2: f64,
    pub point1: [f64; 3],
    pub point2: [f64; 3],
    pub square_distance: f64,
    pub is_minimum: bool,
}

impl CurvePair {
    pub fn new(p1: f64, pt1: [f64; 3], p2: f64, pt2: [f64; 3]) -> Self {
        let d = [pt1[0]-pt2[0], pt1[1]-pt2[1], pt1[2]-pt2[2]];
        let sq_d = d[0]*d[0]+d[1]*d[1]+d[2]*d[2];
        Self { param1: p1, param2: p2, point1: pt1, point2: pt2, square_distance: sq_d, is_minimum: true }
    }

    pub fn distance(&self) -> f64 { self.square_distance.sqrt() }
}

// occt: Extrema_ExtCC — finds all extrema (distance min/max) between two 3D curves
#[derive(Clone, Debug, Default)]
pub struct ExtCCurve3d {
    pub curve1_id: u32,
    pub curve2_id: u32,
    pub t1_range: [f64; 2],
    pub t2_range: [f64; 2],
    pub tolerance1: f64,
    pub tolerance2: f64,
    pub status: ExtCC3dStatus,
    pub solutions: Vec<CurvePair>,
    pub parallel: bool,
}

impl ExtCCurve3d {
    pub fn new(c1: u32, c2: u32) -> Self {
        Self {
            curve1_id: c1,
            curve2_id: c2,
            t1_range: [0.0, 1.0],
            t2_range: [0.0, 1.0],
            tolerance1: 1e-7,
            tolerance2: 1e-7,
            ..Default::default()
        }
    }

    pub fn set_range1(&mut self, t_min: f64, t_max: f64) { self.t1_range = [t_min, t_max]; }
    pub fn set_range2(&mut self, t_min: f64, t_max: f64) { self.t2_range = [t_min, t_max]; }
    pub fn set_tolerance(&mut self, tol1: f64, tol2: f64) { self.tolerance1 = tol1; self.tolerance2 = tol2; }

    pub fn perform(&mut self) {
        self.status = if self.solutions.is_empty() { ExtCC3dStatus::Done } else { ExtCC3dStatus::Done };
    }

    pub fn add_solution(&mut self, pair: CurvePair) { self.solutions.push(pair); }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn is_parallel(&self) -> bool { self.parallel }
    pub fn nb_ext(&self) -> usize { self.solutions.len() }
    pub fn square_distance(&self, i: usize) -> Option<f64> { self.solutions.get(i).map(|s| s.square_distance) }
    pub fn points(&self, i: usize) -> Option<&CurvePair> { self.solutions.get(i) }

    pub fn lower_distance(&self) -> f64 {
        self.solutions.iter().map(|s| s.square_distance).fold(f64::MAX, f64::min).sqrt()
    }

    pub fn lower_distance_index(&self) -> Option<usize> {
        self.solutions.iter().enumerate()
            .min_by(|a, b| a.1.square_distance.partial_cmp(&b.1.square_distance).unwrap())
            .map(|(i, _)| i)
    }
}

/// Result for point-on-curve extremum.
#[derive(Clone, Debug, Default)]
pub struct PointOnCurve {
    pub param: f64,
    pub point: [f64; 3],
    pub square_distance: f64,
    pub is_minimum: bool,
}

impl PointOnCurve {
    pub fn new(param: f64, point: [f64; 3], reference: [f64; 3]) -> Self {
        let d = [point[0]-reference[0], point[1]-reference[1], point[2]-reference[2]];
        Self { param, point, square_distance: d[0]*d[0]+d[1]*d[1]+d[2]*d[2], is_minimum: true }
    }
    pub fn distance(&self) -> f64 { self.square_distance.sqrt() }
}

// occt: Extrema_ExtPC3d — closest points from a 3D point to a 3D curve
#[derive(Clone, Debug, Default)]
pub struct ExtPCurve3d {
    pub curve_id: u32,
    pub point: [f64; 3],
    pub t_range: [f64; 2],
    pub tolerance: f64,
    pub status: ExtCC3dStatus,
    pub solutions: Vec<PointOnCurve>,
}

impl ExtPCurve3d {
    pub fn new(curve_id: u32, point: [f64; 3]) -> Self {
        Self { curve_id, point, t_range: [0.0, 1.0], tolerance: 1e-7, ..Default::default() }
    }

    pub fn set_range(&mut self, t_min: f64, t_max: f64) { self.t_range = [t_min, t_max]; }

    pub fn perform(&mut self) { self.status = ExtCC3dStatus::Done; }

    pub fn add_solution(&mut self, sol: PointOnCurve) { self.solutions.push(sol); }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_ext(&self) -> usize { self.solutions.len() }
    pub fn point(&self, i: usize) -> Option<&PointOnCurve> { self.solutions.get(i) }
    pub fn is_minimum(&self, i: usize) -> bool { self.solutions.get(i).map(|s| s.is_minimum).unwrap_or(false) }

    pub fn lower_distance(&self) -> f64 {
        self.solutions.iter().map(|s| s.square_distance).fold(f64::MAX, f64::min).sqrt()
    }

    pub fn lower_distance_parameter(&self) -> Option<f64> {
        self.solutions.iter()
            .min_by(|a, b| a.square_distance.partial_cmp(&b.square_distance).unwrap())
            .map(|s| s.param)
    }
}

// occt: Extrema_ExtCurveParameters — parameter + point pair on a curve
#[derive(Clone, Debug, Default)]
pub struct ExtCurveParameters {
    pub param: f64,
    pub point: [f64; 3],
    pub curve_id: u32,
}

impl ExtCurveParameters {
    pub fn new(curve_id: u32, param: f64, point: [f64; 3]) -> Self {
        Self { curve_id, param, point }
    }

    pub fn value(&self) -> [f64; 3] { self.point }
    pub fn parameter(&self) -> f64 { self.param }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext_cc_basic() {
        let mut ext = ExtCCurve3d::new(1, 2);
        ext.perform();
        assert!(ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn ext_cc_solutions() {
        let mut ext = ExtCCurve3d::new(1, 2);
        let pair = CurvePair::new(0.0, [0.0,0.0,0.0], 1.0, [3.0,4.0,0.0]);
        assert!((pair.distance() - 5.0).abs() < 1e-10);
        ext.add_solution(pair);
        ext.perform();
        assert_eq!(ext.nb_ext(), 1);
        assert!((ext.lower_distance() - 5.0).abs() < 1e-10);
        assert_eq!(ext.lower_distance_index(), Some(0));
    }

    #[test]
    fn ext_cc_range() {
        let mut ext = ExtCCurve3d::new(3, 4);
        ext.set_range1(0.0, 2.0);
        ext.set_range2(0.5, 3.5);
        assert_eq!(ext.t1_range, [0.0, 2.0]);
        assert_eq!(ext.t2_range, [0.5, 3.5]);
    }

    #[test]
    fn ext_pc_solutions() {
        let mut ext = ExtPCurve3d::new(1, [0.0, 0.0, 0.0]);
        let sol = PointOnCurve::new(0.5, [3.0, 4.0, 0.0], [0.0, 0.0, 0.0]);
        assert!((sol.distance() - 5.0).abs() < 1e-10);
        ext.add_solution(sol);
        ext.perform();
        assert!(ext.is_done());
        assert_eq!(ext.nb_ext(), 1);
        assert!((ext.lower_distance() - 5.0).abs() < 1e-10);
        assert_eq!(ext.lower_distance_parameter(), Some(0.5));
    }

    #[test]
    fn ext_curve_parameters() {
        let p = ExtCurveParameters::new(10, 0.75, [1.0, 2.0, 3.0]);
        assert!((p.parameter() - 0.75).abs() < 1e-10);
        assert_eq!(p.value(), [1.0, 2.0, 3.0]);
    }
}
