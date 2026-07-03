// FILE: geom_tube.rs
// occt: BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeTorus

use std::f64::consts::PI;

// occt: BRepPrimAPI_MakeCylinder
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cylinder {
    pub base_center: [f64; 3],
    pub axis: [f64; 3],
    pub radius: f64,
    pub height: f64,
    pub angle: f64,
}

impl Cylinder {
    pub fn new(base_center: [f64; 3], radius: f64, height: f64) -> Self {
        assert!(radius > 0.0 && height > 0.0);
        Self { base_center, axis: [0.0, 0.0, 1.0], radius, height, angle: 2.0 * PI }
    }

    pub fn with_angle(mut self, angle: f64) -> Self {
        self.angle = angle.clamp(0.0, 2.0 * PI);
        self
    }

    pub fn volume(&self) -> f64 {
        PI * self.radius * self.radius * self.height * (self.angle / (2.0 * PI))
    }

    pub fn lateral_area(&self) -> f64 {
        self.radius * self.angle * self.height
    }

    pub fn top_center(&self) -> [f64; 3] {
        [
            self.base_center[0] + self.axis[0] * self.height,
            self.base_center[1] + self.axis[1] * self.height,
            self.base_center[2] + self.axis[2] * self.height,
        ]
    }
}

// occt: BRepPrimAPI_MakeTorus
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Torus {
    pub center: [f64; 3],
    pub axis: [f64; 3],
    pub major_radius: f64,
    pub minor_radius: f64,
    pub angle1: f64,
    pub angle2: f64,
}

impl Torus {
    pub fn new(center: [f64; 3], major_radius: f64, minor_radius: f64) -> Self {
        assert!(major_radius > 0.0 && minor_radius > 0.0);
        assert!(major_radius > minor_radius);
        Self {
            center,
            axis: [0.0, 0.0, 1.0],
            major_radius,
            minor_radius,
            angle1: 2.0 * PI,
            angle2: 2.0 * PI,
        }
    }

    pub fn volume(&self) -> f64 {
        2.0 * PI * PI * self.major_radius * self.minor_radius * self.minor_radius
            * (self.angle1 / (2.0 * PI)) * (self.angle2 / (2.0 * PI))
    }

    pub fn surface_area(&self) -> f64 {
        4.0 * PI * PI * self.major_radius * self.minor_radius
            * (self.angle1 / (2.0 * PI)) * (self.angle2 / (2.0 * PI))
    }

    pub fn point(&self, u: f64, v: f64) -> [f64; 3] {
        let r = self.major_radius + self.minor_radius * v.cos();
        [
            self.center[0] + r * u.cos(),
            self.center[1] + r * u.sin(),
            self.center[2] + self.minor_radius * v.sin(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cylinder_volume() {
        let c = Cylinder::new([0.0, 0.0, 0.0], 1.0, 1.0);
        assert!((c.volume() - PI).abs() < 1e-10);
    }

    #[test]
    fn cylinder_top_center() {
        let c = Cylinder::new([0.0, 0.0, 0.0], 1.0, 5.0);
        let top = c.top_center();
        assert!((top[2] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn torus_surface_area() {
        let t = Torus::new([0.0, 0.0, 0.0], 3.0, 1.0);
        let expected = 4.0 * PI * PI * 3.0 * 1.0;
        assert!((t.surface_area() - expected).abs() < 1e-6);
    }

    #[test]
    fn torus_point_on_equator() {
        let t = Torus::new([0.0, 0.0, 0.0], 3.0, 1.0);
        let p = t.point(0.0, 0.0);
        assert!((p[0] - 4.0).abs() < 1e-10);
    }
}
