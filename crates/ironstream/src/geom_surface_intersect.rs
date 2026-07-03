// FILE: geom_surface_intersect.rs
// occt: GeomAPI_IntCS (curve/surface), GeomAPI_IntSS (surface/surface)

// occt: GeomAPI_IntCS
/// Intersection of a curve with a surface (finds parameter values where curve meets surface).
#[derive(Clone, Debug)]
pub struct IntCS {
    pub curve_pts: Vec<[f64; 3]>,
    pub surface_plane: Option<[f64; 4]>,
    pub solutions: Vec<IntCSPoint>,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct IntCSPoint {
    pub parameter_on_curve: f64,
    pub u: f64,
    pub v: f64,
    pub point: [f64; 3],
}

impl IntCS {
    pub fn new(curve_pts: Vec<[f64; 3]>) -> Self {
        Self { curve_pts, surface_plane: None, solutions: Vec::new(), done: false }
    }

    pub fn set_plane(&mut self, normal: [f64; 3], d: f64) {
        self.surface_plane = Some([normal[0], normal[1], normal[2], d]);
    }

    pub fn perform(&mut self) {
        self.solutions.clear();
        if let Some(plane) = self.surface_plane {
            let n = [plane[0], plane[1], plane[2]];
            let d = plane[3];
            let pts = &self.curve_pts;
            let np = pts.len();
            if np < 2 { self.done = true; return; }
            for i in 0..np-1 {
                let fa = pts[i][0]*n[0]+pts[i][1]*n[1]+pts[i][2]*n[2] - d;
                let fb = pts[i+1][0]*n[0]+pts[i+1][1]*n[1]+pts[i+1][2]*n[2] - d;
                // Skip if start of segment is exactly on plane and not first segment
                // (avoids double-counting at vertex intersections)
                if fa.abs() < 1e-14 && i > 0 { continue; }
                if fa * fb <= 0.0 {
                    let t = if (fb - fa).abs() < 1e-14 { 0.0 } else { -fa / (fb - fa) };
                    let frac = (i as f64 + t) / (np - 1) as f64;
                    let p = [
                        pts[i][0] + t*(pts[i+1][0]-pts[i][0]),
                        pts[i][1] + t*(pts[i+1][1]-pts[i][1]),
                        pts[i][2] + t*(pts[i+1][2]-pts[i][2]),
                    ];
                    self.solutions.push(IntCSPoint { parameter_on_curve: frac, u: p[0], v: p[1], point: p });
                }
            }
        }
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_points(&self) -> usize { self.solutions.len() }
    pub fn point(&self, i: usize) -> Option<&IntCSPoint> { self.solutions.get(i.saturating_sub(1)) }
}

// occt: GeomAPI_IntSS
/// Intersection of two parametric surfaces (returns a list of intersection curves).
#[derive(Clone, Debug)]
pub struct IntSS {
    pub surface_a_id: usize,
    pub surface_b_id: usize,
    pub tol: f64,
    pub curves: Vec<IntSSCurve>,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct IntSSCurve {
    pub points: Vec<[f64; 3]>,
    pub is_closed: bool,
}

impl IntSSCurve {
    pub fn new(points: Vec<[f64; 3]>) -> Self {
        let is_closed = points.len() > 2 && {
            let a = points[0]; let b = *points.last().unwrap();
            let dx=a[0]-b[0]; let dy=a[1]-b[1]; let dz=a[2]-b[2];
            (dx*dx+dy*dy+dz*dz).sqrt() < 1e-8
        };
        Self { points, is_closed }
    }

    pub fn nb_points(&self) -> usize { self.points.len() }
}

impl IntSS {
    pub fn new(surface_a_id: usize, surface_b_id: usize, tol: f64) -> Self {
        Self { surface_a_id, surface_b_id, tol, curves: Vec::new(), done: false }
    }

    pub fn perform(&mut self) {
        // Stub: no intersection by default
        self.done = true;
    }

    pub fn add_curve(&mut self, c: IntSSCurve) { self.curves.push(c); }
    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_lines(&self) -> usize { self.curves.len() }
    pub fn line(&self, i: usize) -> Option<&IntSSCurve> { self.curves.get(i.saturating_sub(1)) }
}

// occt: IntAna_Quadric (analytical surface for intersection)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuadricType {
    Plane,
    Sphere,
    Cylinder,
    Cone,
    Torus,
}

#[derive(Clone, Debug)]
pub struct Quadric {
    pub qtype: QuadricType,
    pub center: [f64; 3],
    pub radius: f64,
    pub half_angle: f64,
}

impl Quadric {
    pub fn plane(normal: [f64; 3], d: f64) -> Self {
        Self { qtype: QuadricType::Plane, center: normal, radius: d, half_angle: 0.0 }
    }

    pub fn sphere(center: [f64; 3], radius: f64) -> Self {
        Self { qtype: QuadricType::Sphere, center, radius, half_angle: 0.0 }
    }

    pub fn cylinder(axis: [f64; 3], radius: f64) -> Self {
        Self { qtype: QuadricType::Cylinder, center: axis, radius, half_angle: 0.0 }
    }

    pub fn name(&self) -> &'static str {
        match self.qtype {
            QuadricType::Plane => "Plane",
            QuadricType::Sphere => "Sphere",
            QuadricType::Cylinder => "Cylinder",
            QuadricType::Cone => "Cone",
            QuadricType::Torus => "Torus",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intcs_with_plane() {
        let curve = vec![[0.0,0.0,-1.0],[0.0,0.0,0.0],[0.0,0.0,1.0]];
        let mut intcs = IntCS::new(curve);
        intcs.set_plane([0.0,0.0,1.0], 0.0);
        intcs.perform();
        assert!(intcs.is_done());
        assert_eq!(intcs.nb_points(), 1);
        assert!((intcs.point(1).unwrap().point[2]).abs() < 1e-8);
    }

    #[test]
    fn intcs_no_plane() {
        let curve = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let mut intcs = IntCS::new(curve);
        intcs.perform();
        assert!(intcs.is_done());
        assert_eq!(intcs.nb_points(), 0);
    }

    #[test]
    fn intss_empty() {
        let mut s = IntSS::new(1, 2, 1e-6);
        s.perform();
        assert!(s.is_done());
        assert_eq!(s.nb_lines(), 0);
    }

    #[test]
    fn intss_curve() {
        let mut s = IntSS::new(1, 2, 1e-6);
        s.perform();
        s.add_curve(IntSSCurve::new(vec![[0.0,0.0,0.0],[1.0,0.0,0.0]]));
        assert_eq!(s.nb_lines(), 1);
        assert_eq!(s.line(1).unwrap().nb_points(), 2);
    }

    #[test]
    fn quadric_types() {
        let pl = Quadric::plane([0.0,0.0,1.0], 0.0);
        assert_eq!(pl.name(), "Plane");
        let sp = Quadric::sphere([0.0;3], 5.0);
        assert!((sp.radius - 5.0).abs() < 1e-10);
    }
}
