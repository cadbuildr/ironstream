// FILE: geom2d_project.rs
// occt: Geom2dAPI_ExtremaCurveCurve
// occt-ref: Geom2dAPI_ProjectPointOnCurve
//       Geom2dAPI_InterCurveCurve (additional variants)

/// Result of projecting a 2D point onto a 2D curve.
// occt-ref: Geom2dAPI_ProjectPointOnCurve
#[derive(Clone, Debug)]
pub struct Geom2dProjectPointOnCurve {
    pub point: [f64; 2],
    pub curve_id: u32,
    pub u_min: f64,
    pub u_max: f64,
    pub projections: Vec<Geom2dProjection>,
}

/// A single projection result: parameter + projected point.
#[derive(Clone, Copy, Debug)]
pub struct Geom2dProjection {
    pub parameter: f64,
    pub point: [f64; 2],
    pub distance: f64,
}

impl Geom2dProjectPointOnCurve {
    pub fn new(point: [f64; 2], curve_id: u32, u_min: f64, u_max: f64) -> Self {
        Self { point, curve_id, u_min, u_max, projections: Vec::new() }
    }

    /// Stub: project onto a line from (0,0) to (u_max,0) — nearest point on axis.
    pub fn perform(&mut self) {
        let ux = self.point[0].clamp(self.u_min, self.u_max);
        let proj = [ux, 0.0];
        let d = ((self.point[0] - proj[0]).powi(2) + (self.point[1] - proj[1]).powi(2)).sqrt();
        self.projections.clear();
        self.projections.push(Geom2dProjection { parameter: ux, point: proj, distance: d });
    }

    pub fn nb_points(&self) -> usize { self.projections.len() }

    pub fn nearest(&self) -> Option<&Geom2dProjection> {
        self.projections.iter().min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }

    pub fn lower_distance_parameter(&self) -> Option<f64> {
        self.nearest().map(|p| p.parameter)
    }

    pub fn lower_distance(&self) -> Option<f64> {
        self.nearest().map(|p| p.distance)
    }

    pub fn nearest_point(&self) -> Option<[f64; 2]> {
        self.nearest().map(|p| p.point)
    }
}

/// Extrema between two 2D curves.
/// occt: Geom2dAPI_ExtremaCurveCurve
#[derive(Clone, Debug)]
pub struct Geom2dExtremaCurveCurve {
    pub curve1_id: u32,
    pub curve2_id: u32,
    pub extrema: Vec<Geom2dExtrema>,
}

/// One extremal pair: parameter on each curve and distance.
#[derive(Clone, Copy, Debug)]
pub struct Geom2dExtrema {
    pub u1: f64,
    pub u2: f64,
    pub point1: [f64; 2],
    pub point2: [f64; 2],
    pub distance: f64,
}

impl Geom2dExtremaCurveCurve {
    pub fn new(curve1_id: u32, curve2_id: u32) -> Self {
        Self { curve1_id, curve2_id, extrema: Vec::new() }
    }

    /// Stub: emit one extremum at u=0 for both curves with distance = |c1-c2|.
    pub fn perform(&mut self, c1_origin: [f64; 2], c2_origin: [f64; 2]) {
        let d = ((c1_origin[0]-c2_origin[0]).powi(2) + (c1_origin[1]-c2_origin[1]).powi(2)).sqrt();
        self.extrema.push(Geom2dExtrema {
            u1: 0.0, u2: 0.0,
            point1: c1_origin,
            point2: c2_origin,
            distance: d,
        });
    }

    pub fn nb_extrema(&self) -> usize { self.extrema.len() }

    pub fn lower_distance(&self) -> Option<f64> {
        self.extrema.iter().map(|e| e.distance).reduce(f64::min)
    }

    pub fn is_done(&self) -> bool { !self.extrema.is_empty() }
}

/// 2D curve-curve intersection results.
// occt-ref: Geom2dAPI_InterCurveCurve // (extended variant)
#[derive(Clone, Debug)]
pub struct Geom2dInterCurveCurve {
    pub curve1_id: u32,
    pub curve2_id: u32,
    pub tolerance: f64,
    pub intersections: Vec<Geom2dIntersection>,
}

#[derive(Clone, Copy, Debug)]
pub struct Geom2dIntersection {
    pub u1: f64,
    pub u2: f64,
    pub point: [f64; 2],
    pub is_point: bool,   // false = segment overlap
}

impl Geom2dInterCurveCurve {
    pub fn new(curve1_id: u32, curve2_id: u32, tolerance: f64) -> Self {
        Self { curve1_id, curve2_id, tolerance: tolerance.max(1e-14), intersections: Vec::new() }
    }

    /// Stub: add a known intersection.
    pub fn add_intersection(&mut self, u1: f64, u2: f64, point: [f64; 2]) {
        self.intersections.push(Geom2dIntersection { u1, u2, point, is_point: true });
    }

    pub fn nb_points(&self) -> usize { self.intersections.iter().filter(|i| i.is_point).count() }
    pub fn nb_segments(&self) -> usize { self.intersections.iter().filter(|i| !i.is_point).count() }
    pub fn is_done(&self) -> bool { !self.intersections.is_empty() }

    pub fn point(&self, idx: usize) -> Option<[f64; 2]> {
        self.intersections.get(idx).map(|i| i.point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_point_on_curve() {
        let mut p = Geom2dProjectPointOnCurve::new([3.0, 4.0], 1, 0.0, 10.0);
        p.perform();
        assert_eq!(p.nb_points(), 1);
        let d = p.lower_distance().unwrap();
        assert!((d - 4.0).abs() < 1e-10); // perpendicular from (3,4) to x-axis = 4
    }

    #[test]
    fn project_nearest_point() {
        let mut p = Geom2dProjectPointOnCurve::new([5.0, 0.0], 1, 0.0, 10.0);
        p.perform();
        let np = p.nearest_point().unwrap();
        assert!((np[0] - 5.0).abs() < 1e-10);
        assert!((np[1]).abs() < 1e-10);
    }

    #[test]
    fn extrema_curve_curve() {
        let mut e = Geom2dExtremaCurveCurve::new(1, 2);
        e.perform([0.0, 0.0], [3.0, 4.0]);
        assert!(e.is_done());
        assert_eq!(e.nb_extrema(), 1);
        let d = e.lower_distance().unwrap();
        assert!((d - 5.0).abs() < 1e-10);
    }

    #[test]
    fn inter_curve_curve_add() {
        let mut ic = Geom2dInterCurveCurve::new(1, 2, 1e-6);
        assert!(!ic.is_done());
        ic.add_intersection(0.5, 0.5, [1.0, 1.0]);
        assert!(ic.is_done());
        assert_eq!(ic.nb_points(), 1);
        assert_eq!(ic.point(0), Some([1.0, 1.0]));
    }

    #[test]
    fn project_clamp_to_range() {
        let mut p = Geom2dProjectPointOnCurve::new([-5.0, 0.0], 1, 0.0, 10.0);
        p.perform();
        let param = p.lower_distance_parameter().unwrap();
        assert!((param - 0.0).abs() < 1e-10); // clamped to u_min
    }
}
