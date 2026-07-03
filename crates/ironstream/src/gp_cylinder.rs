// FILE: gp_cylinder.rs
//! Port of gp_Cylinder — an infinite cylinder in 3D space.

use crate::gp::{Ax1, Ax2, Ax3, Pnt, Trsf, Vec3};

/// An infinite cylinder defined by an Ax3 placement and a radius.
/// The cylinder axis is the Z axis of the placement.
// occt: gp_Cylinder
#[derive(Clone, Debug)]
pub struct GpCylinder {
    /// The local coordinate system (placement). Z axis = cylinder axis.
    axis: Ax3,
    /// Radius of the cylinder (must be >= 0).
    radius: f64,
}

impl GpCylinder {
    /// Construct a new cylinder from an Ax3 placement and a radius.
    ///
    /// Panics if radius is negative.
    pub fn new(axis: Ax3, radius: f64) -> Self {
        assert!(radius >= 0.0, "gp_Cylinder: radius must be non-negative");
        GpCylinder { axis, radius }
    }

    /// Return the local coordinate system (Ax3) of the cylinder.
    /// The Z axis of this placement is the cylinder axis.
    pub fn position(&self) -> &Ax3 {
        &self.axis
    }

    /// Return the main axis (Z axis) of the cylinder as an Ax1.
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.axis.location, self.axis.z_dir)
    }

    /// Return the location (origin) of the cylinder's local frame.
    pub fn location(&self) -> Pnt {
        self.axis.location
    }

    /// Return the radius of the cylinder.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Return the X axis of the cylinder placement as Ax1.
    pub fn x_axis(&self) -> Ax1 {
        Ax1::new(self.axis.location, self.axis.x_dir)
    }

    /// Return the Y axis of the cylinder placement as Ax1.
    pub fn y_axis(&self) -> Ax1 {
        Ax1::new(self.axis.location, self.axis.y_dir)
    }

    /// Return the V (Z) axis of the cylinder as an Ax1 (same as axis()).
    pub fn vaxis(&self) -> Ax1 {
        Ax1::new(self.axis.location, self.axis.z_dir)
    }

    /// Return a line on the surface of the cylinder at the given angle (radians)
    /// measured from the X axis of the placement, in the XY plane.
    /// The line passes through the point on the rim at that angle and runs
    /// parallel to the cylinder axis.
    ///
    /// OCCT: gp_Cylinder::Line(angle) / at_angle
    pub fn at_angle(&self, angle: f64) -> crate::gp_prim::Lin {
        // point on the cylinder surface rim at the given angle
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        // origin of placement
        let loc = self.axis.location;
        let xdir = self.axis.x_dir;
        let ydir = self.axis.y_dir;
        // P = loc + radius*(cos*xdir + sin*ydir)
        let px = loc.x + self.radius * (cos_a * xdir.x + sin_a * ydir.x);
        let py = loc.y + self.radius * (cos_a * xdir.y + sin_a * ydir.y);
        let pz = loc.z + self.radius * (cos_a * xdir.z + sin_a * ydir.z);
        let p = Pnt::new(px, py, pz);
        // direction is the Z axis of the placement
        let z = self.axis.z_dir;
        crate::gp_prim::Lin::new_pd(p, z)
    }

    /// Set the radius.
    pub fn set_radius(&mut self, r: f64) {
        assert!(r >= 0.0, "gp_Cylinder: radius must be non-negative");
        self.radius = r;
    }

    /// Set the position (Ax3 placement).
    pub fn set_position(&mut self, ax3: Ax3) {
        self.axis = ax3;
    }

    /// Mirror the cylinder through a point.
    pub fn mirror_point(&self, p: &Pnt) -> Self {
        let loc = self.axis.location;
        let new_loc = Pnt::new(2.0 * p.x - loc.x, 2.0 * p.y - loc.y, 2.0 * p.z - loc.z);
        GpCylinder {
            axis: Ax3 {
                location: new_loc,
                x_dir: Pnt::new(-self.axis.x_dir.x, -self.axis.x_dir.y, -self.axis.x_dir.z),
                y_dir: Pnt::new(-self.axis.y_dir.x, -self.axis.y_dir.y, -self.axis.y_dir.z),
                z_dir: Pnt::new(-self.axis.z_dir.x, -self.axis.z_dir.y, -self.axis.z_dir.z),
            },
            radius: self.radius,
        }
    }

    /// Mirror the cylinder through an axis (Ax1).
    pub fn mirror_ax1(&self, ax: &Ax1) -> Self {
        let u = ax.direction.normalized();
        let reflect_vec = |d: Pnt| {
            let proj = 2.0 * d.dot(u);
            Pnt::new(proj * u.x - d.x, proj * u.y - d.y, proj * u.z - d.z)
        };
        GpCylinder {
            axis: Ax3 {
                location: self.axis.location.mirrored_axis(*ax),
                x_dir: reflect_vec(self.axis.x_dir),
                y_dir: reflect_vec(self.axis.y_dir),
                z_dir: reflect_vec(self.axis.z_dir),
            },
            radius: self.radius,
        }
    }

    /// Mirror the cylinder through a plane defined by an Ax2.
    pub fn mirror_ax2(&self, ax: &Ax2) -> Self {
        let n = ax.z_dir.normalized();
        let reflect_vec = |d: Pnt| {
            let proj = 2.0 * d.dot(n);
            Pnt::new(d.x - proj * n.x, d.y - proj * n.y, d.z - proj * n.z)
        };
        GpCylinder {
            axis: Ax3 {
                location: self.axis.location.mirrored_plane(*ax),
                x_dir: reflect_vec(self.axis.x_dir),
                y_dir: reflect_vec(self.axis.y_dir),
                z_dir: reflect_vec(self.axis.z_dir),
            },
            radius: self.radius,
        }
    }

    /// Rotate the cylinder around the given axis by the given angle (radians).
    pub fn rotate(&self, ax: &Ax1, angle: f64) -> Self {
        let t = Trsf::rotation(*ax, angle);
        GpCylinder {
            axis: self.axis.transformed(&t),
            radius: self.radius,
        }
    }

    /// Scale the cylinder from a point by a factor.
    /// The radius scales by the absolute value of the factor.
    pub fn scale(&self, p: &Pnt, factor: f64) -> Self {
        let loc = self.axis.location;
        let new_loc = Pnt::new(
            p.x + factor * (loc.x - p.x),
            p.y + factor * (loc.y - p.y),
            p.z + factor * (loc.z - p.z),
        );
        let sign = if factor < 0.0 { -1.0 } else { 1.0 };
        GpCylinder {
            axis: Ax3 {
                location: new_loc,
                x_dir: Pnt::new(sign * self.axis.x_dir.x, sign * self.axis.x_dir.y, sign * self.axis.x_dir.z),
                y_dir: Pnt::new(sign * self.axis.y_dir.x, sign * self.axis.y_dir.y, sign * self.axis.y_dir.z),
                z_dir: Pnt::new(sign * self.axis.z_dir.x, sign * self.axis.z_dir.y, sign * self.axis.z_dir.z),
            },
            radius: self.radius * factor.abs(),
        }
    }

    /// Translate the cylinder by a vector.
    pub fn translate(&self, v: &Vec3) -> Self {
        GpCylinder {
            axis: Ax3 {
                location: Pnt::new(self.axis.location.x + v.x, self.axis.location.y + v.y, self.axis.location.z + v.z),
                x_dir: self.axis.x_dir,
                y_dir: self.axis.y_dir,
                z_dir: self.axis.z_dir,
            },
            radius: self.radius,
        }
    }

    /// Translate the cylinder from point p1 to point p2.
    pub fn translate_points(&self, p1: &Pnt, p2: &Pnt) -> Self {
        let v = Vec3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        self.translate(&v)
    }
}

// Re-export the line type so tests can use it from this module.
pub use crate::gp_prim::Lin;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Ax1, Ax3, Pnt, Vec3};

    use std::f64::consts::{FRAC_PI_2, PI};

    const EPS: f64 = 1e-10;

    fn default_cylinder() -> GpCylinder {
        // Cylinder centred at origin, Z axis, radius 3.0
        let origin = Pnt::new(0.0, 0.0, 0.0);
        let ax3 = Ax3::from_origin_normal(origin, Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0));
        GpCylinder::new(ax3, 3.0)
    }

    #[test]
    fn test_radius() {
        let cyl = default_cylinder();
        assert!((cyl.radius() - 3.0).abs() < EPS);
    }

    #[test]
    fn test_location() {
        let cyl = default_cylinder();
        let loc = cyl.location();
        assert!(loc.x.abs() < EPS);
        assert!(loc.y.abs() < EPS);
        assert!(loc.z.abs() < EPS);
    }

    #[test]
    fn test_axis_direction() {
        let cyl = default_cylinder();
        let ax = cyl.axis();
        assert!(ax.direction.z - 1.0 < EPS);
        assert!(ax.direction.x.abs() < EPS);
        assert!(ax.direction.y.abs() < EPS);
    }

    #[test]
    fn test_vaxis_equals_axis() {
        let cyl = default_cylinder();
        let ax = cyl.axis();
        let vax = cyl.vaxis();
        assert!((ax.direction.x - vax.direction.x).abs() < EPS);
        assert!((ax.direction.y - vax.direction.y).abs() < EPS);
        assert!((ax.direction.z - vax.direction.z).abs() < EPS);
    }

    #[test]
    fn test_at_angle_zero() {
        let cyl = default_cylinder();
        let line = cyl.at_angle(0.0);
        // At angle 0, point should be (3, 0, 0)
        let loc = line.location();
        assert!((loc.x - 3.0).abs() < EPS, "x={}", loc.x);
        assert!(loc.y.abs() < EPS, "y={}", loc.y);
        assert!(loc.z.abs() < EPS, "z={}", loc.z);
        // Direction should be Z axis
        let d = line.direction();
        assert!(d.z - 1.0 < EPS);
    }

    #[test]
    fn test_at_angle_pi_half() {
        let cyl = default_cylinder();
        let line = cyl.at_angle(FRAC_PI_2);
        let loc = line.location();
        // At 90 degrees, point should be (0, 3, 0)
        assert!(loc.x.abs() < EPS, "x={}", loc.x);
        assert!((loc.y - 3.0).abs() < EPS, "y={}", loc.y);
        assert!(loc.z.abs() < EPS, "z={}", loc.z);
    }

    #[test]
    fn test_at_angle_pi() {
        let cyl = default_cylinder();
        let line = cyl.at_angle(PI);
        let loc = line.location();
        // At 180 degrees, point should be (-3, 0, 0)
        assert!((loc.x + 3.0).abs() < EPS, "x={}", loc.x);
        assert!(loc.y.abs() < EPS, "y={}", loc.y);
        assert!(loc.z.abs() < EPS, "z={}", loc.z);
    }

    #[test]
    fn test_translate() {
        let cyl = default_cylinder();
        let v = Vec3::new(1.0, 2.0, 3.0);
        let moved = cyl.translate(&v);
        let loc = moved.location();
        assert!((loc.x - 1.0).abs() < EPS);
        assert!((loc.y - 2.0).abs() < EPS);
        assert!((loc.z - 3.0).abs() < EPS);
        // Radius unchanged
        assert!((moved.radius() - 3.0).abs() < EPS);
    }

    #[test]
    fn test_translate_points() {
        let cyl = default_cylinder();
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(5.0, 0.0, 0.0);
        let moved = cyl.translate_points(&p1, &p2);
        assert!((moved.location().x - 5.0).abs() < EPS);
    }

    #[test]
    fn test_scale_positive() {
        let cyl = default_cylinder();
        let p = Pnt::new(0.0, 0.0, 0.0);
        let scaled = cyl.scale(&p, 2.0);
        assert!((scaled.radius() - 6.0).abs() < EPS);
        // location stays at origin since centre is the scale point
        assert!(scaled.location().x.abs() < EPS);
    }

    #[test]
    fn test_scale_negative_factor_radius_absolute() {
        let cyl = default_cylinder();
        let p = Pnt::new(0.0, 0.0, 0.0);
        let scaled = cyl.scale(&p, -2.0);
        // Radius uses absolute value
        assert!((scaled.radius() - 6.0).abs() < EPS);
    }

    #[test]
    fn test_scale_off_centre() {
        // Scale from a point offset from the cylinder centre
        let cyl = default_cylinder();
        let p = Pnt::new(1.0, 0.0, 0.0);
        let factor = 2.0;
        let scaled = cyl.scale(&p, factor);
        // New location = p + factor*(old_loc - p) = (1,0,0) + 2*(-1,0,0) = (-1,0,0)
        assert!((scaled.location().x + 1.0).abs() < EPS, "x={}", scaled.location().x);
        assert!((scaled.radius() - 6.0).abs() < EPS);
    }

    #[test]
    fn test_rotate() {
        let cyl = default_cylinder();
        // Rotate 90 degrees around Z axis — location stays at origin,
        // but X axis of the cylinder frame rotates to Y.
        let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
        let rotated = cyl.rotate(&z_ax1, FRAC_PI_2);
        // Location should still be at origin
        assert!(rotated.location().x.abs() < EPS);
        assert!(rotated.location().y.abs() < EPS);
        // Radius unchanged
        assert!((rotated.radius() - 3.0).abs() < EPS);
        // At angle 0 the point on surface should now be at Y direction (because X axis rotated to Y)
        let line = rotated.at_angle(0.0);
        let loc = line.location();
        assert!(loc.x.abs() < EPS, "x={}", loc.x);
        assert!((loc.y - 3.0).abs() < EPS, "y={}", loc.y);
    }

    #[test]
    fn test_mirror_point() {
        let cyl = default_cylinder();
        let p = Pnt::new(1.0, 0.0, 0.0);
        let mirrored = cyl.mirror_point(&p);
        // Centre mirrored through (1,0,0): new centre = 2*(1,0,0) - (0,0,0) = (2,0,0)
        assert!((mirrored.location().x - 2.0).abs() < EPS, "x={}", mirrored.location().x);
        // Radius unchanged
        assert!((mirrored.radius() - 3.0).abs() < EPS);
    }

    #[test]
    fn test_clone_equality() {
        let cyl = default_cylinder();
        let cyl2 = cyl.clone();
        // Compare fields directly since Ax3 doesn't derive PartialEq
        assert!((cyl.radius() - cyl2.radius()).abs() < EPS);
        let loc1 = cyl.location();
        let loc2 = cyl2.location();
        assert!((loc1.x - loc2.x).abs() < EPS);
        assert!((loc1.y - loc2.y).abs() < EPS);
        assert!((loc1.z - loc2.z).abs() < EPS);
    }

    #[test]
    fn test_set_radius() {
        let mut cyl = default_cylinder();
        cyl.set_radius(7.5);
        assert!((cyl.radius() - 7.5).abs() < EPS);
    }

    #[test]
    #[should_panic]
    fn test_negative_radius_panics() {
        let origin = Pnt::new(0.0, 0.0, 0.0);
        let ax3 = Ax3::from_origin_normal(origin, Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0));
        let _cyl = GpCylinder::new(ax3, -1.0);
    }

    #[test]
    fn test_x_axis_y_axis() {
        let cyl = default_cylinder();
        let xax = cyl.x_axis();
        let yax = cyl.y_axis();
        // X axis should point in +X
        assert!((xax.direction.x - 1.0).abs() < EPS);
        assert!(xax.direction.y.abs() < EPS);
        assert!(xax.direction.z.abs() < EPS);
        // Y axis should point in +Y
        assert!(yax.direction.x.abs() < EPS);
        assert!((yax.direction.y - 1.0).abs() < EPS);
        assert!(yax.direction.z.abs() < EPS);
    }

    #[test]
    fn test_position_returns_correct_ax3() {
        let cyl = default_cylinder();
        let pos = cyl.position();
        // Z axis of placement is cylinder axis
        let z = pos.z_dir;
        assert!((z.z - 1.0).abs() < EPS);
    }

    #[test]
    fn test_mirror_ax1() {
        let cyl = default_cylinder();
        // Mirror through Y axis
        let y_ax = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
        let mirrored = cyl.mirror_ax1(&y_ax);
        // Centre at origin stays at origin
        assert!(mirrored.location().x.abs() < EPS);
        assert!(mirrored.location().y.abs() < EPS);
        assert!((mirrored.radius() - 3.0).abs() < EPS);
    }
}
