// FILE: geom_project.rs
// occt-ref: GeomAPI_ProjectPointOnSurf, GeomAPI_ProjectPointOnCurve, ProjLib_ProjectedCurve

// occt-ref: GeomAPI_ProjectPointOnSurf
/// Projects a point onto a parametric surface, finding the nearest (U, V) parameter.
#[derive(Clone, Debug)]
pub struct ProjectPointOnSurf {
    pub point: [f64; 3],
    pub surface_id: usize,
    pub solutions: Vec<ProjSolution>,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct ProjSolution {
    pub u: f64,
    pub v: f64,
    pub distance: f64,
    pub point: [f64; 3],
}

impl ProjectPointOnSurf {
    pub fn new(point: [f64; 3], surface_id: usize) -> Self {
        let mut s = Self { point, surface_id, solutions: Vec::new(), done: false };
        s.perform();
        s
    }

    fn perform(&mut self) {
        // Stub: project onto flat Z=0 plane at surface origin
        let proj = [self.point[0], self.point[1], 0.0];
        let dx = self.point[0] - proj[0];
        let dy = self.point[1] - proj[1];
        let dz = self.point[2] - proj[2];
        let dist = (dx*dx + dy*dy + dz*dz).sqrt();
        self.solutions.push(ProjSolution { u: proj[0], v: proj[1], distance: dist, point: proj });
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_points(&self) -> usize { self.solutions.len() }

    pub fn lower_distance_parameters(&self) -> Option<(f64, f64)> {
        self.solutions.iter().min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .map(|s| (s.u, s.v))
    }

    pub fn lower_distance_point(&self) -> Option<[f64; 3]> {
        self.solutions.iter().min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .map(|s| s.point)
    }

    pub fn lower_distance(&self) -> f64 {
        self.solutions.iter().map(|s| s.distance).fold(f64::INFINITY, f64::min)
    }
}

// occt-ref: GeomAPI_ProjectPointOnCurve
/// Projects a point onto a curve, finding the nearest parameter t.
#[derive(Clone, Debug)]
pub struct ProjectPointOnCurve {
    pub point: [f64; 3],
    pub curve_pts: Vec<[f64; 3]>,
    pub solutions: Vec<CurveProjSolution>,
}

#[derive(Clone, Debug)]
pub struct CurveProjSolution {
    pub parameter: f64,
    pub distance: f64,
    pub point: [f64; 3],
}

impl ProjectPointOnCurve {
    pub fn new(point: [f64; 3], curve_pts: Vec<[f64; 3]>) -> Self {
        let mut s = Self { point, curve_pts, solutions: Vec::new() };
        s.perform();
        s
    }

    fn perform(&mut self) {
        let mut best_dist = f64::INFINITY;
        let mut best_t = 0.0;
        let mut best_pt = self.point;
        let n = self.curve_pts.len();
        for i in 0..n {
            let p = self.curve_pts[i];
            let dx = self.point[0]-p[0]; let dy = self.point[1]-p[1]; let dz = self.point[2]-p[2];
            let d = (dx*dx+dy*dy+dz*dz).sqrt();
            if d < best_dist {
                best_dist = d;
                best_t = i as f64 / (n-1).max(1) as f64;
                best_pt = p;
            }
        }
        self.solutions.push(CurveProjSolution { parameter: best_t, distance: best_dist, point: best_pt });
    }

    pub fn nb_points(&self) -> usize { self.solutions.len() }
    pub fn lower_distance(&self) -> f64 {
        self.solutions.iter().map(|s| s.distance).fold(f64::INFINITY, f64::min)
    }
    pub fn lower_distance_parameter(&self) -> f64 {
        self.solutions.iter().min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .map_or(0.0, |s| s.parameter)
    }
}

// occt-ref: ProjLib_ProjectedCurve
/// A 2D curve resulting from projecting a 3D curve onto a surface.
#[derive(Clone, Debug)]
pub struct ProjectedCurve {
    pub surface_id: usize,
    pub curve_pts_3d: Vec<[f64; 3]>,
    pub curve_pts_2d: Vec<[f64; 2]>,
    pub done: bool,
}

impl ProjectedCurve {
    pub fn new(surface_id: usize, curve_pts_3d: Vec<[f64; 3]>) -> Self {
        let mut c = Self { surface_id, curve_pts_3d, curve_pts_2d: Vec::new(), done: false };
        c.build();
        c
    }

    fn build(&mut self) {
        // Stub: just use (x, y) as the UV projection (flat Z=0 surface)
        self.curve_pts_2d = self.curve_pts_3d.iter().map(|&p| [p[0], p[1]]).collect();
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_knots(&self) -> usize { self.curve_pts_2d.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_point_on_surf() {
        let p = ProjectPointOnSurf::new([3.0, 4.0, 5.0], 1);
        assert!(p.is_done());
        assert_eq!(p.nb_points(), 1);
        let (u, v) = p.lower_distance_parameters().unwrap();
        assert!((u - 3.0).abs() < 1e-10);
        assert!((v - 4.0).abs() < 1e-10);
        assert!((p.lower_distance() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn project_point_on_curve() {
        let curve = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]];
        let p = ProjectPointOnCurve::new([1.0, 1.0, 0.0], curve);
        assert_eq!(p.nb_points(), 1);
        assert!((p.lower_distance() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn projected_curve() {
        let pts = vec![[0.0,0.0,1.0],[1.0,2.0,1.0]];
        let c = ProjectedCurve::new(1, pts);
        assert!(c.is_done());
        assert_eq!(c.nb_knots(), 2);
        assert_eq!(c.curve_pts_2d[0], [0.0, 0.0]);
    }

    #[test]
    fn project_point_on_curve_parameter() {
        let curve = vec![[0.0,0.0,0.0],[5.0,0.0,0.0]];
        let p = ProjectPointOnCurve::new([5.0, 0.0, 0.0], curve);
        assert!((p.lower_distance_parameter() - 1.0).abs() < 1e-10);
    }
}
