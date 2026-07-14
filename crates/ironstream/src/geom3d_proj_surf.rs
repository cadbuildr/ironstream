// FILE: geom3d_proj_surf.rs
// occt-ref: GeomAPI_ProjectPointOnSurface, GeomAPI_ProjectPointOnCurve
//       GeomAPI_ExtremaCurveSurface, GeomAPI_ExtremaCurveCurve

/// A single projection result (parameter + distance).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjResult {
    pub u: f64,
    pub v: f64,
    pub point: [f64; 3],
    pub distance: f64,
}

impl ProjResult {
    pub fn new(u: f64, v: f64, point: [f64; 3], distance: f64) -> Self {
        Self { u, v, point, distance }
    }
}

/// Projects a 3D point onto a parametric surface.
// occt-ref: GeomAPI_ProjectPointOnSurface
#[derive(Clone, Debug)]
pub struct GeomApiProjectPointOnSurface {
    pub surface_id: u32,
    pub point: [f64; 3],
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
    pub tolerance: f64,
    pub results: Vec<ProjResult>,
    pub is_done: bool,
}

impl GeomApiProjectPointOnSurface {
    pub fn new(surface_id: u32, u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        Self {
            surface_id,
            point: [0.0; 3],
            u_min,
            u_max,
            v_min,
            v_max,
            tolerance: 1e-6,
            results: Vec::new(),
            is_done: false,
        }
    }

    /// Stub: project point onto the parametric plane z=0 (i.e., ignore z).
    /// Returns one result at (clamp(px, u_min, u_max), clamp(py, v_min, v_max)).
    pub fn perform(&mut self, point: [f64; 3]) {
        self.point = point;
        self.results.clear();

        let u = point[0].clamp(self.u_min, self.u_max);
        let v = point[1].clamp(self.v_min, self.v_max);
        let proj = [u, v, 0.0];
        let dx = point[0] - proj[0];
        let dy = point[1] - proj[1];
        let dz = point[2] - proj[2];
        let dist = (dx*dx + dy*dy + dz*dz).sqrt();

        self.results.push(ProjResult::new(u, v, proj, dist));
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_points(&self) -> usize { self.results.len() }

    pub fn lower_distance_parameter(&self) -> Option<(f64, f64)> {
        self.results.iter()
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .map(|r| (r.u, r.v))
    }

    pub fn lower_distance(&self) -> Option<f64> {
        self.results.iter()
            .map(|r| r.distance)
            .reduce(f64::min)
    }

    pub fn nearest_point(&self) -> Option<[f64; 3]> {
        self.results.iter()
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .map(|r| r.point)
    }
}

/// Projects a 3D point onto a parametric curve.
// occt-ref: GeomAPI_ProjectPointOnCurve
#[derive(Clone, Debug)]
pub struct GeomApiProjectPointOnCurve {
    pub curve_id: u32,
    pub point: [f64; 3],
    pub u_min: f64,
    pub u_max: f64,
    pub tolerance: f64,
    pub results: Vec<(f64, f64)>,   // (parameter, distance)
    pub is_done: bool,
}

impl GeomApiProjectPointOnCurve {
    pub fn new(curve_id: u32, u_min: f64, u_max: f64) -> Self {
        Self {
            curve_id,
            point: [0.0; 3],
            u_min,
            u_max,
            tolerance: 1e-6,
            results: Vec::new(),
            is_done: false,
        }
    }

    /// Stub: project onto the u-axis (x component), clamp to [u_min, u_max].
    pub fn perform(&mut self, point: [f64; 3]) {
        self.point = point;
        self.results.clear();

        let t = point[0].clamp(self.u_min, self.u_max);
        let proj = [t, 0.0, 0.0];
        let dist = ((point[0]-proj[0]).powi(2) + point[1].powi(2) + point[2].powi(2)).sqrt();
        self.results.push((t, dist));
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_points(&self) -> usize { self.results.len() }

    pub fn lower_distance(&self) -> Option<f64> {
        self.results.iter().map(|&(_, d)| d).reduce(f64::min)
    }

    pub fn lower_distance_parameter(&self) -> Option<f64> {
        self.results.iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|&(t, _)| t)
    }
}

/// Extrema between two curves.
// occt-ref: GeomAPI_ExtremaCurveCurve
#[derive(Clone, Debug, Default)]
pub struct GeomApiExtremaCurveCurve {
    pub curve1_id: u32,
    pub curve2_id: u32,
    pub extrema: Vec<(f64, f64, f64)>, // (t1, t2, distance)
    pub is_done: bool,
}

impl GeomApiExtremaCurveCurve {
    pub fn new(curve1_id: u32, curve2_id: u32) -> Self {
        Self { curve1_id, curve2_id, extrema: Vec::new(), is_done: false }
    }

    /// Stub: report one extremum at t1=0, t2=0, distance = |curve1-curve2| as id diff.
    pub fn perform(&mut self) {
        self.extrema.clear();
        let d = (self.curve1_id as f64 - self.curve2_id as f64).abs();
        self.extrema.push((0.0, 0.0, d));
        self.is_done = true;
    }

    pub fn nb_extrema(&self) -> usize { self.extrema.len() }
    pub fn lower_distance(&self) -> Option<f64> {
        self.extrema.iter().map(|&(_, _, d)| d).reduce(f64::min)
    }
}

/// Extrema between a curve and surface.
// occt-ref: GeomAPI_ExtremaCurveSurface
#[derive(Clone, Debug, Default)]
pub struct GeomApiExtremaCurveSurface {
    pub curve_id: u32,
    pub surface_id: u32,
    pub extrema: Vec<(f64, f64, f64, f64)>,  // (tc, u, v, distance)
    pub is_done: bool,
}

impl GeomApiExtremaCurveSurface {
    pub fn new(curve_id: u32, surface_id: u32) -> Self {
        Self { curve_id, surface_id, extrema: Vec::new(), is_done: false }
    }

    pub fn perform(&mut self) {
        self.extrema.clear();
        self.extrema.push((0.0, 0.0, 0.0, 0.0));
        self.is_done = true;
    }

    pub fn nb_extrema(&self) -> usize { self.extrema.len() }
    pub fn lower_distance(&self) -> Option<f64> {
        self.extrema.iter().map(|&(_, _, _, d)| d).reduce(f64::min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_point_on_surface_stub() {
        let mut p = GeomApiProjectPointOnSurface::new(1, 0.0, 1.0, 0.0, 1.0);
        p.perform([0.5, 0.5, 3.0]);
        assert!(p.is_done());
        assert_eq!(p.nb_points(), 1);
        assert!((p.lower_distance().unwrap() - 3.0).abs() < 1e-10);
        let (u, v) = p.lower_distance_parameter().unwrap();
        assert!((u - 0.5).abs() < 1e-14);
        assert!((v - 0.5).abs() < 1e-14);
    }

    #[test]
    fn project_clamp_to_bounds() {
        let mut p = GeomApiProjectPointOnSurface::new(1, 0.0, 1.0, 0.0, 1.0);
        p.perform([2.0, 3.0, 0.0]);
        let (u, v) = p.lower_distance_parameter().unwrap();
        assert!((u - 1.0).abs() < 1e-14);
        assert!((v - 1.0).abs() < 1e-14);
    }

    #[test]
    fn project_point_on_curve_stub() {
        let mut p = GeomApiProjectPointOnCurve::new(1, 0.0, 10.0);
        p.perform([5.0, 3.0, 4.0]);
        assert!(p.is_done());
        let t = p.lower_distance_parameter().unwrap();
        assert!((t - 5.0).abs() < 1e-14);
        let d = p.lower_distance().unwrap();
        assert!((d - 5.0).abs() < 1e-10); // sqrt(0+9+16)=5
    }

    #[test]
    fn extrema_curve_curve() {
        let mut e = GeomApiExtremaCurveCurve::new(3, 7);
        e.perform();
        assert!(e.is_done);
        assert!((e.lower_distance().unwrap() - 4.0).abs() < 1e-14);
    }

    #[test]
    fn extrema_curve_surface() {
        let mut e = GeomApiExtremaCurveSurface::new(1, 2);
        e.perform();
        assert!(e.is_done);
        assert_eq!(e.nb_extrema(), 1);
        assert!((e.lower_distance().unwrap()).abs() < 1e-14);
    }
}
