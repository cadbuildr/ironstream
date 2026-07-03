// FILE: geom_axis_tool.rs
// occt: gp_Ax1, gp_Ax2, gp_Ax3, gp_Trsf axis placement and transformation

/// Axis 1: a point and a direction (infinite line, no up-vector).
#[derive(Clone, Copy, Debug)]
pub struct Ax1 {
    pub location: [f64; 3],
    pub direction: [f64; 3],
}

impl Ax1 {
    pub fn new(location: [f64; 3], direction: [f64; 3]) -> Self {
        let d = normalize(direction);
        Self { location, direction: d }
    }

    pub fn x_axis() -> Self { Self::new([0.0;3], [1.0,0.0,0.0]) }
    pub fn y_axis() -> Self { Self::new([0.0;3], [0.0,1.0,0.0]) }
    pub fn z_axis() -> Self { Self::new([0.0;3], [0.0,0.0,1.0]) }

    pub fn point_at(&self, t: f64) -> [f64; 3] {
        [self.location[0]+t*self.direction[0], self.location[1]+t*self.direction[1], self.location[2]+t*self.direction[2]]
    }

    pub fn project(&self, p: [f64; 3]) -> f64 {
        let d = sub(p, self.location);
        dot(d, self.direction)
    }

    pub fn distance_to(&self, p: [f64; 3]) -> f64 {
        let t = self.project(p);
        let q = self.point_at(t);
        mag(sub(p, q))
    }

    pub fn is_parallel(&self, other: &Ax1, tol: f64) -> bool {
        let c = cross(self.direction, other.direction);
        mag(c) < tol
    }
}

impl Default for Ax1 {
    fn default() -> Self { Self::z_axis() }
}

/// Axis 2: right-handed coordinate system (origin + main dir + x dir).
#[derive(Clone, Copy, Debug)]
pub struct Ax2 {
    pub origin: [f64; 3],
    pub z_dir: [f64; 3],
    pub x_dir: [f64; 3],
}

impl Ax2 {
    pub fn new(origin: [f64; 3], z_dir: [f64; 3], x_dir: [f64; 3]) -> Self {
        let z = normalize(z_dir);
        let x = normalize(sub(x_dir, scale(dot(x_dir, z), z)));
        Self { origin, z_dir: z, x_dir: x }
    }

    pub fn y_dir(&self) -> [f64; 3] { cross(self.z_dir, self.x_dir) }

    pub fn to_world(&self, local: [f64; 3]) -> [f64; 3] {
        let y = self.y_dir();
        let mut r = self.origin;
        r[0] += local[0]*self.x_dir[0] + local[1]*y[0] + local[2]*self.z_dir[0];
        r[1] += local[0]*self.x_dir[1] + local[1]*y[1] + local[2]*self.z_dir[1];
        r[2] += local[0]*self.x_dir[2] + local[1]*y[2] + local[2]*self.z_dir[2];
        r
    }

    pub fn to_local(&self, world: [f64; 3]) -> [f64; 3] {
        let d = sub(world, self.origin);
        let y = self.y_dir();
        [dot(d, self.x_dir), dot(d, y), dot(d, self.z_dir)]
    }
}

impl Default for Ax2 {
    fn default() -> Self { Self::new([0.0;3], [0.0,0.0,1.0], [1.0,0.0,0.0]) }
}

/// Placement: axis + scale transform.
#[derive(Clone, Copy, Debug)]
pub struct AxisPlacement {
    pub ax2: Ax2,
    pub scale: f64,
}

impl AxisPlacement {
    pub fn new(ax2: Ax2) -> Self { Self { ax2, scale: 1.0 } }
    pub fn with_scale(mut self, s: f64) -> Self { self.scale = s; self }
    pub fn transform(&self, p: [f64; 3]) -> [f64; 3] {
        let local = self.ax2.to_local(p);
        let scaled = [local[0]*self.scale, local[1]*self.scale, local[2]*self.scale];
        self.ax2.to_world(scaled)
    }
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let m = mag(v); if m > 1e-14 { [v[0]/m,v[1]/m,v[2]/m] } else { [0.0,0.0,1.0] }
}
fn mag(v: [f64; 3]) -> f64 { (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt() }
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0],a[1]-b[1],a[2]-b[2]] }
fn scale(s: f64, v: [f64; 3]) -> [f64; 3] { [s*v[0],s*v[1],s*v[2]] }
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}
fn add3(a: [f64; 3], b: [f64; 3]) -> Result<[f64; 3], ()> { Ok([a[0]+b[0],a[1]+b[1],a[2]+b[2]]) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ax1_project() {
        let ax = Ax1::x_axis();
        assert!((ax.project([3.0, 0.0, 0.0]) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn ax1_distance() {
        let ax = Ax1::x_axis();
        assert!((ax.distance_to([0.0, 3.0, 4.0]) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn ax2_round_trip() {
        let ax = Ax2::default();
        let p = [1.0, 2.0, 3.0];
        let local = ax.to_local(p);
        let back = ax.to_world(local);
        assert!((back[0] - p[0]).abs() < 1e-10);
        assert!((back[1] - p[1]).abs() < 1e-10);
        assert!((back[2] - p[2]).abs() < 1e-10);
    }

    #[test]
    fn ax1_parallel() {
        let a = Ax1::x_axis();
        let b = Ax1::new([1.0,1.0,0.0], [2.0,0.0,0.0]);
        assert!(a.is_parallel(&b, 1e-10));
    }
}
