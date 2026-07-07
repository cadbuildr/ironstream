// FILE: shape_prim_api.rs
// occt: BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere,
//       BRepPrimAPI_MakeCone, BRepPrimAPI_MakeTorus

/// Status returned by primitive make algorithms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimStatus {
    Done,
    NullAxis,
    BadAngle,
    NegativeDimension,
    BadParameter,
    NotDone,
}

impl Default for PrimStatus {
    fn default() -> Self { Self::NotDone }
}

impl PrimStatus {
    pub fn is_done(&self) -> bool { *self == PrimStatus::Done }
}

/// Axis placement (position and direction).
#[derive(Clone, Debug)]
pub struct PrimAxes {
    pub origin: [f64; 3],
    pub z_dir: [f64; 3],
    pub x_dir: [f64; 3],
}

impl Default for PrimAxes {
    fn default() -> Self {
        Self {
            origin: [0.0, 0.0, 0.0],
            z_dir: [0.0, 0.0, 1.0],
            x_dir: [1.0, 0.0, 0.0],
        }
    }
}

impl PrimAxes {
    pub fn new(origin: [f64; 3]) -> Self { Self { origin, ..Default::default() } }
}

/// Minimal shape descriptor produced by a primitive builder.
#[derive(Clone, Debug, Default)]
pub struct PrimShape {
    pub id: u32,
    pub nb_faces: usize,
    pub nb_edges: usize,
    pub nb_vertices: usize,
    pub bounding_box: [[f64; 3]; 2],  // [min, max]
}

static NEXT_PRIM_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

fn next_id() -> u32 {
    NEXT_PRIM_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

// occt: BRepPrimAPI_MakeBox — solid box aligned with (or placed at) an axis system
#[derive(Clone, Debug)]
pub struct MakeBox {
    pub axes: PrimAxes,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub status: PrimStatus,
}

impl MakeBox {
    pub fn new(dx: f64, dy: f64, dz: f64) -> Self {
        Self::with_axes(PrimAxes::default(), dx, dy, dz)
    }

    pub fn with_axes(axes: PrimAxes, dx: f64, dy: f64, dz: f64) -> Self {
        let status = if dx > 0.0 && dy > 0.0 && dz > 0.0 { PrimStatus::Done } else { PrimStatus::NegativeDimension };
        Self { axes, dx, dy, dz, status }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }

    pub fn shape(&self) -> Option<PrimShape> {
        if !self.is_done() { return None; }
        let o = self.axes.origin;
        Some(PrimShape {
            id: next_id(),
            nb_faces: 6,
            nb_edges: 12,
            nb_vertices: 8,
            bounding_box: [o, [o[0]+self.dx, o[1]+self.dy, o[2]+self.dz]],
        })
    }

    pub fn volume(&self) -> f64 { self.dx * self.dy * self.dz }
    pub fn surface_area(&self) -> f64 {
        2.0 * (self.dx*self.dy + self.dy*self.dz + self.dz*self.dx)
    }
}

// occt: BRepPrimAPI_MakeCylinder
#[derive(Clone, Debug)]
pub struct MakeCylinder {
    pub axes: PrimAxes,
    pub radius: f64,
    pub height: f64,
    pub angle: f64,   // sweep angle in radians (default: 2π)
    pub status: PrimStatus,
}

impl MakeCylinder {
    pub fn new(radius: f64, height: f64) -> Self {
        Self::with_angle(PrimAxes::default(), radius, height, 2.0 * std::f64::consts::PI)
    }

    pub fn partial(radius: f64, height: f64, angle: f64) -> Self {
        Self::with_angle(PrimAxes::default(), radius, height, angle)
    }

    pub fn with_angle(axes: PrimAxes, radius: f64, height: f64, angle: f64) -> Self {
        let status = if radius > 0.0 && height > 0.0 && angle > 0.0 && angle <= 2.0*std::f64::consts::PI {
            PrimStatus::Done
        } else {
            PrimStatus::NegativeDimension
        };
        Self { axes, radius, height, angle, status }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn is_partial(&self) -> bool { self.angle < 2.0 * std::f64::consts::PI - 1e-10 }

    pub fn shape(&self) -> Option<PrimShape> {
        if !self.is_done() { return None; }
        let o = self.axes.origin;
        let r = self.radius;
        let h = self.height;
        Some(PrimShape {
            id: next_id(),
            nb_faces: if self.is_partial() { 5 } else { 3 },
            nb_edges: if self.is_partial() { 9 } else { 3 },
            nb_vertices: if self.is_partial() { 6 } else { 0 },
            bounding_box: [[o[0]-r, o[1]-r, o[2]], [o[0]+r, o[1]+r, o[2]+h]],
        })
    }

    pub fn volume(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius * self.height * (self.angle / (2.0 * std::f64::consts::PI))
    }
    pub fn lateral_area(&self) -> f64 { self.radius * self.angle * self.height }
}

// occt: BRepPrimAPI_MakeSphere
#[derive(Clone, Debug)]
pub struct MakeSphere {
    pub center: [f64; 3],
    pub radius: f64,
    pub angle1: f64,   // latitude start (default: -π/2)
    pub angle2: f64,   // latitude end   (default:  π/2)
    pub angle3: f64,   // longitude sweep (default: 2π)
    pub status: PrimStatus,
}

impl MakeSphere {
    pub fn new(radius: f64) -> Self {
        Self::full(radius, [0.0, 0.0, 0.0])
    }

    pub fn full(radius: f64, center: [f64; 3]) -> Self {
        let pi = std::f64::consts::PI;
        Self {
            center,
            radius,
            angle1: -pi / 2.0,
            angle2: pi / 2.0,
            angle3: 2.0 * pi,
            status: if radius > 0.0 { PrimStatus::Done } else { PrimStatus::NegativeDimension },
        }
    }

    pub fn partial(radius: f64, center: [f64; 3], angle1: f64, angle2: f64, angle3: f64) -> Self {
        let pi = std::f64::consts::PI;
        let status = if radius > 0.0 && angle2 > angle1 && angle3 > 0.0 && angle3 <= 2.0*pi {
            PrimStatus::Done
        } else {
            PrimStatus::BadParameter
        };
        Self { center, radius, angle1, angle2, angle3, status }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }

    pub fn shape(&self) -> Option<PrimShape> {
        if !self.is_done() { return None; }
        let c = self.center;
        let r = self.radius;
        Some(PrimShape {
            id: next_id(),
            nb_faces: 1,
            nb_edges: 0,
            nb_vertices: 0,
            bounding_box: [[c[0]-r, c[1]-r, c[2]-r], [c[0]+r, c[1]+r, c[2]+r]],
        })
    }

    pub fn volume(&self) -> f64 { 4.0 / 3.0 * std::f64::consts::PI * self.radius.powi(3) }
    pub fn surface_area(&self) -> f64 { 4.0 * std::f64::consts::PI * self.radius * self.radius }
}

// occt: BRepPrimAPI_MakeCone
#[derive(Clone, Debug)]
pub struct MakeCone {
    pub axes: PrimAxes,
    pub radius1: f64,    // base radius
    pub radius2: f64,    // top radius
    pub height: f64,
    pub angle: f64,      // sweep angle
    pub status: PrimStatus,
}

impl MakeCone {
    pub fn new(r1: f64, r2: f64, height: f64) -> Self {
        Self::with_angle(PrimAxes::default(), r1, r2, height, 2.0*std::f64::consts::PI)
    }

    pub fn with_angle(axes: PrimAxes, r1: f64, r2: f64, height: f64, angle: f64) -> Self {
        let ok = r1 >= 0.0 && r2 >= 0.0 && (r1 > 1e-15 || r2 > 1e-15)
            && height > 0.0 && angle > 0.0 && angle <= 2.0*std::f64::consts::PI;
        Self { axes, radius1: r1, radius2: r2, height, angle, status: if ok { PrimStatus::Done } else { PrimStatus::NegativeDimension } }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }

    pub fn shape(&self) -> Option<PrimShape> {
        if !self.is_done() { return None; }
        let o = self.axes.origin;
        let r = self.radius1.max(self.radius2);
        Some(PrimShape {
            id: next_id(),
            // Lateral face plus one planar cap per non-degenerate radius:
            // a truncated cone has 3 faces, a pointed cone (one radius null) has 2.
            // occt: BRepPrim_Cone builds Top/Bottom faces only for non-zero radii.
            nb_faces: 1
                + usize::from(self.radius1 > 0.0)
                + usize::from(self.radius2 > 0.0),
            nb_edges: 3,
            nb_vertices: 1,
            bounding_box: [[o[0]-r, o[1]-r, o[2]], [o[0]+r, o[1]+r, o[2]+self.height]],
        })
    }

    pub fn volume(&self) -> f64 {
        let pi = std::f64::consts::PI;
        let frac = self.angle / (2.0 * pi);
        pi * self.height * (self.radius1*self.radius1 + self.radius1*self.radius2 + self.radius2*self.radius2) * frac / 3.0
    }
}

// occt: BRepPrimAPI_MakeTorus
#[derive(Clone, Debug)]
pub struct MakeTorus {
    pub axes: PrimAxes,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub angle1: f64,     // start angle along tube
    pub angle2: f64,     // end angle along tube
    pub angle3: f64,     // sweep angle around torus axis
    pub status: PrimStatus,
}

impl MakeTorus {
    pub fn new(major: f64, minor: f64) -> Self {
        let pi = std::f64::consts::PI;
        Self::partial(PrimAxes::default(), major, minor, 0.0, 2.0*pi, 2.0*pi)
    }

    pub fn partial(axes: PrimAxes, major: f64, minor: f64, a1: f64, a2: f64, a3: f64) -> Self {
        let pi = std::f64::consts::PI;
        let ok = major > 0.0 && minor > 0.0 && minor < major && a3 > 0.0 && a3 <= 2.0*pi;
        Self {
            axes, major_radius: major, minor_radius: minor,
            angle1: a1, angle2: a2, angle3: a3,
            status: if ok { PrimStatus::Done } else { PrimStatus::NegativeDimension },
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }

    pub fn shape(&self) -> Option<PrimShape> {
        if !self.is_done() { return None; }
        let o = self.axes.origin;
        let r = self.major_radius + self.minor_radius;
        Some(PrimShape {
            id: next_id(),
            nb_faces: 1,
            nb_edges: 2,
            nb_vertices: 0,
            bounding_box: [[o[0]-r, o[1]-r, o[2]-self.minor_radius], [o[0]+r, o[1]+r, o[2]+self.minor_radius]],
        })
    }

    pub fn volume(&self) -> f64 {
        2.0 * std::f64::consts::PI * std::f64::consts::PI * self.major_radius * self.minor_radius * self.minor_radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_box_valid() {
        let b = MakeBox::new(2.0, 3.0, 4.0);
        assert!(b.is_done());
        assert!((b.volume() - 24.0).abs() < 1e-10);
        assert!((b.surface_area() - 52.0).abs() < 1e-10);
        let s = b.shape().unwrap();
        assert_eq!(s.nb_faces, 6);
    }

    #[test]
    fn make_box_invalid() {
        let b = MakeBox::new(-1.0, 3.0, 4.0);
        assert!(!b.is_done());
        assert!(b.shape().is_none());
    }

    #[test]
    fn make_cylinder_volume() {
        let c = MakeCylinder::new(1.0, 2.0);
        assert!(c.is_done());
        let vol = c.volume();
        assert!((vol - std::f64::consts::PI * 2.0).abs() < 1e-8);
        assert!(!c.is_partial());
    }

    #[test]
    fn make_sphere_volume() {
        let s = MakeSphere::new(1.0);
        assert!(s.is_done());
        let v = s.volume();
        assert!((v - 4.0/3.0 * std::f64::consts::PI).abs() < 1e-8);
    }

    #[test]
    fn make_cone_valid() {
        let c = MakeCone::new(2.0, 0.0, 3.0);
        assert!(c.is_done());
        let s = c.shape().unwrap();
        assert_eq!(s.nb_faces, 2);
    }

    #[test]
    fn make_torus_volume() {
        let t = MakeTorus::new(3.0, 1.0);
        assert!(t.is_done());
        let v = t.volume();
        let expected = 2.0 * std::f64::consts::PI * std::f64::consts::PI * 3.0;
        assert!((v - expected).abs() < 1e-8);
    }
}
