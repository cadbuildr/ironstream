// FILE: geom_lcs.rs
// occt-note: Geom_LocalCoordinateSystem, TopLoc_Datum3D, TopLoc_Location extensions

// occt-note: Geom_LocalCoordinateSystem (approximated by Geom_Axis2Placement)
/// A local coordinate system in 3-D: origin + X axis + Y axis (Z = X cross Y).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalCS {
    pub origin: [f64; 3],
    pub x_dir: [f64; 3],
    pub y_dir: [f64; 3],
}

impl LocalCS {
    /// Construct from origin, X direction, and Y direction (orthogonalized internally).
    pub fn new(origin: [f64; 3], x_dir: [f64; 3], y_dir: [f64; 3]) -> Self {
        let x = normalize3(x_dir);
        let z = normalize3(cross3(x, y_dir));
        let y = cross3(z, x);
        Self { origin, x_dir: x, y_dir: y }
    }

    pub fn identity() -> Self {
        Self { origin: [0.0; 3], x_dir: [1.0, 0.0, 0.0], y_dir: [0.0, 1.0, 0.0] }
    }

    pub fn z_dir(&self) -> [f64; 3] {
        normalize3(cross3(self.x_dir, self.y_dir))
    }

    /// Transform a point from world to local coordinates.
    pub fn world_to_local(&self, p: [f64; 3]) -> [f64; 3] {
        let d = sub3(p, self.origin);
        [dot3(d, self.x_dir), dot3(d, self.y_dir), dot3(d, self.z_dir())]
    }

    /// Transform a point from local to world coordinates.
    pub fn local_to_world(&self, p: [f64; 3]) -> [f64; 3] {
        [
            self.origin[0] + p[0]*self.x_dir[0] + p[1]*self.y_dir[0] + p[2]*self.z_dir()[0],
            self.origin[1] + p[0]*self.x_dir[1] + p[1]*self.y_dir[1] + p[2]*self.z_dir()[1],
            self.origin[2] + p[0]*self.x_dir[2] + p[1]*self.y_dir[2] + p[2]*self.z_dir()[2],
        ]
    }

    /// Compose: apply this LCS after `other` (i.e. `self` is in `other`'s frame).
    pub fn compose(&self, other: &LocalCS) -> LocalCS {
        let origin = other.local_to_world(self.origin);
        let x = other.local_to_world(add3(self.origin, self.x_dir));
        let y = other.local_to_world(add3(self.origin, self.y_dir));
        LocalCS::new(origin, sub3(x, origin), sub3(y, origin))
    }

    /// Translate the origin.
    pub fn translated(&self, v: [f64; 3]) -> LocalCS {
        LocalCS { origin: add3(self.origin, v), ..*self }
    }
}

// occt-ref: TopLoc_Datum3D // (represents a simple placement)
#[derive(Clone, Copy, Debug)]
pub struct Datum3D {
    pub lcs: LocalCS,
}

impl Datum3D {
    pub fn new(lcs: LocalCS) -> Self { Self { lcs } }
    pub fn is_identity(&self) -> bool {
        self.lcs == LocalCS::identity()
    }
}

fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();
    if len < 1e-14 { return [1.0, 0.0, 0.0]; }
    [v[0]/len, v[1]/len, v[2]/len]
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0]-b[0], a[1]-b[1], a[2]-b[2]]
}

fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0]+b[0], a[1]+b[1], a[2]+b[2]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_round_trip() {
        let lcs = LocalCS::identity();
        let p = [3.0, 4.0, 5.0];
        let local = lcs.world_to_local(p);
        let world = lcs.local_to_world(local);
        assert!((world[0]-p[0]).abs() < 1e-10);
        assert!((world[1]-p[1]).abs() < 1e-10);
        assert!((world[2]-p[2]).abs() < 1e-10);
    }

    #[test]
    fn lcs_z_dir() {
        let lcs = LocalCS::identity();
        let z = lcs.z_dir();
        assert!((z[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn lcs_world_to_local() {
        let lcs = LocalCS::new([1.0,0.0,0.0], [1.0,0.0,0.0], [0.0,1.0,0.0]);
        let local = lcs.world_to_local([2.0, 3.0, 0.0]);
        assert!((local[0] - 1.0).abs() < 1e-10);
        assert!((local[1] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn lcs_translate() {
        let lcs = LocalCS::identity();
        let moved = lcs.translated([5.0, 0.0, 0.0]);
        assert!((moved.origin[0] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn datum3d_identity() {
        let d = Datum3D::new(LocalCS::identity());
        assert!(d.is_identity());
    }
}
