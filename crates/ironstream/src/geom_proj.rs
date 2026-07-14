// FILE: geom_proj.rs
// occt-ref: GeomAPI_ProjectPointOnCurve, GeomAPI_ProjectPointOnSurface

/// Status of a projection computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjStatus {
    NotDone,
    Done,
    NullGeometry,
    NoSolution,
}

impl Default for ProjStatus {
    fn default() -> Self { Self::NotDone }
}

impl ProjStatus {
    pub fn is_done(&self) -> bool { *self == ProjStatus::Done }
}

// occt-ref: GeomAPI_ProjectPointOnCurve // — projects a 3D point onto a curve
#[derive(Clone, Debug)]
pub struct ProjectPointOnCurve {
    pub point: [f64; 3],
    pub curve_id: u32,
    pub first_param: f64,
    pub last_param: f64,
    pub extrema_params: Vec<f64>,
    pub extrema_distances: Vec<f64>,
    pub status: ProjStatus,
}

impl ProjectPointOnCurve {
    pub fn new(point: [f64; 3], curve_id: u32) -> Self {
        Self {
            point,
            curve_id,
            first_param: 0.0,
            last_param: 1.0,
            extrema_params: Vec::new(),
            extrema_distances: Vec::new(),
            status: ProjStatus::NotDone,
        }
    }

    pub fn with_bounds(mut self, first: f64, last: f64) -> Self {
        self.first_param = first;
        self.last_param = last;
        self
    }

    pub fn perform(&mut self) {
        if self.curve_id == 0 {
            self.status = ProjStatus::NullGeometry;
            return;
        }
        // Stub: project onto a unit-line [0,1] in x
        let t = self.point[0].clamp(self.first_param, self.last_param);
        let dist = ((self.point[0] - t).powi(2) + self.point[1].powi(2) + self.point[2].powi(2)).sqrt();
        self.extrema_params = vec![t];
        self.extrema_distances = vec![dist];
        self.status = ProjStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_points(&self) -> usize { self.extrema_params.len() }
    pub fn nearest_point(&self) -> Option<f64> { self.extrema_params.first().copied() }
    pub fn lower_distance_parameter(&self) -> Option<f64> { self.nearest_point() }
    pub fn lower_distance(&self) -> Option<f64> { self.extrema_distances.first().copied() }
    pub fn parameter(&self, i: usize) -> Option<f64> { self.extrema_params.get(i).copied() }
    pub fn distance(&self, i: usize) -> Option<f64> { self.extrema_distances.get(i).copied() }
}

// occt-note: GeomAPI_ProjectPointOnSurface — projects a 3D point onto a surface
#[derive(Clone, Debug)]
pub struct ProjectPointOnSurface {
    pub point: [f64; 3],
    pub surface_id: u32,
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
    pub nearest_u: f64,
    pub nearest_v: f64,
    pub distance: f64,
    pub status: ProjStatus,
}

impl ProjectPointOnSurface {
    pub fn new(point: [f64; 3], surface_id: u32) -> Self {
        Self {
            point,
            surface_id,
            u_min: 0.0, u_max: 1.0,
            v_min: 0.0, v_max: 1.0,
            nearest_u: 0.0, nearest_v: 0.0,
            distance: 0.0,
            status: ProjStatus::NotDone,
        }
    }

    pub fn with_bounds(mut self, u1: f64, u2: f64, v1: f64, v2: f64) -> Self {
        self.u_min = u1; self.u_max = u2;
        self.v_min = v1; self.v_max = v2;
        self
    }

    pub fn perform(&mut self) {
        if self.surface_id == 0 {
            self.status = ProjStatus::NullGeometry;
            return;
        }
        // Stub: project onto XY plane (z=0)
        self.nearest_u = self.point[0].clamp(self.u_min, self.u_max);
        self.nearest_v = self.point[1].clamp(self.v_min, self.v_max);
        self.distance = self.point[2].abs();
        self.status = ProjStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_points(&self) -> usize { if self.is_done() { 1 } else { 0 } }
    pub fn lower_distance_parameters(&self) -> Option<(f64, f64)> {
        if self.is_done() { Some((self.nearest_u, self.nearest_v)) } else { None }
    }
    pub fn lower_distance(&self) -> Option<f64> { if self.is_done() { Some(self.distance) } else { None } }

    pub fn nearest_point(&self) -> Option<[f64; 3]> {
        if self.is_done() { Some([self.nearest_u, self.nearest_v, 0.0]) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_on_curve_basic() {
        let mut p = ProjectPointOnCurve::new([0.5, 0.0, 0.0], 1);
        p.perform();
        assert!(p.is_done());
        assert_eq!(p.nb_points(), 1);
        let param = p.nearest_point().unwrap();
        assert!((param - 0.5).abs() < 1e-10);
    }

    #[test]
    fn project_on_curve_off_axis() {
        let mut p = ProjectPointOnCurve::new([0.5, 1.0, 0.0], 1);
        p.perform();
        assert!(p.is_done());
        let dist = p.lower_distance().unwrap();
        assert!((dist - 1.0).abs() < 1e-10);
    }

    #[test]
    fn project_on_curve_null_geom() {
        let mut p = ProjectPointOnCurve::new([0.5, 0.0, 0.0], 0);
        p.perform();
        assert_eq!(p.status, ProjStatus::NullGeometry);
    }

    #[test]
    fn project_on_surface_basic() {
        let mut p = ProjectPointOnSurface::new([0.3, 0.7, 2.0], 1);
        p.perform();
        assert!(p.is_done());
        let (u, v) = p.lower_distance_parameters().unwrap();
        assert!((u - 0.3).abs() < 1e-10);
        assert!((v - 0.7).abs() < 1e-10);
        let dist = p.lower_distance().unwrap();
        assert!((dist - 2.0).abs() < 1e-10);
    }

    #[test]
    fn project_on_surface_null() {
        let mut p = ProjectPointOnSurface::new([0.0; 3], 0);
        p.perform();
        assert_eq!(p.status, ProjStatus::NullGeometry);
        assert!(p.nearest_point().is_none());
    }

    #[test]
    fn project_on_surface_with_bounds() {
        let mut p = ProjectPointOnSurface::new([5.0, 5.0, 1.0], 2)
            .with_bounds(0.0, 3.0, 0.0, 3.0);
        p.perform();
        assert!(p.is_done());
        let (u, v) = p.lower_distance_parameters().unwrap();
        assert!((u - 3.0).abs() < 1e-10); // clamped to u_max
        assert!((v - 3.0).abs() < 1e-10);
    }
}
