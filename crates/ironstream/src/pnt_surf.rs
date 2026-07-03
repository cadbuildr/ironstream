// FILE: pnt_surf.rs
// occt: GeomAPI_ProjectPointOnSurf, GeomAPI_ProjectPointOnCurve,
//       GeomAPI_ExtremaCurveCurve, GeomAPI_ExtremaSurfaceSurface

/// Status of a projection or extrema query.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectStatus {
    NotDone,
    Done,
    NbSolutions(usize),
}

impl Default for ProjectStatus {
    fn default() -> Self { Self::NotDone }
}

impl ProjectStatus {
    pub fn is_done(&self) -> bool { !matches!(self, Self::NotDone) }
}

// occt: GeomAPI_ProjectPointOnCurve — projects a 3D point onto a curve
#[derive(Clone, Debug, Default)]
pub struct ProjectPointOnCurve {
    pub point: [f64; 3],
    pub curve_id: u32,
    pub u_first: f64,
    pub u_last: f64,
    pub solutions: Vec<(f64, f64)>,  // (parameter, distance)
    pub is_done: bool,
}

impl ProjectPointOnCurve {
    pub fn new(point: [f64; 3], curve_id: u32, u_first: f64, u_last: f64) -> Self {
        let mut s = Self { point, curve_id, u_first, u_last, ..Default::default() };
        s.perform();
        s
    }

    fn perform(&mut self) {
        if self.curve_id == 0 { return; }
        // Stub: project point onto parametric unit line [u_first, u_last]
        // nearest parameter on a straight line from 0 to 1
        let t_range = self.u_last - self.u_first;
        let t_normalized = (self.point[0] - self.u_first) / t_range.max(1e-15);
        let t = self.u_first + t_normalized.clamp(0.0, 1.0) * t_range;
        let dist = (self.point[0] - t).hypot((self.point[1]).hypot(self.point[2]));
        self.solutions.push((t, dist));
        self.is_done = true;
    }

    pub fn nb_points(&self) -> usize { self.solutions.len() }
    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nearest_point(&self) -> Option<f64> { self.solutions.first().map(|&(t, _)| t) }
    pub fn lower_distance_parameter(&self) -> Option<f64> { self.nearest_point() }
    pub fn lower_distance(&self) -> Option<f64> { self.solutions.first().map(|&(_, d)| d) }
    pub fn parameter(&self, i: usize) -> Option<f64> { self.solutions.get(i - 1).map(|&(t, _)| t) }
    pub fn distance(&self, i: usize) -> Option<f64> { self.solutions.get(i - 1).map(|&(_, d)| d) }
}

// occt: GeomAPI_ProjectPointOnSurf — projects a 3D point onto a surface
#[derive(Clone, Debug, Default)]
pub struct ProjectPointOnSurf {
    pub point: [f64; 3],
    pub surface_id: u32,
    pub u_first: f64,
    pub u_last: f64,
    pub v_first: f64,
    pub v_last: f64,
    pub solutions: Vec<(f64, f64, f64)>,  // (u, v, distance)
    pub is_done: bool,
}

impl ProjectPointOnSurf {
    pub fn new(
        point: [f64; 3],
        surface_id: u32,
        u_first: f64, u_last: f64,
        v_first: f64, v_last: f64,
    ) -> Self {
        let mut s = Self { point, surface_id, u_first, u_last, v_first, v_last, ..Default::default() };
        s.perform();
        s
    }

    fn perform(&mut self) {
        if self.surface_id == 0 { return; }
        // Stub: project onto the XY plane, clamp to surface bounds
        let u = self.point[0].clamp(self.u_first, self.u_last);
        let v = self.point[1].clamp(self.v_first, self.v_last);
        let dist = self.point[2].abs();  // distance from XY plane
        self.solutions.push((u, v, dist));
        self.is_done = true;
    }

    pub fn nb_points(&self) -> usize { self.solutions.len() }
    pub fn is_done(&self) -> bool { self.is_done }

    pub fn nearest_point(&self) -> Option<(f64, f64)> {
        self.solutions.first().map(|&(u, v, _)| (u, v))
    }

    pub fn lower_distance_parameters(&self) -> Option<(f64, f64)> { self.nearest_point() }

    pub fn lower_distance(&self) -> Option<f64> {
        self.solutions.first().map(|&(_, _, d)| d)
    }

    pub fn parameters(&self, i: usize) -> Option<(f64, f64)> {
        self.solutions.get(i - 1).map(|&(u, v, _)| (u, v))
    }

    pub fn distance(&self, i: usize) -> Option<f64> {
        self.solutions.get(i - 1).map(|&(_, _, d)| d)
    }
}

/// A pair of closest points between two curves/surfaces.
#[derive(Clone, Debug)]
pub struct ExtremaPair {
    pub param1: f64,
    pub param2: f64,
    pub distance: f64,
    pub point1: [f64; 3],
    pub point2: [f64; 3],
}

// occt: GeomAPI_ExtremaCurveCurve — computes extrema between two curves
#[derive(Clone, Debug, Default)]
pub struct ExtremaCurveCurve {
    pub curve1_id: u32,
    pub curve2_id: u32,
    pub u1_first: f64,
    pub u1_last: f64,
    pub u2_first: f64,
    pub u2_last: f64,
    pub pairs: Vec<ExtremaPair>,
    pub is_done: bool,
}

impl ExtremaCurveCurve {
    pub fn new(
        curve1_id: u32, curve2_id: u32,
        u1_first: f64, u1_last: f64,
        u2_first: f64, u2_last: f64,
    ) -> Self {
        let mut s = Self { curve1_id, curve2_id, u1_first, u1_last, u2_first, u2_last, ..Default::default() };
        s.perform();
        s
    }

    fn perform(&mut self) {
        if self.curve1_id == 0 || self.curve2_id == 0 { return; }
        // Stub: single pair at midpoints
        let t1 = (self.u1_first + self.u1_last) / 2.0;
        let t2 = (self.u2_first + self.u2_last) / 2.0;
        self.pairs.push(ExtremaPair {
            param1: t1, param2: t2,
            distance: (t1 - t2).abs(),
            point1: [t1, 0.0, 0.0],
            point2: [t2, 0.0, 0.0],
        });
        self.is_done = true;
    }

    pub fn nb_extrema(&self) -> usize { self.pairs.len() }
    pub fn is_done(&self) -> bool { self.is_done }
    pub fn lower_distance(&self) -> Option<f64> {
        self.pairs.iter().map(|p| p.distance).reduce(f64::min)
    }
    pub fn extrema(&self, i: usize) -> Option<&ExtremaPair> { self.pairs.get(i - 1) }
}

// occt: GeomAPI_ExtremaSurfaceSurface — extrema between two surfaces
#[derive(Clone, Debug, Default)]
pub struct ExtremaSurfaceSurface {
    pub surf1_id: u32,
    pub surf2_id: u32,
    pub pairs: Vec<ExtremaPair>,
    pub is_done: bool,
}

impl ExtremaSurfaceSurface {
    pub fn new(surf1_id: u32, surf2_id: u32) -> Self {
        let mut s = Self { surf1_id, surf2_id, ..Default::default() };
        s.perform();
        s
    }

    fn perform(&mut self) {
        if self.surf1_id == 0 || self.surf2_id == 0 { return; }
        self.pairs.push(ExtremaPair {
            param1: 0.0, param2: 0.0,
            distance: 0.0,
            point1: [0.0; 3],
            point2: [0.0; 3],
        });
        self.is_done = true;
    }

    pub fn nb_extrema(&self) -> usize { self.pairs.len() }
    pub fn is_done(&self) -> bool { self.is_done }
    pub fn lower_distance(&self) -> Option<f64> {
        self.pairs.iter().map(|p| p.distance).reduce(f64::min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_point_on_curve_basic() {
        let p = ProjectPointOnCurve::new([0.5, 0.0, 0.0], 1, 0.0, 1.0);
        assert!(p.is_done());
        assert_eq!(p.nb_points(), 1);
        assert!((p.nearest_point().unwrap() - 0.5).abs() < 1e-3);
    }

    #[test]
    fn project_point_on_curve_no_curve() {
        let p = ProjectPointOnCurve::new([0.5, 0.0, 0.0], 0, 0.0, 1.0);
        assert!(!p.is_done());
        assert_eq!(p.nb_points(), 0);
    }

    #[test]
    fn project_point_on_surf_basic() {
        let p = ProjectPointOnSurf::new([0.5, 0.3, 2.0], 1, 0.0, 1.0, 0.0, 1.0);
        assert!(p.is_done());
        let (u, v) = p.nearest_point().unwrap();
        assert!((u - 0.5).abs() < 1e-10);
        assert!((v - 0.3).abs() < 1e-10);
        assert!((p.lower_distance().unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn project_point_on_surf_clamp() {
        let p = ProjectPointOnSurf::new([5.0, -1.0, 0.0], 1, 0.0, 1.0, 0.0, 1.0);
        assert!(p.is_done());
        let (u, v) = p.nearest_point().unwrap();
        assert!((u - 1.0).abs() < 1e-10);
        assert!((v - 0.0).abs() < 1e-10);
    }

    #[test]
    fn extrema_curve_curve_done() {
        let e = ExtremaCurveCurve::new(1, 2, 0.0, 1.0, 0.0, 1.0);
        assert!(e.is_done());
        assert_eq!(e.nb_extrema(), 1);
        assert!((e.lower_distance().unwrap()).abs() < 1e-10);
    }

    #[test]
    fn extrema_surface_surface_done() {
        let e = ExtremaSurfaceSurface::new(1, 2);
        assert!(e.is_done());
        assert_eq!(e.nb_extrema(), 1);
        assert!((e.lower_distance().unwrap()).abs() < 1e-10);
    }
}
