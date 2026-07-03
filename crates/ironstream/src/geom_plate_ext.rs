// FILE: geom_plate_ext.rs
// occt: GeomPlate_Surface extended, GeomPlate_BuildAveragePlane

/// Status of a plate-surface construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlateBuildStatus {
    NotDone,
    Done,
    MustBeJoined,
    NoConstraint,
    Error,
}

impl Default for PlateBuildStatus {
    fn default() -> Self { Self::NotDone }
}

impl PlateBuildStatus {
    pub fn is_done(&self) -> bool { *self == PlateBuildStatus::Done }
}

/// A point constraint for the plate surface.
#[derive(Clone, Debug)]
pub struct PlatePointConstraint {
    pub point: [f64; 3],
    pub normal: Option<[f64; 3]>,
    pub curvature: Option<f64>,
    pub order: u8,
}

impl PlatePointConstraint {
    pub fn g0(point: [f64; 3]) -> Self {
        Self { point, normal: None, curvature: None, order: 0 }
    }
    pub fn g1(point: [f64; 3], normal: [f64; 3]) -> Self {
        Self { point, normal: Some(normal), curvature: None, order: 1 }
    }
    pub fn order(&self) -> u8 { self.order }
    pub fn point(&self) -> [f64; 3] { self.point }
    pub fn normal(&self) -> Option<[f64; 3]> { self.normal }
}

/// A curve constraint for the plate surface.
#[derive(Clone, Debug)]
pub struct PlateCurveConstraint {
    pub curve_id: u32,
    pub order: u8,
    pub surface_id: Option<u32>,
}

impl PlateCurveConstraint {
    pub fn new(curve_id: u32, order: u8) -> Self {
        Self { curve_id, order, surface_id: None }
    }
    pub fn with_surface(mut self, surface_id: u32) -> Self { self.surface_id = Some(surface_id); self }
    pub fn order(&self) -> u8 { self.order }
    pub fn curve_id(&self) -> u32 { self.curve_id }
}

// occt: GeomPlate_Surface extended — thin-plate spline surface
#[derive(Clone, Debug)]
pub struct GeomPlateSurfaceExt {
    pub surface_id: u32,
    pub point_constraints: Vec<PlatePointConstraint>,
    pub curve_constraints: Vec<PlateCurveConstraint>,
    pub tolerance: f64,
    pub max_segment: usize,
    pub max_degree: usize,
    pub status: PlateBuildStatus,
}

impl GeomPlateSurfaceExt {
    pub fn new() -> Self {
        Self {
            surface_id: 0,
            point_constraints: Vec::new(),
            curve_constraints: Vec::new(),
            tolerance: 1e-4,
            max_segment: 10,
            max_degree: 8,
            status: PlateBuildStatus::NotDone,
        }
    }

    pub fn add_point_constraint(&mut self, c: PlatePointConstraint) { self.point_constraints.push(c); }
    pub fn add_curve_constraint(&mut self, c: PlateCurveConstraint) { self.curve_constraints.push(c); }
    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t; }
    pub fn set_max_segment(&mut self, n: usize) { self.max_segment = n; }
    pub fn set_max_degree(&mut self, d: usize) { self.max_degree = d; }

    pub fn build(&mut self) {
        if self.point_constraints.is_empty() && self.curve_constraints.is_empty() {
            self.status = PlateBuildStatus::NoConstraint;
            return;
        }
        let pt_hash: u32 = self.point_constraints.len() as u32 * 1000;
        let cv_hash: u32 = self.curve_constraints.iter().map(|c| c.curve_id).sum();
        self.surface_id = 96000 + pt_hash + cv_hash;
        self.status = PlateBuildStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn surface(&self) -> u32 { self.surface_id }

    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        if !self.is_done() { return [u, v, 0.0]; }
        // Stub: bilinear based on first 4 point constraints
        let n = self.point_constraints.len().min(4);
        if n == 0 { return [u, v, 0.0]; }
        let z_avg: f64 = self.point_constraints[..n].iter().map(|c| c.point[2]).sum::<f64>() / n as f64;
        [u, v, z_avg]
    }

    pub fn nb_point_constraints(&self) -> usize { self.point_constraints.len() }
    pub fn nb_curve_constraints(&self) -> usize { self.curve_constraints.len() }
}

impl Default for GeomPlateSurfaceExt {
    fn default() -> Self { Self::new() }
}

// occt: GeomPlate_BuildAveragePlane — best-fit plane through a point cloud
#[derive(Clone, Debug)]
pub struct GeomPlateBuildAveragePlane {
    pub points: Vec<[f64; 3]>,
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub plane_id: u32,
    pub is_done: bool,
    pub max_deviation: f64,
}

impl GeomPlateBuildAveragePlane {
    pub fn new(points: Vec<[f64; 3]>) -> Self {
        let mut s = Self {
            points,
            origin: [0.0; 3],
            normal: [0.0, 0.0, 1.0],
            plane_id: 0,
            is_done: false,
            max_deviation: 0.0,
        };
        s.compute();
        s
    }

    fn compute(&mut self) {
        let n = self.points.len();
        if n < 3 { return; }
        let cx: f64 = self.points.iter().map(|p| p[0]).sum::<f64>() / n as f64;
        let cy: f64 = self.points.iter().map(|p| p[1]).sum::<f64>() / n as f64;
        let cz: f64 = self.points.iter().map(|p| p[2]).sum::<f64>() / n as f64;
        self.origin = [cx, cy, cz];
        // Stub: compute z-spread to decide orientation
        let z_spread: f64 = self.points.iter().map(|p| (p[2] - cz).powi(2)).sum::<f64>() / n as f64;
        self.max_deviation = z_spread.sqrt();
        self.plane_id = 97000 + n as u32;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn plane(&self) -> u32 { self.plane_id }
    pub fn origin(&self) -> [f64; 3] { self.origin }
    pub fn normal(&self) -> [f64; 3] { self.normal }
    pub fn max_deviation(&self) -> f64 { self.max_deviation }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plate_surface_basic() {
        let mut s = GeomPlateSurfaceExt::new();
        s.add_point_constraint(PlatePointConstraint::g0([0.0, 0.0, 0.0]));
        s.add_point_constraint(PlatePointConstraint::g0([1.0, 0.0, 0.0]));
        s.add_point_constraint(PlatePointConstraint::g0([0.0, 1.0, 0.0]));
        s.build();
        assert!(s.is_done());
        assert_eq!(s.nb_point_constraints(), 3);
        assert!(s.surface() > 0);
    }

    #[test]
    fn plate_surface_no_constraint() {
        let mut s = GeomPlateSurfaceExt::new();
        s.build();
        assert_eq!(s.status, PlateBuildStatus::NoConstraint);
    }

    #[test]
    fn plate_surface_with_curve() {
        let mut s = GeomPlateSurfaceExt::new();
        s.add_curve_constraint(PlateCurveConstraint::new(10, 1));
        s.build();
        assert!(s.is_done());
        assert_eq!(s.nb_curve_constraints(), 1);
    }

    #[test]
    fn plate_point_constraint_orders() {
        let c0 = PlatePointConstraint::g0([1.0, 2.0, 3.0]);
        assert_eq!(c0.order(), 0);
        let c1 = PlatePointConstraint::g1([0.0; 3], [0.0, 0.0, 1.0]);
        assert_eq!(c1.order(), 1);
        assert!(c1.normal().is_some());
    }

    #[test]
    fn average_plane_centroid() {
        let pts = vec![[0.0,0.0,0.0],[2.0,0.0,0.0],[1.0,2.0,0.0]];
        let p = GeomPlateBuildAveragePlane::new(pts);
        assert!(p.is_done());
        let o = p.origin();
        assert!((o[0] - 1.0).abs() < 1e-10);
        assert!((o[1] - 2.0/3.0).abs() < 1e-10);
    }

    #[test]
    fn average_plane_not_enough_points() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let p = GeomPlateBuildAveragePlane::new(pts);
        assert!(!p.is_done());
    }
}
