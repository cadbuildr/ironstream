// FILE: geom_geometry.rs
use crate::gp::{Pnt, Vec3, Ax1, Ax2};

// occt: Geom_Geometry
pub trait GeomGeometry {
    fn mirror_point(&self, pt: &Pnt) -> Self where Self: Sized;
    fn mirror_ax1(&self, ax: &Ax1) -> Self where Self: Sized;
    fn mirror_ax2(&self, ax: &Ax2) -> Self where Self: Sized;
    fn rotate(&self, ax: &Ax1, angle: f64) -> Self where Self: Sized;
    fn scale(&self, pt: &Pnt, s: f64) -> Self where Self: Sized;
    fn translate_vec(&self, v: &Vec3) -> Self where Self: Sized;
    fn translate_pnt(&self, from: &Pnt, to: &Pnt) -> Self where Self: Sized;
    fn copy(&self) -> Box<dyn GeomGeometry> where Self: 'static;
}

// occt: Geom_Point
#[derive(Debug, Clone, PartialEq)]
pub struct GeomPoint {
    pub pnt: Pnt,
}

impl GeomPoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GeomPoint { pnt: Pnt { x, y, z } }
    }

    pub fn from_pnt(pnt: Pnt) -> Self {
        GeomPoint { pnt }
    }

    pub fn x(&self) -> f64 { self.pnt.x }
    pub fn y(&self) -> f64 { self.pnt.y }
    pub fn z(&self) -> f64 { self.pnt.z }
}

/// Reflect a point p through mirror center pt: result = 2*pt - p
fn mirror_pnt_through_point(p: &Pnt, pt: &Pnt) -> Pnt {
    Pnt {
        x: 2.0 * pt.x - p.x,
        y: 2.0 * pt.y - p.y,
        z: 2.0 * pt.z - p.z,
    }
}

/// Reflect a point p through an axis (Ax1: location + unit direction).
/// Formula: p' = 2*(p·d - loc·d)*d + 2*loc - p
/// where d is the unit direction of the axis.
fn mirror_pnt_through_ax1(p: &Pnt, ax: &Ax1) -> Pnt {
    let loc = &ax.location;
    let d = &ax.direction;
    // translate p relative to axis location
    let rx = p.x - loc.x;
    let ry = p.y - loc.y;
    let rz = p.z - loc.z;
    // dot product with direction
    let dot = rx * d.x + ry * d.y + rz * d.z;
    // reflection: r' = 2*(r·d)*d - r
    let rx2 = 2.0 * dot * d.x - rx;
    let ry2 = 2.0 * dot * d.y - ry;
    let rz2 = 2.0 * dot * d.z - rz;
    Pnt {
        x: rx2 + loc.x,
        y: ry2 + loc.y,
        z: rz2 + loc.z,
    }
}

/// Reflect a point p through a plane defined by Ax2 (location + normal direction).
/// The normal of the plane is ax.z_dir (the main/Z direction of Ax2/Ax3).
/// Formula: p' = p - 2*(p-loc)·n * n
fn mirror_pnt_through_ax2(p: &Pnt, ax: &Ax2) -> Pnt {
    let loc = &ax.location;
    let n = &ax.z_dir;
    let rx = p.x - loc.x;
    let ry = p.y - loc.y;
    let rz = p.z - loc.z;
    let dot = rx * n.x + ry * n.y + rz * n.z;
    Pnt {
        x: p.x - 2.0 * dot * n.x,
        y: p.y - 2.0 * dot * n.y,
        z: p.z - 2.0 * dot * n.z,
    }
}

/// Rotate a point around an axis (Rodrigues' rotation formula).
fn rotate_pnt_around_ax1(p: &Pnt, ax: &Ax1, angle: f64) -> Pnt {
    let loc = &ax.location;
    let d = &ax.direction;
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    // vector from axis location to point
    let rx = p.x - loc.x;
    let ry = p.y - loc.y;
    let rz = p.z - loc.z;
    // Rodrigues: r' = r*cos + (d x r)*sin + d*(d·r)*(1-cos)
    let dot = rx * d.x + ry * d.y + rz * d.z;
    // cross product d x r
    let cx = d.y * rz - d.z * ry;
    let cy = d.z * rx - d.x * rz;
    let cz = d.x * ry - d.y * rx;
    let rx2 = rx * cos_a + cx * sin_a + d.x * dot * (1.0 - cos_a);
    let ry2 = ry * cos_a + cy * sin_a + d.y * dot * (1.0 - cos_a);
    let rz2 = rz * cos_a + cz * sin_a + d.z * dot * (1.0 - cos_a);
    Pnt {
        x: rx2 + loc.x,
        y: ry2 + loc.y,
        z: rz2 + loc.z,
    }
}

/// Scale a point from center pt by factor s.
fn scale_pnt(p: &Pnt, pt: &Pnt, s: f64) -> Pnt {
    Pnt {
        x: pt.x + s * (p.x - pt.x),
        y: pt.y + s * (p.y - pt.y),
        z: pt.z + s * (p.z - pt.z),
    }
}

impl GeomGeometry for GeomPoint {
    fn mirror_point(&self, pt: &Pnt) -> Self {
        GeomPoint { pnt: mirror_pnt_through_point(&self.pnt, pt) }
    }

    fn mirror_ax1(&self, ax: &Ax1) -> Self {
        GeomPoint { pnt: mirror_pnt_through_ax1(&self.pnt, ax) }
    }

    fn mirror_ax2(&self, ax: &Ax2) -> Self {
        GeomPoint { pnt: mirror_pnt_through_ax2(&self.pnt, ax) }
    }

    fn rotate(&self, ax: &Ax1, angle: f64) -> Self {
        GeomPoint { pnt: rotate_pnt_around_ax1(&self.pnt, ax, angle) }
    }

    fn scale(&self, pt: &Pnt, s: f64) -> Self {
        GeomPoint { pnt: scale_pnt(&self.pnt, pt, s) }
    }

    fn translate_vec(&self, v: &Vec3) -> Self {
        GeomPoint {
            pnt: Pnt {
                x: self.pnt.x + v.x,
                y: self.pnt.y + v.y,
                z: self.pnt.z + v.z,
            },
        }
    }

    fn translate_pnt(&self, from: &Pnt, to: &Pnt) -> Self {
        GeomPoint {
            pnt: Pnt {
                x: self.pnt.x + (to.x - from.x),
                y: self.pnt.y + (to.y - from.y),
                z: self.pnt.z + (to.z - from.z),
            },
        }
    }

    fn copy(&self) -> Box<dyn GeomGeometry> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Pnt, Vec3, Ax1, Ax2, Ax3};

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    fn pnt_approx_eq(p: &Pnt, x: f64, y: f64, z: f64, tol: f64) -> bool {
        approx_eq(p.x, x, tol) && approx_eq(p.y, y, tol) && approx_eq(p.z, z, tol)
    }

    fn make_ax1(lx: f64, ly: f64, lz: f64, dx: f64, dy: f64, dz: f64) -> Ax1 {
        Ax1::new(
            Pnt { x: lx, y: ly, z: lz },
            Pnt { x: dx, y: dy, z: dz },
        )
    }

    /// Build an Ax2 (= Ax3) from a location and a normal (z_dir).
    /// x_dir and y_dir are computed to form a right-handed frame.
    fn make_ax2(lx: f64, ly: f64, lz: f64, nx: f64, ny: f64, nz: f64) -> Ax2 {
        Ax3::from_origin_normal(
            Pnt { x: lx, y: ly, z: lz },
            Pnt { x: nx, y: ny, z: nz },
            Pnt { x: 1.0, y: 0.0, z: 0.0 },
        )
    }

    #[test]
    fn test_geom_point_new() {
        let gp = GeomPoint::new(1.0, 2.0, 3.0);
        assert_eq!(gp.x(), 1.0);
        assert_eq!(gp.y(), 2.0);
        assert_eq!(gp.z(), 3.0);
    }

    #[test]
    fn test_mirror_point_origin() {
        // Mirror (1,2,3) through origin -> (-1,-2,-3)
        let gp = GeomPoint::new(1.0, 2.0, 3.0);
        let origin = Pnt { x: 0.0, y: 0.0, z: 0.0 };
        let result = gp.mirror_point(&origin);
        assert!(pnt_approx_eq(&result.pnt, -1.0, -2.0, -3.0, 1e-10));
    }

    #[test]
    fn test_mirror_point_arbitrary() {
        // Mirror (3,4,5) through (1,1,1) -> (-1,-2,-3)
        let gp = GeomPoint::new(3.0, 4.0, 5.0);
        let center = Pnt { x: 1.0, y: 1.0, z: 1.0 };
        let result = gp.mirror_point(&center);
        assert!(pnt_approx_eq(&result.pnt, -1.0, -2.0, -3.0, 1e-10));
    }

    #[test]
    fn test_mirror_point_is_involution() {
        // Mirroring twice returns original
        let gp = GeomPoint::new(5.0, -3.0, 2.0);
        let center = Pnt { x: 1.0, y: 1.0, z: 1.0 };
        let result = gp.mirror_point(&center).mirror_point(&center);
        assert!(pnt_approx_eq(&result.pnt, 5.0, -3.0, 2.0, 1e-10));
    }

    #[test]
    fn test_mirror_ax1_z_axis() {
        // Mirror (3,4,0) through Z-axis at origin: x->-x, y->-y, z unchanged
        // Wait: reflection through an axis keeps the component along the axis
        // and negates the perpendicular components.
        // Point (3,4,0), Z-axis: dot with Z is 0, so r' = 2*(0)*Z - r = -r
        // Result: (-3,-4,0)
        let gp = GeomPoint::new(3.0, 4.0, 0.0);
        let z_ax = make_ax1(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let result = gp.mirror_ax1(&z_ax);
        assert!(pnt_approx_eq(&result.pnt, -3.0, -4.0, 0.0, 1e-10));
    }

    #[test]
    fn test_mirror_ax1_x_axis() {
        // Mirror (1,2,3) through X-axis at origin.
        // dot with X = 1, r=(1,2,3), r' = 2*1*(1,0,0) - (1,2,3) = (2,0,0)-(1,2,3) = (1,-2,-3)
        let gp = GeomPoint::new(1.0, 2.0, 3.0);
        let x_ax = make_ax1(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let result = gp.mirror_ax1(&x_ax);
        assert!(pnt_approx_eq(&result.pnt, 1.0, -2.0, -3.0, 1e-10));
    }

    #[test]
    fn test_mirror_ax1_is_involution() {
        let gp = GeomPoint::new(2.0, 5.0, -1.0);
        let ax = make_ax1(1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let result = gp.mirror_ax1(&ax).mirror_ax1(&ax);
        assert!(pnt_approx_eq(&result.pnt, 2.0, 5.0, -1.0, 1e-10));
    }

    #[test]
    fn test_mirror_ax2_xy_plane() {
        // Mirror (1,2,3) through XY plane (normal = Z).
        // result: (1,2,-3)
        let gp = GeomPoint::new(1.0, 2.0, 3.0);
        let xy_plane = make_ax2(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let result = gp.mirror_ax2(&xy_plane);
        assert!(pnt_approx_eq(&result.pnt, 1.0, 2.0, -3.0, 1e-10));
    }

    #[test]
    fn test_mirror_ax2_xz_plane() {
        // Mirror (1,2,3) through XZ plane (normal = Y).
        // result: (1,-2,3)
        let gp = GeomPoint::new(1.0, 2.0, 3.0);
        let xz_plane = make_ax2(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let result = gp.mirror_ax2(&xz_plane);
        assert!(pnt_approx_eq(&result.pnt, 1.0, -2.0, 3.0, 1e-10));
    }

    #[test]
    fn test_mirror_ax2_offset_plane() {
        // Mirror (0,0,0) through plane z=2 (normal Z, location (0,0,2)).
        // result: (0,0,4)
        let gp = GeomPoint::new(0.0, 0.0, 0.0);
        let plane = make_ax2(0.0, 0.0, 2.0, 0.0, 0.0, 1.0);
        let result = gp.mirror_ax2(&plane);
        assert!(pnt_approx_eq(&result.pnt, 0.0, 0.0, 4.0, 1e-10));
    }

    #[test]
    fn test_mirror_ax2_is_involution() {
        let gp = GeomPoint::new(3.0, -1.0, 7.0);
        let plane = make_ax2(1.0, 1.0, 1.0, 1.0, 1.0, 0.0);
        let result = gp.mirror_ax2(&plane).mirror_ax2(&plane);
        assert!(pnt_approx_eq(&result.pnt, 3.0, -1.0, 7.0, 1e-10));
    }

    #[test]
    fn test_rotate_z_axis_90deg() {
        // Rotate (1,0,0) 90 degrees around Z-axis -> (0,1,0)
        let gp = GeomPoint::new(1.0, 0.0, 0.0);
        let z_ax = make_ax1(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let angle = std::f64::consts::PI / 2.0;
        let result = gp.rotate(&z_ax, angle);
        assert!(pnt_approx_eq(&result.pnt, 0.0, 1.0, 0.0, 1e-10));
    }

    #[test]
    fn test_rotate_z_axis_180deg() {
        // Rotate (1,0,0) 180 degrees around Z-axis -> (-1,0,0)
        let gp = GeomPoint::new(1.0, 0.0, 0.0);
        let z_ax = make_ax1(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let angle = std::f64::consts::PI;
        let result = gp.rotate(&z_ax, angle);
        assert!(pnt_approx_eq(&result.pnt, -1.0, 0.0, 0.0, 1e-10));
    }

    #[test]
    fn test_rotate_360_is_identity() {
        let gp = GeomPoint::new(3.0, 4.0, 5.0);
        let ax = make_ax1(1.0, 0.0, 0.0, 0.0, 1.0, 1.0);
        let angle = 2.0 * std::f64::consts::PI;
        let result = gp.rotate(&ax, angle);
        assert!(pnt_approx_eq(&result.pnt, 3.0, 4.0, 5.0, 1e-10));
    }

    #[test]
    fn test_rotate_x_axis_90deg() {
        // Rotate (0,1,0) 90 degrees around X-axis -> (0,0,1)
        let gp = GeomPoint::new(0.0, 1.0, 0.0);
        let x_ax = make_ax1(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let angle = std::f64::consts::PI / 2.0;
        let result = gp.rotate(&x_ax, angle);
        assert!(pnt_approx_eq(&result.pnt, 0.0, 0.0, 1.0, 1e-10));
    }

    #[test]
    fn test_rotate_offset_axis() {
        // Rotate (2,0,0) 90 degrees around Z-axis at (1,0,0).
        // Relative to axis loc: r=(1,0,0), after 90-deg rot: r'=(0,1,0)
        // Absolute: (1,1,0)
        let gp = GeomPoint::new(2.0, 0.0, 0.0);
        let ax = make_ax1(1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let angle = std::f64::consts::PI / 2.0;
        let result = gp.rotate(&ax, angle);
        assert!(pnt_approx_eq(&result.pnt, 1.0, 1.0, 0.0, 1e-10));
    }

    #[test]
    fn test_scale_from_origin() {
        let gp = GeomPoint::new(1.0, 2.0, 3.0);
        let origin = Pnt { x: 0.0, y: 0.0, z: 0.0 };
        let result = gp.scale(&origin, 2.0);
        assert!(pnt_approx_eq(&result.pnt, 2.0, 4.0, 6.0, 1e-10));
    }

    #[test]
    fn test_scale_from_arbitrary_center() {
        // Scale (3,3,3) by 2 from (1,1,1): result = (1,1,1)+2*(2,2,2) = (5,5,5)
        let gp = GeomPoint::new(3.0, 3.0, 3.0);
        let center = Pnt { x: 1.0, y: 1.0, z: 1.0 };
        let result = gp.scale(&center, 2.0);
        assert!(pnt_approx_eq(&result.pnt, 5.0, 5.0, 5.0, 1e-10));
    }

    #[test]
    fn test_scale_zero() {
        // Scale by 0 collapses to center
        let gp = GeomPoint::new(5.0, 7.0, 9.0);
        let center = Pnt { x: 1.0, y: 2.0, z: 3.0 };
        let result = gp.scale(&center, 0.0);
        assert!(pnt_approx_eq(&result.pnt, 1.0, 2.0, 3.0, 1e-10));
    }

    #[test]
    fn test_scale_negative() {
        // Scale by -1 is point inversion through center
        let gp = GeomPoint::new(3.0, 0.0, 0.0);
        let center = Pnt { x: 1.0, y: 0.0, z: 0.0 };
        let result = gp.scale(&center, -1.0);
        assert!(pnt_approx_eq(&result.pnt, -1.0, 0.0, 0.0, 1e-10));
    }

    #[test]
    fn test_translate_vec() {
        let gp = GeomPoint::new(1.0, 2.0, 3.0);
        let v = Vec3 { x: 10.0, y: -5.0, z: 0.5 };
        let result = gp.translate_vec(&v);
        assert!(pnt_approx_eq(&result.pnt, 11.0, -3.0, 3.5, 1e-10));
    }

    #[test]
    fn test_translate_vec_zero() {
        let gp = GeomPoint::new(1.0, 2.0, 3.0);
        let v = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        let result = gp.translate_vec(&v);
        assert!(pnt_approx_eq(&result.pnt, 1.0, 2.0, 3.0, 1e-10));
    }

    #[test]
    fn test_translate_pnt() {
        // Translate by vector from (0,0,0) to (1,2,3): shifts by (1,2,3)
        let gp = GeomPoint::new(5.0, 5.0, 5.0);
        let from = Pnt { x: 0.0, y: 0.0, z: 0.0 };
        let to = Pnt { x: 1.0, y: 2.0, z: 3.0 };
        let result = gp.translate_pnt(&from, &to);
        assert!(pnt_approx_eq(&result.pnt, 6.0, 7.0, 8.0, 1e-10));
    }

    #[test]
    fn test_translate_pnt_same() {
        let gp = GeomPoint::new(1.0, 2.0, 3.0);
        let pt = Pnt { x: 5.0, y: 5.0, z: 5.0 };
        let result = gp.translate_pnt(&pt, &pt);
        assert!(pnt_approx_eq(&result.pnt, 1.0, 2.0, 3.0, 1e-10));
    }

    #[test]
    fn test_copy_trait_object() {
        let gp = GeomPoint::new(7.0, 8.0, 9.0);
        let boxed: Box<dyn GeomGeometry> = gp.copy();
        // We can't downcast without Any, but we can call methods on it.
        // Translate and check that the returned type works:
        let v = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let _ = boxed.copy(); // ensure copy on Box<dyn> works
    }

    #[test]
    fn test_geom_point_from_pnt() {
        let p = Pnt { x: 3.0, y: 4.0, z: 5.0 };
        let gp = GeomPoint::from_pnt(p);
        assert_eq!(gp.x(), 3.0);
        assert_eq!(gp.y(), 4.0);
        assert_eq!(gp.z(), 5.0);
    }

    #[test]
    fn test_chain_transforms() {
        // Translate then rotate
        let gp = GeomPoint::new(1.0, 0.0, 0.0);
        let v = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        let z_ax = make_ax1(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        // after translate: (1,1,0), after 90-deg rotate: (-1,1,0)
        let result = gp.translate_vec(&v).rotate(&z_ax, std::f64::consts::PI / 2.0);
        assert!(pnt_approx_eq(&result.pnt, -1.0, 1.0, 0.0, 1e-10));
    }

    #[test]
    fn test_mirror_ax1_point_on_axis() {
        // A point ON the axis should be unchanged by mirror through axis
        let gp = GeomPoint::new(0.0, 0.0, 5.0);
        let z_ax = make_ax1(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let result = gp.mirror_ax1(&z_ax);
        assert!(pnt_approx_eq(&result.pnt, 0.0, 0.0, 5.0, 1e-10));
    }

    #[test]
    fn test_mirror_ax2_point_on_plane() {
        // A point on the plane should be unchanged by mirror through plane
        let gp = GeomPoint::new(3.0, 4.0, 0.0);
        let xy_plane = make_ax2(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let result = gp.mirror_ax2(&xy_plane);
        assert!(pnt_approx_eq(&result.pnt, 3.0, 4.0, 0.0, 1e-10));
    }

    #[test]
    fn test_rotate_point_on_axis() {
        // A point on the rotation axis should be unchanged
        let gp = GeomPoint::new(0.0, 0.0, 5.0);
        let z_ax = make_ax1(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let result = gp.rotate(&z_ax, 1.23456);
        assert!(pnt_approx_eq(&result.pnt, 0.0, 0.0, 5.0, 1e-10));
    }
}
