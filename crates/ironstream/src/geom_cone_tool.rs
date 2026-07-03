// FILE: geom_cone_tool.rs
// occt: BRepPrimAPI_MakeCone, BRepPrimAPI_MakeSphere

use std::f64::consts::PI;

// occt: BRepPrimAPI_MakeCone
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cone {
    pub base_center: [f64; 3],
    pub axis: [f64; 3],
    pub radius1: f64,
    pub radius2: f64,
    pub height: f64,
    pub angle: f64,
}

impl Cone {
    pub fn new(base_center: [f64; 3], radius1: f64, radius2: f64, height: f64) -> Self {
        assert!(radius1 >= 0.0 && radius2 >= 0.0);
        assert!(height > 0.0);
        Self { base_center, axis: [0.0, 0.0, 1.0], radius1, radius2, height, angle: 2.0 * PI }
    }

    pub fn with_angle(mut self, angle: f64) -> Self {
        self.angle = angle.clamp(0.0, 2.0 * PI);
        self
    }

    pub fn volume(&self) -> f64 {
        PI * self.height * (self.angle / (2.0 * PI))
            * (self.radius1 * self.radius1 + self.radius1 * self.radius2 + self.radius2 * self.radius2)
            / 3.0
    }

    pub fn lateral_area(&self) -> f64 {
        let slant = (self.height * self.height + (self.radius1 - self.radius2).powi(2)).sqrt();
        0.5 * (self.radius1 + self.radius2) * slant * self.angle
    }
}

// occt: BRepPrimAPI_MakeSphere
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: [f64; 3],
    pub radius: f64,
    pub angle1: f64,
    pub angle2: f64,
    pub angle3: f64,
}

impl Sphere {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        assert!(radius > 0.0);
        Self { center, radius, angle1: -PI / 2.0, angle2: PI / 2.0, angle3: 2.0 * PI }
    }

    pub fn volume(&self) -> f64 {
        let lat = self.angle2.sin() - self.angle1.sin();
        (1.0 / 3.0) * self.radius.powi(3) * self.angle3 * lat
    }

    pub fn surface_area(&self) -> f64 {
        let lat = self.angle2.sin() - self.angle1.sin();
        self.radius * self.radius * self.angle3 * lat
    }

    pub fn point(&self, u: f64, v: f64) -> [f64; 3] {
        [
            self.center[0] + self.radius * v.cos() * u.cos(),
            self.center[1] + self.radius * v.cos() * u.sin(),
            self.center[2] + self.radius * v.sin(),
        ]
    }
}

// occt: BRepPrimAPI_MakeHalfSpace
#[derive(Clone, Debug)]
pub struct HalfSpace {
    pub face_center: [f64; 3],
    pub face_normal: [f64; 3],
    pub ref_point: [f64; 3],
}

impl HalfSpace {
    pub fn new(face_center: [f64; 3], face_normal: [f64; 3], ref_point: [f64; 3]) -> Self {
        Self { face_center, face_normal, ref_point }
    }

    pub fn contains(&self, p: [f64; 3]) -> bool {
        let d = [p[0] - self.face_center[0], p[1] - self.face_center[1], p[2] - self.face_center[2]];
        let dot = d[0] * self.face_normal[0] + d[1] * self.face_normal[1] + d[2] * self.face_normal[2];
        let r = [
            self.ref_point[0] - self.face_center[0],
            self.ref_point[1] - self.face_center[1],
            self.ref_point[2] - self.face_center[2],
        ];
        let ref_dot = r[0] * self.face_normal[0] + r[1] * self.face_normal[1] + r[2] * self.face_normal[2];
        dot * ref_dot > 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_volume_full() {
        let s = Sphere::new([0.0, 0.0, 0.0], 1.0);
        assert!((s.volume() - 4.0 / 3.0 * PI).abs() < 1e-6);
    }

    #[test]
    fn sphere_surface_area() {
        let s = Sphere::new([0.0, 0.0, 0.0], 1.0);
        assert!((s.surface_area() - 4.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn cone_cylinder_degenerate() {
        let c = Cone::new([0.0, 0.0, 0.0], 1.0, 1.0, 1.0);
        assert!((c.volume() - PI).abs() < 1e-10);
    }

    #[test]
    fn half_space_contains() {
        let hs = HalfSpace::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]);
        assert!(hs.contains([0.0, 0.0, 5.0]));
        assert!(!hs.contains([0.0, 0.0, -1.0]));
    }
}
