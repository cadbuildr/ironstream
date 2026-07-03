// FILE: primitive_shape.rs
// occt: Primitive_Box, Primitive_Cylinder, Primitive_Cone, Primitive_Sphere, Primitive_Torus

/// Primitive box shape.
/// occt: Primitive_Box
#[derive(Clone, Debug)]
pub struct PrimitiveBox {
    pub origin: [f64; 3],
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

impl PrimitiveBox {
    pub fn new(origin: [f64; 3], l: f64, w: f64, h: f64) -> Self {
        Self {
            origin,
            length: l.max(0.001),
            width: w.max(0.001),
            height: h.max(0.001),
        }
    }

    pub fn volume(&self) -> f64 { self.length * self.width * self.height }
    pub fn surface_area(&self) -> f64 {
        2.0 * (self.length * self.width + self.width * self.height + self.height * self.length)
    }
}

impl Default for PrimitiveBox {
    fn default() -> Self {
        Self::new([0.0, 0.0, 0.0], 1.0, 1.0, 1.0)
    }
}

/// Primitive cylinder.
/// occt: Primitive_Cylinder
#[derive(Clone, Debug)]
pub struct PrimitiveCylinder {
    pub origin: [f64; 3],
    pub axis: [f64; 3],
    pub radius: f64,
    pub height: f64,
}

impl PrimitiveCylinder {
    pub fn new(origin: [f64; 3], axis: [f64; 3], r: f64, h: f64) -> Self {
        Self {
            origin,
            axis,
            radius: r.max(0.001),
            height: h.max(0.001),
        }
    }

    pub fn volume(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius * self.height }
    pub fn lateral_area(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius * self.height }
}

impl Default for PrimitiveCylinder {
    fn default() -> Self {
        Self::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0, 1.0)
    }
}

/// Primitive cone.
/// occt: Primitive_Cone
#[derive(Clone, Debug)]
pub struct PrimitiveCone {
    pub apex: [f64; 3],
    pub axis: [f64; 3],
    pub half_angle: f64,
    pub height: f64,
}

impl PrimitiveCone {
    pub fn new(apex: [f64; 3], axis: [f64; 3], angle: f64, h: f64) -> Self {
        Self {
            apex,
            axis,
            half_angle: angle.max(0.01).min(std::f64::consts::PI / 2.0 - 0.01),
            height: h.max(0.001),
        }
    }

    pub fn base_radius(&self) -> f64 { self.height * self.half_angle.tan() }
    pub fn volume(&self) -> f64 {
        let r = self.base_radius();
        (1.0 / 3.0) * std::f64::consts::PI * r * r * self.height
    }
}

impl Default for PrimitiveCone {
    fn default() -> Self {
        Self::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.3, 1.0)
    }
}

/// Primitive sphere.
/// occt: Primitive_Sphere
#[derive(Clone, Debug)]
pub struct PrimitiveSphere {
    pub center: [f64; 3],
    pub radius: f64,
}

impl PrimitiveSphere {
    pub fn new(center: [f64; 3], r: f64) -> Self {
        Self {
            center,
            radius: r.max(0.001),
        }
    }

    pub fn volume(&self) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * self.radius * self.radius * self.radius
    }
    pub fn surface_area(&self) -> f64 { 4.0 * std::f64::consts::PI * self.radius * self.radius }
}

impl Default for PrimitiveSphere {
    fn default() -> Self {
        Self::new([0.0, 0.0, 0.0], 1.0)
    }
}

/// Primitive torus.
/// occt: Primitive_Torus
#[derive(Clone, Debug)]
pub struct PrimitiveTorus {
    pub center: [f64; 3],
    pub major_radius: f64,
    pub minor_radius: f64,
}

impl PrimitiveTorus {
    pub fn new(center: [f64; 3], major: f64, minor: f64) -> Self {
        Self {
            center,
            major_radius: major.max(0.001),
            minor_radius: minor.max(0.001),
        }
    }

    pub fn volume(&self) -> f64 {
        let r = self.minor_radius;
        let R = self.major_radius;
        4.0 * std::f64::consts::PI * std::f64::consts::PI * R * r * r
    }
}

impl Default for PrimitiveTorus {
    fn default() -> Self {
        Self::new([0.0, 0.0, 0.0], 2.0, 0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_box() {
        let b = PrimitiveBox::new([0.0, 0.0, 0.0], 2.0, 3.0, 4.0);
        assert_eq!(b.volume(), 24.0);
        assert_eq!(b.surface_area(), 52.0);
    }

    #[test]
    fn primitive_cylinder() {
        let c = PrimitiveCylinder::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0, 2.0);
        assert!(c.volume() > 0.0);
        assert!(c.lateral_area() > 0.0);
    }

    #[test]
    fn primitive_cone() {
        let co = PrimitiveCone::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.3, 1.0);
        assert!(co.base_radius() > 0.0);
        assert!(co.volume() > 0.0);
    }

    #[test]
    fn primitive_sphere() {
        let s = PrimitiveSphere::new([0.0, 0.0, 0.0], 1.0);
        assert!(s.volume() > 0.0);
        assert!(s.surface_area() > 0.0);
    }

    #[test]
    fn primitive_torus() {
        let t = PrimitiveTorus::new([0.0, 0.0, 0.0], 2.0, 0.5);
        assert!(t.volume() > 0.0);
    }
}
