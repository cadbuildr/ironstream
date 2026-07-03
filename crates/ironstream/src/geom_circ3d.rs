// FILE: geom_circ3d.rs
// occt: GC_MakeCircle, GC_MakeArcOfCircle, gp_Circ (extensions), Geom_Circle

// occt: GC_MakeCircle (full circle through 3 points or center+radius)
#[derive(Clone, Debug)]
pub struct MakeCircle {
    pub center: [f64; 3],
    pub normal: [f64; 3],
    pub radius: f64,
    pub done: bool,
}

impl MakeCircle {
    /// From center, normal direction, and radius.
    pub fn from_center_normal(center: [f64; 3], normal: [f64; 3], radius: f64) -> Self {
        let n = normalize3(normal);
        Self { center, normal: n, radius, done: true }
    }

    /// Circumscribed circle through three non-collinear points.
    pub fn from_3_points(p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> Self {
        let center = circumcenter(p1, p2, p3);
        let v1 = sub3(p2, p1);
        let v2 = sub3(p3, p1);
        let normal = normalize3(cross3(v1, v2));
        let dx = center[0]-p1[0]; let dy = center[1]-p1[1]; let dz = center[2]-p1[2];
        let radius = (dx*dx+dy*dy+dz*dz).sqrt();
        let done = radius > 1e-14;
        Self { center, normal, radius, done }
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn center(&self) -> [f64; 3] { self.center }

    /// Evaluate a point on the circle at parameter t (radians).
    pub fn point_at(&self, t: f64) -> [f64; 3] {
        let x_dir = perp_to(self.normal);
        let y_dir = cross3(self.normal, x_dir);
        [
            self.center[0] + self.radius*(t.cos()*x_dir[0] + t.sin()*y_dir[0]),
            self.center[1] + self.radius*(t.cos()*x_dir[1] + t.sin()*y_dir[1]),
            self.center[2] + self.radius*(t.cos()*x_dir[2] + t.sin()*y_dir[2]),
        ]
    }

    /// Discretize the circle into n points.
    pub fn discretize(&self, n: usize) -> Vec<[f64; 3]> {
        (0..=n).map(|i| {
            let t = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            self.point_at(t)
        }).collect()
    }
}

// occt: GC_MakeArcOfCircle
#[derive(Clone, Debug)]
pub struct MakeArcOfCircle {
    pub center: [f64; 3],
    pub normal: [f64; 3],
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    pub done: bool,
}

impl MakeArcOfCircle {
    pub fn new(center: [f64; 3], normal: [f64; 3], radius: f64, start_deg: f64, end_deg: f64) -> Self {
        let pi = std::f64::consts::PI;
        Self {
            center, normal: normalize3(normal), radius,
            start_angle: start_deg * pi / 180.0,
            end_angle: end_deg * pi / 180.0,
            done: true,
        }
    }

    pub fn from_3_points(p1: [f64; 3], p_mid: [f64; 3], p2: [f64; 3]) -> Self {
        let circ = MakeCircle::from_3_points(p1, p_mid, p2);
        Self {
            center: circ.center,
            normal: circ.normal,
            radius: circ.radius,
            start_angle: 0.0,
            end_angle: std::f64::consts::PI,
            done: circ.done,
        }
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn arc_length(&self) -> f64 { self.radius * (self.end_angle - self.start_angle).abs() }

    pub fn discretize(&self, n: usize) -> Vec<[f64; 3]> {
        let x_dir = perp_to(self.normal);
        let y_dir = cross3(self.normal, x_dir);
        (0..=n).map(|i| {
            let t = self.start_angle + (self.end_angle - self.start_angle) * i as f64 / n as f64;
            [
                self.center[0] + self.radius*(t.cos()*x_dir[0] + t.sin()*y_dir[0]),
                self.center[1] + self.radius*(t.cos()*x_dir[1] + t.sin()*y_dir[1]),
                self.center[2] + self.radius*(t.cos()*x_dir[2] + t.sin()*y_dir[2]),
            ]
        }).collect()
    }
}

fn circumcenter(p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> [f64; 3] {
    let a = sub3(p2, p1);
    let b = sub3(p3, p1);
    let d = 2.0 * dot3(cross3(a, b), cross3(a, b));
    if d.abs() < 1e-28 { return p1; }
    let x = cross3(
        [dot3(a,a)*b[0]-dot3(b,b)*a[0], dot3(a,a)*b[1]-dot3(b,b)*a[1], dot3(a,a)*b[2]-dot3(b,b)*a[2]],
        cross3(a, b)
    );
    [p1[0]+x[0]/d, p1[1]+x[1]/d, p1[2]+x[2]/d]
}

fn perp_to(n: [f64; 3]) -> [f64; 3] {
    let candidate = if n[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    normalize3(cross3(n, candidate))
}

fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();
    if len < 1e-14 { return [1.0, 0.0, 0.0]; }
    [v[0]/len, v[1]/len, v[2]/len]
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0], a[1]-b[1], a[2]-b[2]] }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_circle_center_radius() {
        let c = MakeCircle::from_center_normal([0.0;3], [0.0, 0.0, 1.0], 5.0);
        assert!(c.is_done());
        assert!((c.radius() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn make_circle_3_points() {
        let p1 = [1.0, 0.0, 0.0];
        let p2 = [0.0, 1.0, 0.0];
        let p3 = [-1.0, 0.0, 0.0];
        let c = MakeCircle::from_3_points(p1, p2, p3);
        assert!(c.is_done());
        assert!((c.radius() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn make_circle_discretize() {
        let c = MakeCircle::from_center_normal([0.0;3], [0.0, 0.0, 1.0], 1.0);
        let pts = c.discretize(8);
        assert_eq!(pts.len(), 9);
        for p in &pts {
            let r = (p[0]*p[0]+p[1]*p[1]+p[2]*p[2]).sqrt();
            assert!((r - 1.0).abs() < 1e-8, "radius mismatch: {}", r);
        }
    }

    #[test]
    fn make_arc_length() {
        let arc = MakeArcOfCircle::new([0.0;3], [0.0,0.0,1.0], 1.0, 0.0, 180.0);
        assert!(arc.is_done());
        assert!((arc.arc_length() - std::f64::consts::PI).abs() < 1e-8);
    }

    #[test]
    fn make_arc_3_points() {
        let arc = MakeArcOfCircle::from_3_points([1.0,0.0,0.0], [0.0,1.0,0.0], [-1.0,0.0,0.0]);
        assert!(arc.is_done());
        let pts = arc.discretize(4);
        assert_eq!(pts.len(), 5);
    }
}
