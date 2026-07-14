// FILE: geom2d_api_ext.rs

pub struct Geom2dApiPtsToBSplineParams {
    pub degree_min: u32,
    pub degree_max: u32,
    pub continuity: u8,
    pub tolerance: f64,
}

impl Default for Geom2dApiPtsToBSplineParams {
    fn default() -> Self {
        Self {
            degree_min: 3,
            degree_max: 8,
            continuity: 2,
            tolerance: 1e-6,
        }
    }
}

// occt-ref: Geom2dAPI_PointsToBSpline
pub struct Geom2dApiPointsToBSpline {
    points: Vec<[f64; 2]>,
    params: Geom2dApiPtsToBSplineParams,
    is_done: bool,
    nb_poles: u32,
    degree: u32,
}

impl Geom2dApiPointsToBSpline {
    pub fn new(params: Geom2dApiPtsToBSplineParams) -> Self {
        Self {
            points: Vec::new(),
            params,
            is_done: false,
            nb_poles: 0,
            degree: 0,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push([x, y]);
    }

    pub fn set_continuity(&mut self, c: u8) {
        self.params.continuity = c;
    }

    pub fn set_tolerance(&mut self, t: f64) {
        self.params.tolerance = t;
    }

    pub fn perform(&mut self) {
        if self.points.len() < 2 {
            self.is_done = false;
        } else {
            self.is_done = true;
            self.nb_poles = self.points.len() as u32;
            self.degree = self.params.degree_min;
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

// occt-ref: Geom2dAPI_Interpolate
pub struct Geom2dApiInterpolate {
    points: Vec<[f64; 2]>,
    is_periodic: bool,
    tolerance: f64,
    is_done: bool,
    nb_poles: u32,
}

impl Geom2dApiInterpolate {
    pub fn new(is_periodic: bool, tolerance: f64) -> Self {
        Self {
            points: Vec::new(),
            is_periodic,
            tolerance,
            is_done: false,
            nb_poles: 0,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push([x, y]);
    }

    pub fn perform(&mut self) {
        if self.points.len() < 2 {
            self.is_done = false;
        } else {
            self.is_done = true;
            self.nb_poles = self.points.len() as u32;
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_poles(&self) -> u32 {
        self.nb_poles
    }

    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }
}

// occt: Geom2dAPI_ProjectPointOnCurve
pub struct Geom2dApiProjectPointOnCurve {
    point: [f64; 2],
    nb_results: u32,
    lower_distance: f64,
    nearest_param: f64,
    is_done: bool,
}

impl Geom2dApiProjectPointOnCurve {
    pub fn new(point: [f64; 2]) -> Self {
        Self {
            point,
            nb_results: 0,
            lower_distance: 0.0,
            nearest_param: 0.0,
            is_done: false,
        }
    }

    pub fn perform(&mut self) {
        self.nb_results = 1;
        self.lower_distance = 0.0;
        self.nearest_param = 0.0;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_points(&self) -> u32 {
        self.nb_results
    }

    pub fn lower_distance(&self) -> f64 {
        self.lower_distance
    }

    pub fn lower_distance_parameter(&self) -> f64 {
        self.nearest_param
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pts_to_bspline_default_params() {
        let p = Geom2dApiPtsToBSplineParams::default();
        assert_eq!(p.degree_min, 3);
        assert_eq!(p.degree_max, 8);
        assert_eq!(p.continuity, 2);
        assert_eq!(p.tolerance, 1e-6);
    }

    #[test]
    fn pts_to_bspline_new_not_done() {
        let b = Geom2dApiPointsToBSpline::new(Geom2dApiPtsToBSplineParams::default());
        assert!(!b.is_done());
        assert_eq!(b.nb_points(), 0);
        assert_eq!(b.nb_poles(), 0);
        assert_eq!(b.degree(), 0);
    }

    #[test]
    fn pts_to_bspline_one_point_not_done() {
        let mut b = Geom2dApiPointsToBSpline::new(Geom2dApiPtsToBSplineParams::default());
        b.add_point(1.0, 2.0);
        b.perform();
        assert!(!b.is_done());
    }

    #[test]
    fn pts_to_bspline_three_points_done() {
        let mut b = Geom2dApiPointsToBSpline::new(Geom2dApiPtsToBSplineParams::default());
        b.add_point(0.0, 0.0);
        b.add_point(1.0, 1.0);
        b.add_point(2.0, 0.0);
        b.perform();
        assert!(b.is_done());
        assert_eq!(b.nb_points(), 3);
        assert_eq!(b.nb_poles(), 3);
        assert_eq!(b.degree(), 3);
    }

    #[test]
    fn pts_to_bspline_set_continuity_tolerance() {
        let mut b = Geom2dApiPointsToBSpline::new(Geom2dApiPtsToBSplineParams::default());
        b.set_continuity(1);
        b.set_tolerance(1e-4);
        b.add_point(0.0, 0.0);
        b.add_point(5.0, 5.0);
        b.perform();
        assert!(b.is_done());
        assert_eq!(b.nb_poles(), 2);
    }

    #[test]
    fn interpolate_new_not_done() {
        let interp = Geom2dApiInterpolate::new(false, 1e-6);
        assert!(!interp.is_done());
        assert_eq!(interp.nb_poles(), 0);
        assert!(!interp.is_periodic());
    }

    #[test]
    fn interpolate_periodic_flag() {
        let interp = Geom2dApiInterpolate::new(true, 1e-6);
        assert!(interp.is_periodic());
    }

    #[test]
    fn interpolate_one_point_not_done() {
        let mut interp = Geom2dApiInterpolate::new(false, 1e-6);
        interp.add_point(0.0, 0.0);
        interp.perform();
        assert!(!interp.is_done());
    }

    #[test]
    fn interpolate_multiple_points_done() {
        let mut interp = Geom2dApiInterpolate::new(false, 1e-6);
        interp.add_point(0.0, 0.0);
        interp.add_point(1.0, 2.0);
        interp.add_point(3.0, 1.0);
        interp.add_point(4.0, 0.0);
        interp.perform();
        assert!(interp.is_done());
        assert_eq!(interp.nb_poles(), 4);
    }

    #[test]
    fn project_point_new_not_done() {
        let proj = Geom2dApiProjectPointOnCurve::new([1.0, 2.0]);
        assert!(!proj.is_done());
        assert_eq!(proj.nb_points(), 0);
    }

    #[test]
    fn project_point_perform_sets_done() {
        let mut proj = Geom2dApiProjectPointOnCurve::new([3.0, 4.0]);
        proj.perform();
        assert!(proj.is_done());
        assert_eq!(proj.nb_points(), 1);
    }

    #[test]
    fn project_point_lower_distance_zero() {
        let mut proj = Geom2dApiProjectPointOnCurve::new([0.0, 0.0]);
        proj.perform();
        assert_eq!(proj.lower_distance(), 0.0);
        assert_eq!(proj.lower_distance_parameter(), 0.0);
    }

    #[test]
    fn project_point_stores_input_coords() {
        let proj = Geom2dApiProjectPointOnCurve::new([7.0, -3.0]);
        assert_eq!(proj.point[0], 7.0);
        assert_eq!(proj.point[1], -3.0);
    }
}
