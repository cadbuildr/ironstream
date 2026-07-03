// FILE: geom_curve_builder2.rs
// occt: GC_MakeLine, GC_MakeSegment, GC_MakeHyperbola, GC_MakeParabola

// occt: GC_MakeLine
/// Builds an infinite line from origin + direction.
#[derive(Clone, Debug)]
pub struct MakeLine {
    pub origin: [f64; 3],
    pub direction: [f64; 3],
    pub done: bool,
}

impl MakeLine {
    pub fn from_origin_dir(origin: [f64; 3], direction: [f64; 3]) -> Self {
        let len = (direction[0]*direction[0]+direction[1]*direction[1]+direction[2]*direction[2]).sqrt();
        let done = len > 1e-14;
        let dir = if done { [direction[0]/len, direction[1]/len, direction[2]/len] } else { [1.0, 0.0, 0.0] };
        Self { origin, direction: dir, done }
    }

    pub fn from_two_points(p1: [f64; 3], p2: [f64; 3]) -> Self {
        let d = [p2[0]-p1[0], p2[1]-p1[1], p2[2]-p1[2]];
        Self::from_origin_dir(p1, d)
    }

    pub fn is_done(&self) -> bool { self.done }

    pub fn point_at(&self, t: f64) -> [f64; 3] {
        [
            self.origin[0] + t * self.direction[0],
            self.origin[1] + t * self.direction[1],
            self.origin[2] + t * self.direction[2],
        ]
    }

    pub fn project_parameter(&self, p: [f64; 3]) -> f64 {
        let d = [p[0]-self.origin[0], p[1]-self.origin[1], p[2]-self.origin[2]];
        d[0]*self.direction[0]+d[1]*self.direction[1]+d[2]*self.direction[2]
    }

    pub fn distance_to_point(&self, p: [f64; 3]) -> f64 {
        let t = self.project_parameter(p);
        let q = self.point_at(t);
        let dx=p[0]-q[0]; let dy=p[1]-q[1]; let dz=p[2]-q[2];
        (dx*dx+dy*dy+dz*dz).sqrt()
    }
}

// occt: GC_MakeSegment
/// Builds a bounded line segment from two points or from line + two params.
#[derive(Clone, Debug)]
pub struct MakeSegment {
    pub line: MakeLine,
    pub t1: f64,
    pub t2: f64,
    pub done: bool,
}

impl MakeSegment {
    pub fn from_two_points(p1: [f64; 3], p2: [f64; 3]) -> Self {
        let line = MakeLine::from_two_points(p1, p2);
        let dx=p2[0]-p1[0]; let dy=p2[1]-p1[1]; let dz=p2[2]-p1[2];
        let len = (dx*dx+dy*dy+dz*dz).sqrt();
        let done = len > 1e-14;
        Self { line, t1: 0.0, t2: len, done }
    }

    pub fn from_line(origin: [f64; 3], direction: [f64; 3], t1: f64, t2: f64) -> Self {
        let line = MakeLine::from_origin_dir(origin, direction);
        let done = line.done && t2 > t1;
        Self { line, t1, t2, done }
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn length(&self) -> f64 { (self.t2 - self.t1).abs() }
    pub fn start_point(&self) -> [f64; 3] { self.line.point_at(self.t1) }
    pub fn end_point(&self) -> [f64; 3] { self.line.point_at(self.t2) }

    pub fn discretize(&self, n: usize) -> Vec<[f64; 3]> {
        (0..=n).map(|i| {
            let t = self.t1 + (self.t2 - self.t1) * i as f64 / n as f64;
            self.line.point_at(t)
        }).collect()
    }
}

// occt: GC_MakeHyperbola
#[derive(Clone, Debug)]
pub struct MakeHyperbola {
    pub center: [f64; 3],
    pub x_dir: [f64; 3],
    pub major_radius: f64,
    pub minor_radius: f64,
    pub done: bool,
}

impl MakeHyperbola {
    pub fn new(center: [f64; 3], x_dir: [f64; 3], major: f64, minor: f64) -> Self {
        let len = (x_dir[0]*x_dir[0]+x_dir[1]*x_dir[1]+x_dir[2]*x_dir[2]).sqrt();
        let x = if len > 1e-14 { [x_dir[0]/len, x_dir[1]/len, x_dir[2]/len] } else { [1.0,0.0,0.0] };
        Self { center, x_dir: x, major_radius: major, minor_radius: minor, done: major > 0.0 && minor > 0.0 }
    }

    pub fn is_done(&self) -> bool { self.done }

    pub fn point_at(&self, t: f64) -> [f64; 3] {
        // Parametric: x = a*cosh(t), y = b*sinh(t)
        let perp = perp_to(self.x_dir);
        [
            self.center[0] + self.major_radius * t.cosh() * self.x_dir[0] + self.minor_radius * t.sinh() * perp[0],
            self.center[1] + self.major_radius * t.cosh() * self.x_dir[1] + self.minor_radius * t.sinh() * perp[1],
            self.center[2] + self.major_radius * t.cosh() * self.x_dir[2] + self.minor_radius * t.sinh() * perp[2],
        ]
    }

    pub fn eccentricity(&self) -> f64 {
        if self.major_radius < 1e-14 { return 1.0; }
        ((self.major_radius*self.major_radius + self.minor_radius*self.minor_radius).sqrt()) / self.major_radius
    }
}

// occt: GC_MakeParabola
#[derive(Clone, Debug)]
pub struct MakeParabola {
    pub focus: [f64; 3],
    pub axis_dir: [f64; 3],
    pub focal_param: f64,
    pub done: bool,
}

impl MakeParabola {
    pub fn new(focus: [f64; 3], axis_dir: [f64; 3], focal_param: f64) -> Self {
        let len = (axis_dir[0]*axis_dir[0]+axis_dir[1]*axis_dir[1]+axis_dir[2]*axis_dir[2]).sqrt();
        let ax = if len > 1e-14 { [axis_dir[0]/len, axis_dir[1]/len, axis_dir[2]/len] } else { [1.0,0.0,0.0] };
        Self { focus, axis_dir: ax, focal_param, done: focal_param > 0.0 }
    }

    pub fn is_done(&self) -> bool { self.done }

    pub fn point_at(&self, t: f64) -> [f64; 3] {
        let perp = perp_to(self.axis_dir);
        let x = t * t / (4.0 * self.focal_param);
        [
            self.focus[0] + x * self.axis_dir[0] + t * perp[0],
            self.focus[1] + x * self.axis_dir[1] + t * perp[1],
            self.focus[2] + x * self.axis_dir[2] + t * perp[2],
        ]
    }

    pub fn directrix_distance(&self, p: [f64; 3]) -> f64 {
        let d = [p[0]-self.focus[0], p[1]-self.focus[1], p[2]-self.focus[2]];
        let proj = d[0]*self.axis_dir[0]+d[1]*self.axis_dir[1]+d[2]*self.axis_dir[2];
        (proj + self.focal_param).abs()
    }
}

fn perp_to(n: [f64; 3]) -> [f64; 3] {
    let c = if n[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let v = [n[1]*c[2]-n[2]*c[1], n[2]*c[0]-n[0]*c[2], n[0]*c[1]-n[1]*c[0]];
    let len = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();
    if len < 1e-14 { return [0.0,1.0,0.0]; }
    [v[0]/len, v[1]/len, v[2]/len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_line() {
        let l = MakeLine::from_origin_dir([0.0;3], [1.0,0.0,0.0]);
        assert!(l.is_done());
        let p = l.point_at(3.0);
        assert!((p[0] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn make_line_distance() {
        let l = MakeLine::from_origin_dir([0.0;3], [1.0,0.0,0.0]);
        assert!((l.distance_to_point([2.0, 3.0, 0.0]) - 3.0).abs() < 1e-8);
    }

    #[test]
    fn make_segment() {
        let s = MakeSegment::from_two_points([0.0,0.0,0.0], [4.0,0.0,0.0]);
        assert!(s.is_done());
        assert!((s.length() - 4.0).abs() < 1e-8);
        let pts = s.discretize(4);
        assert_eq!(pts.len(), 5);
    }

    #[test]
    fn make_hyperbola() {
        let h = MakeHyperbola::new([0.0;3], [1.0,0.0,0.0], 2.0, 1.0);
        assert!(h.is_done());
        let p0 = h.point_at(0.0);
        assert!((p0[0] - 2.0).abs() < 1e-8);
        assert!(h.eccentricity() > 1.0);
    }

    #[test]
    fn make_parabola() {
        let p = MakeParabola::new([0.0;3], [1.0,0.0,0.0], 1.0);
        assert!(p.is_done());
        let pt = p.point_at(0.0);
        assert!((pt[0]).abs() < 1e-10);
    }
}
