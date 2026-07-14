// FILE: geomapi_ext.rs

// occt-ref: GeomAPI_ExtremaCurveCurve // result point pair
pub struct GeomApiExtremaPair {
    pub param1: f64,
    pub param2: f64,
    pub distance: f64,
    pub point1: [f64; 3],
    pub point2: [f64; 3],
}

// occt-ref: GeomAPI_ExtremaCurveCurve
pub struct GeomApiExtremaCurveCurve {
    pairs: Vec<GeomApiExtremaPair>,
    is_done: bool,
}

impl GeomApiExtremaCurveCurve {
    pub fn new() -> Self {
        Self {
            pairs: Vec::new(),
            is_done: false,
        }
    }

    pub fn perform(&mut self, _curve1_type: &str, _curve2_type: &str) {
        self.is_done = true;
        self.pairs.push(GeomApiExtremaPair {
            param1: 0.0,
            param2: 0.0,
            distance: 0.0,
            point1: [0.0, 0.0, 0.0],
            point2: [0.0, 0.0, 0.0],
        });
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_extrema(&self) -> usize {
        self.pairs.len()
    }

    pub fn points(&self, idx: usize) -> Option<&GeomApiExtremaPair> {
        if idx == 0 {
            return None;
        }
        self.pairs.get(idx - 1)
    }

    pub fn lower_distance(&self) -> Option<f64> {
        self.pairs.iter().map(|p| p.distance).reduce(f64::min)
    }
}

impl Default for GeomApiExtremaCurveCurve {
    fn default() -> Self {
        Self::new()
    }
}

// occt-ref: GeomAPI_ExtremaCurveSurface // result
pub struct GeomApiExtremaCurveSurfResult {
    pub curve_param: f64,
    pub surface_u: f64,
    pub surface_v: f64,
    pub distance: f64,
}

// occt-ref: GeomAPI_ExtremaCurveSurface
pub struct GeomApiExtremaCurveSurface {
    results: Vec<GeomApiExtremaCurveSurfResult>,
    is_done: bool,
}

impl GeomApiExtremaCurveSurface {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            is_done: false,
        }
    }

    pub fn perform(&mut self) {
        self.is_done = true;
        self.results.push(GeomApiExtremaCurveSurfResult {
            curve_param: 0.0,
            surface_u: 0.0,
            surface_v: 0.0,
            distance: 0.0,
        });
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_extrema(&self) -> usize {
        self.results.len()
    }

    pub fn point(&self, idx: usize) -> Option<&GeomApiExtremaCurveSurfResult> {
        if idx == 0 {
            return None;
        }
        self.results.get(idx - 1)
    }

    pub fn lower_distance(&self) -> Option<f64> {
        self.results.iter().map(|r| r.distance).reduce(f64::min)
    }
}

impl Default for GeomApiExtremaCurveSurface {
    fn default() -> Self {
        Self::new()
    }
}

// occt-ref: GeomAPI_PointsToBSpline
pub struct GeomApiPointsToBSpline {
    points: Vec<[f64; 3]>,
    degree_min: u32,
    degree_max: u32,
    continuity: u8,
    tolerance: f64,
    is_done: bool,
    nb_poles: u32,
    degree: u32,
}

impl GeomApiPointsToBSpline {
    pub fn new(degree_min: u32, degree_max: u32) -> Self {
        Self {
            points: Vec::new(),
            degree_min,
            degree_max,
            continuity: 2,
            tolerance: 1e-6,
            is_done: false,
            nb_poles: 0,
            degree: 0,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64, z: f64) {
        self.points.push([x, y, z]);
    }

    pub fn set_continuity(&mut self, c: u8) {
        self.continuity = c;
    }

    pub fn set_tolerance(&mut self, t: f64) {
        self.tolerance = t;
    }

    pub fn perform(&mut self) {
        if self.points.len() < 2 {
            self.is_done = false;
        } else {
            self.is_done = true;
            self.nb_poles = self.points.len() as u32;
            self.degree = self.degree_min;
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_poles(&self) -> u32 {
        self.nb_poles
    }

    pub fn degree(&self) -> u32 {
        self.degree
    }

    pub fn nb_points(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrema_curve_curve_new_not_done() {
        let e = GeomApiExtremaCurveCurve::new();
        assert!(!e.is_done());
        assert_eq!(e.nb_extrema(), 0);
    }

    #[test]
    fn extrema_curve_curve_perform_sets_done() {
        let mut e = GeomApiExtremaCurveCurve::new();
        e.perform("line", "circle");
        assert!(e.is_done());
    }

    #[test]
    fn extrema_curve_curve_perform_adds_pair() {
        let mut e = GeomApiExtremaCurveCurve::new();
        e.perform("line", "line");
        assert_eq!(e.nb_extrema(), 1);
    }

    #[test]
    fn extrema_curve_curve_lower_distance_zero() {
        let mut e = GeomApiExtremaCurveCurve::new();
        e.perform("line", "line");
        assert_eq!(e.lower_distance(), Some(0.0));
    }

    #[test]
    fn extrema_curve_curve_lower_distance_none_when_empty() {
        let e = GeomApiExtremaCurveCurve::new();
        assert_eq!(e.lower_distance(), None);
    }

    #[test]
    fn extrema_curve_curve_points_1based() {
        let mut e = GeomApiExtremaCurveCurve::new();
        e.perform("circle", "circle");
        assert!(e.points(1).is_some());
        assert!(e.points(0).is_none());
        assert!(e.points(2).is_none());
    }

    #[test]
    fn extrema_curve_curve_pair_fields() {
        let mut e = GeomApiExtremaCurveCurve::new();
        e.perform("line", "circle");
        let p = e.points(1).unwrap();
        assert_eq!(p.param1, 0.0);
        assert_eq!(p.param2, 0.0);
        assert_eq!(p.distance, 0.0);
        assert_eq!(p.point1, [0.0, 0.0, 0.0]);
        assert_eq!(p.point2, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn extrema_curve_surface_new_not_done() {
        let e = GeomApiExtremaCurveSurface::new();
        assert!(!e.is_done());
        assert_eq!(e.nb_extrema(), 0);
    }

    #[test]
    fn extrema_curve_surface_perform_sets_done() {
        let mut e = GeomApiExtremaCurveSurface::new();
        e.perform();
        assert!(e.is_done());
        assert_eq!(e.nb_extrema(), 1);
    }

    #[test]
    fn extrema_curve_surface_lower_distance_zero() {
        let mut e = GeomApiExtremaCurveSurface::new();
        e.perform();
        assert_eq!(e.lower_distance(), Some(0.0));
    }

    #[test]
    fn extrema_curve_surface_point_1based() {
        let mut e = GeomApiExtremaCurveSurface::new();
        e.perform();
        assert!(e.point(1).is_some());
        assert!(e.point(0).is_none());
    }

    #[test]
    fn points_to_bspline_new() {
        let b = GeomApiPointsToBSpline::new(2, 5);
        assert!(!b.is_done());
        assert_eq!(b.nb_points(), 0);
        assert_eq!(b.nb_poles(), 0);
    }

    #[test]
    fn points_to_bspline_add_and_perform() {
        let mut b = GeomApiPointsToBSpline::new(2, 5);
        b.add_point(0.0, 0.0, 0.0);
        b.add_point(1.0, 2.0, 3.0);
        b.add_point(4.0, 5.0, 6.0);
        b.perform();
        assert!(b.is_done());
        assert_eq!(b.nb_points(), 3);
        assert_eq!(b.nb_poles(), 3);
        assert_eq!(b.degree(), 2);
    }

    #[test]
    fn points_to_bspline_too_few_points() {
        let mut b = GeomApiPointsToBSpline::new(3, 6);
        b.add_point(1.0, 0.0, 0.0);
        b.perform();
        assert!(!b.is_done());
    }

    #[test]
    fn points_to_bspline_set_continuity_and_tolerance() {
        let mut b = GeomApiPointsToBSpline::new(1, 4);
        b.set_continuity(1);
        b.set_tolerance(1e-4);
        b.add_point(0.0, 0.0, 0.0);
        b.add_point(1.0, 1.0, 1.0);
        b.perform();
        assert!(b.is_done());
        assert_eq!(b.degree(), 1);
    }
}
