// FILE: brep_prim_ext.rs
// occt: BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere,
//       BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeTorus

use std::f64::consts::PI;

/// Status of a BRepPrim build operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrepPrimStatus {
    NotDone,
    Done,
    Error,
}

impl Default for BrepPrimStatus {
    fn default() -> Self { Self::NotDone }
}

impl BrepPrimStatus {
    pub fn is_done(&self) -> bool { *self == BrepPrimStatus::Done }
}

/// Corner point for the primitive (axis-aligned position).
#[derive(Clone, Copy, Debug, Default)]
pub struct PrimCorner {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PrimCorner {
    pub fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
}

// occt: BRepPrimAPI_MakeBox — axis-aligned box
#[derive(Clone, Debug)]
pub struct MakeBox {
    pub corner: PrimCorner,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub status: BrepPrimStatus,
    pub shape_id: u32,
}

impl MakeBox {
    pub fn new(dx: f64, dy: f64, dz: f64) -> Self {
        let ok = dx > 0.0 && dy > 0.0 && dz > 0.0;
        Self {
            corner: PrimCorner::default(),
            dx, dy, dz,
            status: if ok { BrepPrimStatus::Done } else { BrepPrimStatus::Error },
            shape_id: if ok { 1001 } else { 0 },
        }
    }

    pub fn with_corner(corner: PrimCorner, dx: f64, dy: f64, dz: f64) -> Self {
        let mut b = Self::new(dx, dy, dz);
        b.corner = corner;
        b
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.shape_id }
    pub fn volume(&self) -> f64 { self.dx * self.dy * self.dz }

    pub fn nb_faces(&self) -> usize { if self.is_done() { 6 } else { 0 } }
    pub fn nb_edges(&self) -> usize { if self.is_done() { 12 } else { 0 } }
    pub fn nb_vertices(&self) -> usize { if self.is_done() { 8 } else { 0 } }

    /// Returns the face shape-id for the i-th face (1-6: Zmin,Zmax,Xmin,Xmax,Ymin,Ymax).
    pub fn face(&self, i: usize) -> Option<u32> {
        if !self.is_done() || i == 0 || i > 6 { None } else { Some(self.shape_id + i as u32) }
    }
}

// occt: BRepPrimAPI_MakeSphere
#[derive(Clone, Debug)]
pub struct MakeSphere {
    pub center: [f64; 3],
    pub radius: f64,
    pub angle1: f64,  // latitude start
    pub angle2: f64,  // latitude end
    pub angle3: f64,  // longitude sweep
    pub status: BrepPrimStatus,
    pub shape_id: u32,
}

impl MakeSphere {
    pub fn new(radius: f64) -> Self {
        let ok = radius > 0.0;
        Self {
            center: [0.0; 3],
            radius,
            angle1: -PI / 2.0,
            angle2: PI / 2.0,
            angle3: 2.0 * PI,
            status: if ok { BrepPrimStatus::Done } else { BrepPrimStatus::Error },
            shape_id: if ok { 2001 } else { 0 },
        }
    }

    pub fn with_angles(radius: f64, angle1: f64, angle2: f64, angle3: f64) -> Self {
        let mut s = Self::new(radius);
        s.angle1 = angle1;
        s.angle2 = angle2;
        s.angle3 = angle3;
        s
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.shape_id }
    pub fn volume(&self) -> f64 {
        4.0 / 3.0 * PI * self.radius.powi(3)
    }
    pub fn surface_area(&self) -> f64 { 4.0 * PI * self.radius.powi(2) }
}

// occt: BRepPrimAPI_MakeCylinder
#[derive(Clone, Debug)]
pub struct MakeCylinder {
    pub radius: f64,
    pub height: f64,
    pub angle: f64,   // sweep angle
    pub status: BrepPrimStatus,
    pub shape_id: u32,
}

impl MakeCylinder {
    pub fn new(radius: f64, height: f64) -> Self {
        let ok = radius > 0.0 && height > 0.0;
        Self {
            radius, height,
            angle: 2.0 * PI,
            status: if ok { BrepPrimStatus::Done } else { BrepPrimStatus::Error },
            shape_id: if ok { 3001 } else { 0 },
        }
    }

    pub fn with_angle(radius: f64, height: f64, angle: f64) -> Self {
        let mut c = Self::new(radius, height);
        c.angle = angle;
        c
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.shape_id }
    pub fn volume(&self) -> f64 { PI * self.radius.powi(2) * self.height * (self.angle / (2.0 * PI)) }
    pub fn lateral_area(&self) -> f64 { self.radius * self.angle * self.height }
}

// occt: BRepPrimAPI_MakeCone
#[derive(Clone, Debug)]
pub struct MakeCone {
    pub radius1: f64,  // base radius
    pub radius2: f64,  // top radius
    pub height: f64,
    pub angle: f64,
    pub status: BrepPrimStatus,
    pub shape_id: u32,
}

impl MakeCone {
    pub fn new(radius1: f64, radius2: f64, height: f64) -> Self {
        let ok = height > 0.0 && (radius1 >= 0.0) && (radius2 >= 0.0) && (radius1 + radius2 > 0.0);
        Self {
            radius1, radius2, height,
            angle: 2.0 * PI,
            status: if ok { BrepPrimStatus::Done } else { BrepPrimStatus::Error },
            shape_id: if ok { 4001 } else { 0 },
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.shape_id }
    pub fn volume(&self) -> f64 {
        let frac = self.angle / (2.0 * PI);
        PI * self.height * (self.radius1.powi(2) + self.radius1 * self.radius2 + self.radius2.powi(2)) / 3.0 * frac
    }

    pub fn slant_height(&self) -> f64 {
        ((self.radius1 - self.radius2).powi(2) + self.height.powi(2)).sqrt()
    }
}

// occt: BRepPrimAPI_MakeTorus
#[derive(Clone, Debug)]
pub struct MakeTorus {
    pub r1: f64,  // major radius
    pub r2: f64,  // minor radius
    pub angle1: f64,
    pub angle2: f64,
    pub angle3: f64,
    pub status: BrepPrimStatus,
    pub shape_id: u32,
}

impl MakeTorus {
    pub fn new(r1: f64, r2: f64) -> Self {
        let ok = r1 > 0.0 && r2 > 0.0;
        Self {
            r1, r2,
            angle1: 0.0,
            angle2: 2.0 * PI,
            angle3: 2.0 * PI,
            status: if ok { BrepPrimStatus::Done } else { BrepPrimStatus::Error },
            shape_id: if ok { 5001 } else { 0 },
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.shape_id }
    pub fn volume(&self) -> f64 { 2.0 * PI.powi(2) * self.r1 * self.r2.powi(2) }
    pub fn surface_area(&self) -> f64 { 4.0 * PI.powi(2) * self.r1 * self.r2 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_box_basic() {
        let b = MakeBox::new(2.0, 3.0, 4.0);
        assert!(b.is_done());
        assert_eq!(b.nb_faces(), 6);
        assert_eq!(b.nb_edges(), 12);
        assert!((b.volume() - 24.0).abs() < 1e-10);
        assert_eq!(b.face(1), Some(1002));
    }

    #[test]
    fn make_box_invalid() {
        let b = MakeBox::new(0.0, 3.0, 4.0);
        assert!(!b.is_done());
        assert_eq!(b.face(1), None);
    }

    #[test]
    fn make_sphere_volume() {
        let s = MakeSphere::new(1.0);
        assert!(s.is_done());
        assert!((s.volume() - 4.0 / 3.0 * PI).abs() < 1e-6);
        assert!((s.surface_area() - 4.0 * PI).abs() < 1e-6);
    }

    #[test]
    fn make_cylinder_volume() {
        let c = MakeCylinder::new(1.0, 2.0);
        assert!(c.is_done());
        assert!((c.volume() - 2.0 * PI).abs() < 1e-6);
    }

    #[test]
    fn make_cone_slant() {
        let c = MakeCone::new(1.0, 0.0, 3.0);
        assert!(c.is_done());
        assert!((c.slant_height() - (1.0f64 + 9.0).sqrt()).abs() < 1e-10);
    }

    #[test]
    fn make_torus_geometry() {
        let t = MakeTorus::new(3.0, 1.0);
        assert!(t.is_done());
        assert!((t.volume() - 2.0 * PI.powi(2) * 3.0 * 1.0).abs() < 1e-6);
    }
}
