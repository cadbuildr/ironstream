// FILE: ais_plane.rs
// occt: AIS_Plane, AIS_Circle, AIS_Point, AIS_Line

/// Display mode for geometric AIS shapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomAisMode {
    Whole,       // entire plane/line/circle
    Axis,        // axis of the geometry
    NPlane,      // normal to plane (for display)
}

// occt: AIS_Plane — interactive plane shape
#[derive(Clone, Debug)]
pub struct AisPlane {
    pub center: [f64; 3],
    pub normal: [f64; 3],
    pub x_axis: [f64; 3],
    pub size: f64,
    pub display_mode: GeomAisMode,
    pub color: [f32; 3],
}

impl AisPlane {
    pub fn new(center: [f64; 3], normal: [f64; 3], size: f64) -> Self {
        let n = normalize(normal);
        let x = perp(n);
        Self { center, normal: n, x_axis: x, size, display_mode: GeomAisMode::Whole, color: [0.5,0.5,0.8] }
    }

    pub fn y_axis(&self) -> [f64; 3] { cross(self.normal, self.x_axis) }

    /// Four corners of the square representation.
    pub fn corners(&self) -> [[f64; 3]; 4] {
        let h = self.size / 2.0;
        let y = self.y_axis();
        [
            add(self.center, add(scale(h, self.x_axis), scale(h, y))),
            add(self.center, add(scale(-h, self.x_axis), scale(h, y))),
            add(self.center, add(scale(-h, self.x_axis), scale(-h, y))),
            add(self.center, add(scale(h, self.x_axis), scale(-h, y))),
        ]
    }

    pub fn contains_point(&self, p: [f64; 3], tol: f64) -> bool {
        let d = dot(sub(p, self.center), self.normal);
        d.abs() < tol
    }

    pub fn project_point(&self, p: [f64; 3]) -> [f64; 3] {
        let d = dot(sub(p, self.center), self.normal);
        sub(p, scale(d, self.normal))
    }
}

// occt: AIS_Circle — interactive circle shape
#[derive(Clone, Debug)]
pub struct AisCircle {
    pub center: [f64; 3],
    pub normal: [f64; 3],
    pub radius: f64,
    pub color: [f32; 3],
    pub is_filled: bool,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl AisCircle {
    pub fn new(center: [f64; 3], normal: [f64; 3], radius: f64) -> Self {
        Self {
            center, normal: normalize(normal), radius,
            color: [1.0,1.0,0.0], is_filled: false,
            start_angle: 0.0, end_angle: std::f64::consts::TAU,
        }
    }

    pub fn arc(center: [f64; 3], normal: [f64; 3], radius: f64, start: f64, end: f64) -> Self {
        let mut c = Self::new(center, normal, radius);
        c.start_angle = start; c.end_angle = end; c
    }

    pub fn is_full_circle(&self) -> bool {
        (self.end_angle - self.start_angle - std::f64::consts::TAU).abs() < 1e-10
    }

    pub fn perimeter(&self) -> f64 {
        let span = (self.end_angle - self.start_angle).abs();
        self.radius * span
    }

    pub fn area(&self) -> f64 {
        let span = (self.end_angle - self.start_angle).abs();
        0.5 * self.radius * self.radius * span
    }

    pub fn point_at(&self, angle: f64) -> [f64; 3] {
        let x = perp(self.normal);
        let y = cross(self.normal, x);
        add(self.center, add(scale(self.radius * angle.cos(), x), scale(self.radius * angle.sin(), y)))
    }
}

// occt: AIS_Point — interactive point marker
#[derive(Clone, Debug)]
pub struct AisPoint {
    pub position: [f64; 3],
    pub marker_type: PointMarker,
    pub marker_size: f32,
    pub color: [f32; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointMarker {
    Point,      // dot
    Plus,       // +
    Star,       // *
    X,          // x
    Circle,     // O
    Square,     // □
    Triangle,   // △
}

impl AisPoint {
    pub fn new(position: [f64; 3]) -> Self {
        Self { position, marker_type: PointMarker::Point, marker_size: 4.0, color: [1.0,1.0,0.0] }
    }

    pub fn with_marker(mut self, m: PointMarker) -> Self { self.marker_type = m; self }
    pub fn with_size(mut self, s: f32) -> Self { self.marker_size = s; self }
}

// occt: AIS_Line — interactive line/segment shape
#[derive(Clone, Debug)]
pub struct AisLine {
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub color: [f32; 3],
    pub line_width: f32,
    pub is_infinite: bool,
}

impl AisLine {
    pub fn segment(start: [f64; 3], end: [f64; 3]) -> Self {
        Self { start, end, color: [1.0,1.0,1.0], line_width: 1.0, is_infinite: false }
    }

    pub fn infinite(origin: [f64; 3], direction: [f64; 3]) -> Self {
        let d = normalize(direction);
        Self { start: origin, end: add(origin, d), color: [1.0,1.0,1.0], line_width: 1.0, is_infinite: true }
    }

    pub fn length(&self) -> f64 {
        let dx=self.end[0]-self.start[0]; let dy=self.end[1]-self.start[1]; let dz=self.end[2]-self.start[2];
        (dx*dx+dy*dy+dz*dz).sqrt()
    }
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0],a[1]-b[1],a[2]-b[2]] }
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]+b[0],a[1]+b[1],a[2]+b[2]] }
fn scale(s: f64, v: [f64; 3]) -> [f64; 3] { [s*v[0],s*v[1],s*v[2]] }
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]] }
fn mag(v: [f64; 3]) -> f64 { (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt() }
fn normalize(v: [f64; 3]) -> [f64; 3] { let m=mag(v); if m>1e-14{[v[0]/m,v[1]/m,v[2]/m]}else{[0.0,0.0,1.0]} }
fn perp(n: [f64; 3]) -> [f64; 3] { let c=if n[0].abs()<0.9{[1.0,0.0,0.0]}else{[0.0,1.0,0.0]}; normalize(cross(n,c)) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ais_plane_corners() {
        let p = AisPlane::new([0.0;3], [0.0,0.0,1.0], 2.0);
        let c = p.corners();
        assert_eq!(c.len(), 4);
        // All corners should be in z=0 plane
        for corner in &c { assert!(corner[2].abs() < 1e-10); }
    }

    #[test]
    fn ais_plane_project() {
        let p = AisPlane::new([0.0;3], [0.0,0.0,1.0], 4.0);
        let pt = [1.0, 2.0, 5.0];
        let proj = p.project_point(pt);
        assert!(proj[2].abs() < 1e-10);
        assert!((proj[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn ais_circle_full() {
        let c = AisCircle::new([0.0;3], [0.0,0.0,1.0], 3.0);
        assert!(c.is_full_circle());
        assert!((c.perimeter() - 2.0*std::f64::consts::PI*3.0).abs() < 1e-8);
    }

    #[test]
    fn ais_line_length() {
        let l = AisLine::segment([0.0;3], [3.0,4.0,0.0]);
        assert!((l.length() - 5.0).abs() < 1e-10);
    }
}
