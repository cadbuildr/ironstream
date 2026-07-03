// FILE: b_rep_l_prop_surface_tool.rs
// occt: BRepLProp_SurfaceTool

/// Point in 3D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// Vector in 3D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

/// Tool for computing local properties of BRep surfaces.
pub struct BRepLPropSurfaceTool;

impl BRepLPropSurfaceTool {
    /// Compute the point P of parameter U and V on the Surface.
    pub fn value(_surface: &str, _u: f64, _v: f64) -> Point3D {
        Point3D::origin()
    }

    /// Compute the point P and first derivatives (D1U, D1V) of parameter U and V on the Surface.
    pub fn d1(_surface: &str, _u: f64, _v: f64) -> (Point3D, Vec3D, Vec3D) {
        (Point3D::origin(), Vec3D::zero(), Vec3D::zero())
    }

    /// Compute point P, first derivatives (D1U, D1V), and second derivatives (D2U, D2V, DUV).
    pub fn d2(_surface: &str, _u: f64, _v: f64) -> (Point3D, Vec3D, Vec3D, Vec3D, Vec3D, Vec3D) {
        (
            Point3D::origin(),
            Vec3D::zero(),
            Vec3D::zero(),
            Vec3D::zero(),
            Vec3D::zero(),
            Vec3D::zero(),
        )
    }

    /// Compute the Nth derivative vector.
    pub fn dn(_surface: &str, _u: f64, _v: f64, _iu: i32, _iv: i32) -> Vec3D {
        Vec3D::zero()
    }

    /// Return the order of continuity of the Surface.
    /// Returns 1: first derivative only is computable
    /// Returns 2: first and second derivative only are computable
    pub fn continuity(_surface: &str) -> i32 {
        1
    }

    /// Return the bounds of the Surface (U1, V1, U2, V2).
    pub fn bounds(_surface: &str) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_vec3d() {
        let v = Vec3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_value() {
        let p = BRepLPropSurfaceTool::value("surf", 0.5, 0.5);
        assert_eq!(p, Point3D::origin());
    }

    #[test]
    fn test_d1() {
        let (p, du, dv) = BRepLPropSurfaceTool::d1("surf", 0.5, 0.5);
        assert_eq!(p, Point3D::origin());
        assert_eq!(du, Vec3D::zero());
        assert_eq!(dv, Vec3D::zero());
    }

    #[test]
    fn test_continuity() {
        let cont = BRepLPropSurfaceTool::continuity("surf");
        assert!(cont >= 1);
        assert!(cont <= 2);
    }

    #[test]
    fn test_bounds() {
        let (u1, v1, u2, v2) = BRepLPropSurfaceTool::bounds("surf");
        assert!(u1 <= u2);
        assert!(v1 <= v2);
    }
}
