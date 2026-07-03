// FILE: gce_make.rs
// occt: gce_MakeAx1, gce_MakeAx2, gce_MakeCirc, gce_MakeLin, gce_MakePln

/// Return status for gce construction algorithms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceMakeStatus {
    Done,
    NullAxis,
    NullNormal,
    BadRadius,
    ColinearPoints,
    IntersectionError,
    ConfusedPoints,
    NegativeRadius,
    BadAngle,
    NullVector,
    Error,
}

impl Default for GceMakeStatus {
    fn default() -> Self { Self::Error }
}

impl GceMakeStatus {
    pub fn is_done(&self) -> bool { *self == GceMakeStatus::Done }
    pub fn message(&self) -> &'static str {
        match self {
            Self::Done => "Done",
            Self::NullAxis => "Null axis",
            Self::NullNormal => "Null normal",
            Self::BadRadius => "Bad radius",
            Self::ColinearPoints => "Colinear points",
            Self::IntersectionError => "Intersection error",
            Self::ConfusedPoints => "Confused points",
            Self::NegativeRadius => "Negative radius",
            Self::BadAngle => "Bad angle",
            Self::NullVector => "Null vector",
            Self::Error => "Error",
        }
    }
}

// occt: gce_MakeAx1 — constructs a gp_Ax1 (axis = point + direction)
#[derive(Clone, Debug)]
pub struct GceMakeAx1 {
    pub origin: [f64; 3],
    pub direction: [f64; 3],
    pub status: GceMakeStatus,
}

impl GceMakeAx1 {
    pub fn from_point_dir(point: [f64; 3], dir: [f64; 3]) -> Self {
        let mag = (dir[0]*dir[0] + dir[1]*dir[1] + dir[2]*dir[2]).sqrt();
        if mag < 1e-15 {
            return Self { origin: point, direction: [0.0, 0.0, 1.0], status: GceMakeStatus::NullVector };
        }
        let d = [dir[0]/mag, dir[1]/mag, dir[2]/mag];
        Self { origin: point, direction: d, status: GceMakeStatus::Done }
    }

    pub fn from_two_points(p1: [f64; 3], p2: [f64; 3]) -> Self {
        let dir = [p2[0]-p1[0], p2[1]-p1[1], p2[2]-p1[2]];
        Self::from_point_dir(p1, dir)
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn origin(&self) -> [f64; 3] { self.origin }
    pub fn direction(&self) -> [f64; 3] { self.direction }
}

// occt: gce_MakeAx2 — constructs a gp_Ax2 (right-handed coordinate system)
#[derive(Clone, Debug)]
pub struct GceMakeAx2 {
    pub origin: [f64; 3],
    pub z_dir: [f64; 3],
    pub x_dir: [f64; 3],
    pub status: GceMakeStatus,
}

impl GceMakeAx2 {
    pub fn from_origin_normal(origin: [f64; 3], normal: [f64; 3]) -> Self {
        let mag = (normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2]).sqrt();
        if mag < 1e-15 {
            return Self { origin, z_dir: [0.0, 0.0, 1.0], x_dir: [1.0, 0.0, 0.0], status: GceMakeStatus::NullNormal };
        }
        let z = [normal[0]/mag, normal[1]/mag, normal[2]/mag];
        // Find an x_dir orthogonal to z
        let x = if z[0].abs() < 0.9 {
            let v = [1.0_f64, 0.0, 0.0];
            let dot = v[0]*z[0] + v[1]*z[1] + v[2]*z[2];
            let raw = [v[0]-dot*z[0], v[1]-dot*z[1], v[2]-dot*z[2]];
            let m = (raw[0]*raw[0] + raw[1]*raw[1] + raw[2]*raw[2]).sqrt();
            [raw[0]/m, raw[1]/m, raw[2]/m]
        } else {
            let v = [0.0_f64, 1.0, 0.0];
            let dot = v[0]*z[0] + v[1]*z[1] + v[2]*z[2];
            let raw = [v[0]-dot*z[0], v[1]-dot*z[1], v[2]-dot*z[2]];
            let m = (raw[0]*raw[0] + raw[1]*raw[1] + raw[2]*raw[2]).sqrt();
            [raw[0]/m, raw[1]/m, raw[2]/m]
        };
        Self { origin, z_dir: z, x_dir: x, status: GceMakeStatus::Done }
    }

    pub fn from_origin_z_x(origin: [f64; 3], z_dir: [f64; 3], x_dir: [f64; 3]) -> Self {
        let mz = (z_dir[0]*z_dir[0] + z_dir[1]*z_dir[1] + z_dir[2]*z_dir[2]).sqrt();
        let mx = (x_dir[0]*x_dir[0] + x_dir[1]*x_dir[1] + x_dir[2]*x_dir[2]).sqrt();
        if mz < 1e-15 || mx < 1e-15 {
            return Self { origin, z_dir: [0.0,0.0,1.0], x_dir: [1.0,0.0,0.0], status: GceMakeStatus::NullAxis };
        }
        let z = [z_dir[0]/mz, z_dir[1]/mz, z_dir[2]/mz];
        let xn = [x_dir[0]/mx, x_dir[1]/mx, x_dir[2]/mx];
        // Project xn onto the plane normal to z
        let dot = xn[0]*z[0] + xn[1]*z[1] + xn[2]*z[2];
        let xp = [xn[0]-dot*z[0], xn[1]-dot*z[1], xn[2]-dot*z[2]];
        let m = (xp[0]*xp[0] + xp[1]*xp[1] + xp[2]*xp[2]).sqrt();
        if m < 1e-15 {
            return Self { origin, z_dir: z, x_dir: [1.0,0.0,0.0], status: GceMakeStatus::NullAxis };
        }
        Self { origin, z_dir: z, x_dir: [xp[0]/m, xp[1]/m, xp[2]/m], status: GceMakeStatus::Done }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn y_dir(&self) -> [f64; 3] {
        // y = z × x
        [
            self.z_dir[1]*self.x_dir[2] - self.z_dir[2]*self.x_dir[1],
            self.z_dir[2]*self.x_dir[0] - self.z_dir[0]*self.x_dir[2],
            self.z_dir[0]*self.x_dir[1] - self.z_dir[1]*self.x_dir[0],
        ]
    }
}

// occt: gce_MakeCirc — constructs a 3D circle from center, normal and radius
#[derive(Clone, Debug)]
pub struct GceMakeCirc {
    pub center: [f64; 3],
    pub normal: [f64; 3],
    pub radius: f64,
    pub status: GceMakeStatus,
}

impl GceMakeCirc {
    pub fn from_center_normal_radius(center: [f64; 3], normal: [f64; 3], radius: f64) -> Self {
        if radius < 0.0 {
            return Self { center, normal, radius, status: GceMakeStatus::NegativeRadius };
        }
        let mag = (normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2]).sqrt();
        if mag < 1e-15 {
            return Self { center, normal, radius, status: GceMakeStatus::NullNormal };
        }
        let n = [normal[0]/mag, normal[1]/mag, normal[2]/mag];
        Self { center, normal: n, radius, status: GceMakeStatus::Done }
    }

    pub fn from_three_points(p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> Self {
        // Compute circumcenter
        let a = [p2[0]-p1[0], p2[1]-p1[1], p2[2]-p1[2]];
        let b = [p3[0]-p1[0], p3[1]-p1[1], p3[2]-p1[2]];
        // normal = a × b
        let n = [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]];
        let mn = (n[0]*n[0] + n[1]*n[1] + n[2]*n[2]).sqrt();
        if mn < 1e-15 {
            return Self { center: p1, normal: [0.0,0.0,1.0], radius: 0.0, status: GceMakeStatus::ColinearPoints };
        }
        let nn = [n[0]/mn, n[1]/mn, n[2]/mn];
        let aa = a[0]*a[0]+a[1]*a[1]+a[2]*a[2];
        let bb = b[0]*b[0]+b[1]*b[1]+b[2]*b[2];
        let ab = a[0]*b[0]+a[1]*b[1]+a[2]*b[2];
        let denom = 2.0*(aa*bb - ab*ab);
        if denom.abs() < 1e-15 {
            return Self { center: p1, normal: nn, radius: 0.0, status: GceMakeStatus::ColinearPoints };
        }
        let s = (bb*(aa-ab)) / denom;
        let t = (aa*(bb-ab)) / denom;
        let center = [p1[0]+s*a[0]+t*b[0], p1[1]+s*a[1]+t*b[1], p1[2]+s*a[2]+t*b[2]];
        let d = [center[0]-p1[0], center[1]-p1[1], center[2]-p1[2]];
        let radius = (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt();
        Self { center, normal: nn, radius, status: GceMakeStatus::Done }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn circumference(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
    pub fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

// occt: gce_MakeLin — constructs a 3D line from a point and direction
#[derive(Clone, Debug)]
pub struct GceMakeLin {
    pub origin: [f64; 3],
    pub direction: [f64; 3],
    pub status: GceMakeStatus,
}

impl GceMakeLin {
    pub fn from_point_dir(origin: [f64; 3], dir: [f64; 3]) -> Self {
        let mag = (dir[0]*dir[0] + dir[1]*dir[1] + dir[2]*dir[2]).sqrt();
        if mag < 1e-15 {
            return Self { origin, direction: [0.0,0.0,1.0], status: GceMakeStatus::NullVector };
        }
        Self { origin, direction: [dir[0]/mag, dir[1]/mag, dir[2]/mag], status: GceMakeStatus::Done }
    }

    pub fn from_two_points(p1: [f64; 3], p2: [f64; 3]) -> Self {
        let d = [p2[0]-p1[0], p2[1]-p1[1], p2[2]-p1[2]];
        let mag = (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt();
        if mag < 1e-15 {
            return Self { origin: p1, direction: [0.0,0.0,1.0], status: GceMakeStatus::ConfusedPoints };
        }
        Self { origin: p1, direction: [d[0]/mag, d[1]/mag, d[2]/mag], status: GceMakeStatus::Done }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }

    pub fn point_at(&self, t: f64) -> [f64; 3] {
        [
            self.origin[0] + t * self.direction[0],
            self.origin[1] + t * self.direction[1],
            self.origin[2] + t * self.direction[2],
        ]
    }

    pub fn distance_to_point(&self, p: [f64; 3]) -> f64 {
        let v = [p[0]-self.origin[0], p[1]-self.origin[1], p[2]-self.origin[2]];
        let dot = v[0]*self.direction[0] + v[1]*self.direction[1] + v[2]*self.direction[2];
        let proj = [dot*self.direction[0], dot*self.direction[1], dot*self.direction[2]];
        let diff = [v[0]-proj[0], v[1]-proj[1], v[2]-proj[2]];
        (diff[0]*diff[0]+diff[1]*diff[1]+diff[2]*diff[2]).sqrt()
    }
}

// occt: gce_MakePln — constructs a 3D plane from a point and normal
#[derive(Clone, Debug)]
pub struct GceMakePln {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub status: GceMakeStatus,
}

impl GceMakePln {
    pub fn from_point_normal(origin: [f64; 3], normal: [f64; 3]) -> Self {
        let mag = (normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2]).sqrt();
        if mag < 1e-15 {
            return Self { origin, normal: [0.0,0.0,1.0], status: GceMakeStatus::NullNormal };
        }
        let n = [normal[0]/mag, normal[1]/mag, normal[2]/mag];
        Self { origin, normal: n, status: GceMakeStatus::Done }
    }

    pub fn from_three_points(p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> Self {
        let a = [p2[0]-p1[0], p2[1]-p1[1], p2[2]-p1[2]];
        let b = [p3[0]-p1[0], p3[1]-p1[1], p3[2]-p1[2]];
        let n = [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]];
        Self::from_point_normal(p1, n)
    }

    pub fn from_equation(a: f64, b: f64, c: f64, d: f64) -> Self {
        let mag = (a*a+b*b+c*c).sqrt();
        if mag < 1e-15 {
            return Self { origin: [0.0,0.0,0.0], normal: [0.0,0.0,1.0], status: GceMakeStatus::NullNormal };
        }
        // ax+by+cz+d=0 => origin = closest point to origin on plane
        let t = -d / (a*a+b*b+c*c);
        let origin = [a*t, b*t, c*t];
        Self { origin, normal: [a/mag, b/mag, c/mag], status: GceMakeStatus::Done }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }

    pub fn distance_to_point(&self, p: [f64; 3]) -> f64 {
        let v = [p[0]-self.origin[0], p[1]-self.origin[1], p[2]-self.origin[2]];
        (v[0]*self.normal[0] + v[1]*self.normal[1] + v[2]*self.normal[2]).abs()
    }

    pub fn contains_point(&self, p: [f64; 3], tol: f64) -> bool {
        self.distance_to_point(p) <= tol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_ax1_from_points() {
        let ax = GceMakeAx1::from_two_points([0.0,0.0,0.0], [0.0,0.0,1.0]);
        assert!(ax.is_done());
        assert!((ax.direction()[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn make_ax1_null_vector() {
        let ax = GceMakeAx1::from_point_dir([0.0,0.0,0.0], [0.0,0.0,0.0]);
        assert!(!ax.is_done());
        assert_eq!(ax.status, GceMakeStatus::NullVector);
    }

    #[test]
    fn make_circ_from_center() {
        let c = GceMakeCirc::from_center_normal_radius([0.0,0.0,0.0], [0.0,0.0,1.0], 5.0);
        assert!(c.is_done());
        assert!((c.radius - 5.0).abs() < 1e-10);
        assert!((c.area() - std::f64::consts::PI*25.0).abs() < 1e-8);
    }

    #[test]
    fn make_circ_negative_radius() {
        let c = GceMakeCirc::from_center_normal_radius([0.0,0.0,0.0], [0.0,0.0,1.0], -1.0);
        assert!(!c.is_done());
        assert_eq!(c.status, GceMakeStatus::NegativeRadius);
    }

    #[test]
    fn make_lin_from_two_points() {
        let l = GceMakeLin::from_two_points([0.0,0.0,0.0], [3.0,4.0,0.0]);
        assert!(l.is_done());
        let pt = l.point_at(5.0);
        assert!((pt[0] - 3.0).abs() < 1e-10);
        assert!((pt[1] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn make_lin_distance() {
        let l = GceMakeLin::from_point_dir([0.0,0.0,0.0], [1.0,0.0,0.0]);
        let d = l.distance_to_point([0.0,3.0,4.0]);
        assert!((d - 5.0).abs() < 1e-10);
    }

    #[test]
    fn make_pln_from_three_points() {
        let p = GceMakePln::from_three_points([0.0,0.0,0.0], [1.0,0.0,0.0], [0.0,1.0,0.0]);
        assert!(p.is_done());
        assert!((p.normal[2].abs() - 1.0).abs() < 1e-10);
        assert!(p.contains_point([0.5,0.5,0.0], 1e-10));
    }

    #[test]
    fn make_pln_distance() {
        let p = GceMakePln::from_point_normal([0.0,0.0,1.0], [0.0,0.0,1.0]);
        assert!(p.is_done());
        assert!((p.distance_to_point([5.0,3.0,4.0]) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn make_ax2_orthogonal() {
        let ax2 = GceMakeAx2::from_origin_normal([0.0,0.0,0.0], [0.0,0.0,1.0]);
        assert!(ax2.is_done());
        let dot = ax2.z_dir[0]*ax2.x_dir[0] + ax2.z_dir[1]*ax2.x_dir[1] + ax2.z_dir[2]*ax2.x_dir[2];
        assert!(dot.abs() < 1e-10);
    }
}
