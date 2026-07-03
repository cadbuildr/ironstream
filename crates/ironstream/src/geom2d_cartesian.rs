// FILE: geom2d_cartesian.rs
use crate::gp2d::{Pnt2d, Vec2d, Ax2d};
use crate::geom2d_circle::Dir2d;
use crate::precision::{CONFUSION, ANGULAR};

/// A 2D Cartesian point in the Geom2d package, wrapping a gp_Pnt2d.
// occt: Geom2d_CartesianPoint
#[derive(Debug, Clone, PartialEq)]
pub struct Geom2dCartesianPoint {
    pnt: Pnt2d,
}

impl Geom2dCartesianPoint {
    /// Create a new Geom2dCartesianPoint from x and y coordinates.
    pub fn new(x: f64, y: f64) -> Self {
        Self { pnt: Pnt2d::new(x, y) }
    }

    /// Create a new Geom2dCartesianPoint from an existing gp_Pnt2d.
    pub fn new_from_pnt(pnt: Pnt2d) -> Self {
        Self { pnt }
    }

    /// Return the underlying gp_Pnt2d.
    pub fn pnt(&self) -> &Pnt2d {
        &self.pnt
    }

    /// Return the X coordinate.
    pub fn x(&self) -> f64 {
        self.pnt.x
    }

    /// Return the Y coordinate.
    pub fn y(&self) -> f64 {
        self.pnt.y
    }

    /// Set the X coordinate.
    pub fn set_x(&mut self, x: f64) {
        self.pnt.x = x;
    }

    /// Set the Y coordinate.
    pub fn set_y(&mut self, y: f64) {
        self.pnt.y = y;
    }

    /// Replace the underlying gp_Pnt2d.
    pub fn set_pnt(&mut self, pnt: Pnt2d) {
        self.pnt = pnt;
    }

    /// Euclidean distance to another Geom2dCartesianPoint.
    pub fn distance(&self, other: &Geom2dCartesianPoint) -> f64 {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        (dx * dx + dy * dy).sqrt()
    }

    /// Translate this point by a 2D vector (in place).
    pub fn translate_vec(&mut self, v: &Vec2d) {
        self.pnt.x += v.x;
        self.pnt.y += v.y;
    }

    /// Translate this point so that `p1` maps to `p2` (in place).
    pub fn translate_pnt(&mut self, p1: &Pnt2d, p2: &Pnt2d) {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        self.pnt.x += dx;
        self.pnt.y += dy;
    }

    /// Mirror this point through another point (in place).
    pub fn mirror_pt(&mut self, center: &Pnt2d) {
        let nx = 2.0 * center.x - self.pnt.x;
        let ny = 2.0 * center.y - self.pnt.y;
        self.pnt.x = nx;
        self.pnt.y = ny;
    }

    /// Mirror this point through an axis (in place).
    /// The axis is defined by an origin point and a unit direction.
    pub fn mirror_ax(&mut self, ax: &Ax2d) {
        let loc = ax.location;
        let dir = ax.direction;
        // Translate to axis origin
        let px = self.pnt.x - loc.x;
        let py = self.pnt.y - loc.y;
        // direction unit vector
        let dx = dir.x;
        let dy = dir.y;
        // Reflection: r = 2*(p dot d)*d - p
        let dot = px * dx + py * dy;
        let rx = 2.0 * dot * dx - px;
        let ry = 2.0 * dot * dy - py;
        self.pnt.x = rx + loc.x;
        self.pnt.y = ry + loc.y;
    }

    /// Rotate this point around a center point by an angle in radians (in place).
    pub fn rotate(&mut self, center: &Pnt2d, angle: f64) {
        let px = self.pnt.x - center.x;
        let py = self.pnt.y - center.y;
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let rx = cos_a * px - sin_a * py;
        let ry = sin_a * px + cos_a * py;
        self.pnt.x = rx + center.x;
        self.pnt.y = ry + center.y;
    }

    /// Scale this point with respect to a center point by a scalar factor (in place).
    pub fn scale(&mut self, center: &Pnt2d, factor: f64) {
        let nx = center.x + factor * (self.pnt.x - center.x);
        let ny = center.y + factor * (self.pnt.y - center.y);
        self.pnt.x = nx;
        self.pnt.y = ny;
    }
}

/// A 2D axis placement in the Geom2d package, wrapping a gp_Ax2d.
// occt: Geom2d_AxisPlacement
#[derive(Debug, Clone)]
pub struct Geom2dAxisPlacement {
    ax: Ax2d,
}

impl PartialEq for Geom2dAxisPlacement {
    fn eq(&self, other: &Self) -> bool {
        self.ax.location == other.ax.location && self.ax.direction == other.ax.direction
    }
}

impl Geom2dAxisPlacement {
    /// Create a new Geom2dAxisPlacement from a location and direction.
    pub fn new(location: Pnt2d, direction: Dir2d) -> Self {
        Self { ax: Ax2d::new(location, direction) }
    }

    /// Create from an existing gp_Ax2d.
    pub fn new_from_ax2d(ax: Ax2d) -> Self {
        Self { ax }
    }

    /// Return the underlying gp_Ax2d.
    pub fn ax2d(&self) -> &Ax2d {
        &self.ax
    }

    /// Return the location (origin) of this axis.
    pub fn location(&self) -> &Pnt2d {
        &self.ax.location
    }

    /// Return the direction of this axis.
    pub fn direction(&self) -> &Dir2d {
        &self.ax.direction
    }

    /// Compute the angle in radians between this axis and another axis.
    /// Returns a value in [0, pi].
    pub fn angle(&self, other: &Geom2dAxisPlacement) -> f64 {
        let d1 = self.ax.direction;
        let d2 = other.ax.direction;
        let dot = (d1.x * d2.x + d1.y * d2.y).clamp(-1.0, 1.0);
        dot.acos()
    }

    /// Return true if this axis and another are coaxial (same location and parallel directions)
    /// within the precision tolerances.
    pub fn is_coaxial(&self, other: &Geom2dAxisPlacement, linear_tol: f64, angular_tol: f64) -> bool {
        let loc1 = self.ax.location;
        let loc2 = other.ax.location;
        let dlx = loc1.x - loc2.x;
        let dly = loc1.y - loc2.y;
        let dist = (dlx * dlx + dly * dly).sqrt();
        if dist > linear_tol {
            return false;
        }
        self.is_parallel(other, angular_tol)
    }

    /// Return true if this axis is normal (perpendicular) to another within angular_tol.
    pub fn is_normal(&self, other: &Geom2dAxisPlacement, angular_tol: f64) -> bool {
        let angle = self.angle(other);
        let diff = (angle - core::f64::consts::FRAC_PI_2).abs();
        diff <= angular_tol
    }

    /// Return true if this axis is opposite (anti-parallel) to another within angular_tol.
    pub fn is_opposite(&self, other: &Geom2dAxisPlacement, angular_tol: f64) -> bool {
        let angle = self.angle(other);
        let diff = (angle - core::f64::consts::PI).abs();
        diff <= angular_tol
    }

    /// Return true if this axis is parallel (or anti-parallel) to another within angular_tol.
    pub fn is_parallel(&self, other: &Geom2dAxisPlacement, angular_tol: f64) -> bool {
        let angle = self.angle(other);
        angle <= angular_tol || (core::f64::consts::PI - angle) <= angular_tol
    }

    /// Reverse the direction of this axis in place.
    pub fn reverse(&mut self) {
        let dir = self.ax.direction;
        let new_dir = Dir2d::new(-dir.x, -dir.y);
        self.ax = Ax2d::new(self.ax.location, new_dir);
    }

    /// Return a new axis with reversed direction, leaving self unchanged.
    pub fn reversed_result(&self) -> Geom2dAxisPlacement {
        let dir = self.ax.direction;
        let new_dir = Dir2d::new(-dir.x, -dir.y);
        Geom2dAxisPlacement {
            ax: Ax2d::new(self.ax.location, new_dir),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp2d::{Pnt2d, Vec2d, Ax2d};
    use crate::geom2d_circle::Dir2d;
    use core::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};

    // ----------------------------------------------------------------
    // Geom2dCartesianPoint tests
    // ----------------------------------------------------------------

    #[test]
    fn test_point_new_and_accessors() {
        let p = Geom2dCartesianPoint::new(3.0, 4.0);
        assert!((p.x() - 3.0).abs() < 1e-14);
        assert!((p.y() - 4.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_new_from_pnt() {
        let pnt = Pnt2d::new(1.5, 2.5);
        let p = Geom2dCartesianPoint::new_from_pnt(pnt);
        assert!((p.x() - 1.5).abs() < 1e-14);
        assert!((p.y() - 2.5).abs() < 1e-14);
    }

    #[test]
    fn test_point_set_x_set_y() {
        let mut p = Geom2dCartesianPoint::new(0.0, 0.0);
        p.set_x(7.0);
        p.set_y(-3.0);
        assert!((p.x() - 7.0).abs() < 1e-14);
        assert!((p.y() + 3.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_set_pnt() {
        let mut p = Geom2dCartesianPoint::new(0.0, 0.0);
        let pnt = Pnt2d::new(10.0, 20.0);
        p.set_pnt(pnt);
        assert!((p.x() - 10.0).abs() < 1e-14);
        assert!((p.y() - 20.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Geom2dCartesianPoint::new(0.0, 0.0);
        let p2 = Geom2dCartesianPoint::new(3.0, 4.0);
        assert!((p1.distance(&p2) - 5.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_distance_zero() {
        let p = Geom2dCartesianPoint::new(1.0, 2.0);
        assert!(p.distance(&p.clone()).abs() < 1e-14);
    }

    #[test]
    fn test_point_translate_vec() {
        let mut p = Geom2dCartesianPoint::new(1.0, 2.0);
        let v = Vec2d::new(3.0, -1.0);
        p.translate_vec(&v);
        assert!((p.x() - 4.0).abs() < 1e-14);
        assert!((p.y() - 1.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_translate_pnt() {
        let mut p = Geom2dCartesianPoint::new(5.0, 5.0);
        let p1 = Pnt2d::new(1.0, 1.0);
        let p2 = Pnt2d::new(4.0, 2.0);
        p.translate_pnt(&p1, &p2);
        // moved by (3, 1)
        assert!((p.x() - 8.0).abs() < 1e-14);
        assert!((p.y() - 6.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_mirror_pt() {
        let mut p = Geom2dCartesianPoint::new(1.0, 0.0);
        let center = Pnt2d::new(0.0, 0.0);
        p.mirror_pt(&center);
        assert!((p.x() + 1.0).abs() < 1e-14);
        assert!((p.y()).abs() < 1e-14);
    }

    #[test]
    fn test_point_mirror_pt_non_origin() {
        let mut p = Geom2dCartesianPoint::new(1.0, 0.0);
        let center = Pnt2d::new(2.0, 0.0);
        p.mirror_pt(&center);
        // 2*2 - 1 = 3
        assert!((p.x() - 3.0).abs() < 1e-14);
        assert!((p.y()).abs() < 1e-14);
    }

    #[test]
    fn test_point_mirror_ax_x_axis() {
        // Mirror (1, 2) through the X axis (origin, direction (1, 0))
        let mut p = Geom2dCartesianPoint::new(1.0, 2.0);
        let ax = Ax2d::new(Pnt2d::new(0.0, 0.0), Dir2d::new(1.0, 0.0));
        p.mirror_ax(&ax);
        assert!((p.x() - 1.0).abs() < 1e-14);
        assert!((p.y() + 2.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_mirror_ax_y_axis() {
        // Mirror (3, 1) through Y axis (direction (0, 1))
        let mut p = Geom2dCartesianPoint::new(3.0, 1.0);
        let ax = Ax2d::new(Pnt2d::new(0.0, 0.0), Dir2d::new(0.0, 1.0));
        p.mirror_ax(&ax);
        assert!((p.x() + 3.0).abs() < 1e-14);
        assert!((p.y() - 1.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_mirror_ax_diagonal() {
        // Mirror (1, 0) through y=x axis (direction (1/sqrt2, 1/sqrt2))
        let sq2 = 2.0_f64.sqrt();
        let mut p = Geom2dCartesianPoint::new(1.0, 0.0);
        let ax = Ax2d::new(Pnt2d::new(0.0, 0.0), Dir2d::new(1.0 / sq2, 1.0 / sq2));
        p.mirror_ax(&ax);
        // should become (0, 1)
        assert!((p.x()).abs() < 1e-12);
        assert!((p.y() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_point_rotate_90() {
        let mut p = Geom2dCartesianPoint::new(1.0, 0.0);
        let center = Pnt2d::new(0.0, 0.0);
        p.rotate(&center, FRAC_PI_2);
        assert!((p.x()).abs() < 1e-14);
        assert!((p.y() - 1.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_rotate_180() {
        let mut p = Geom2dCartesianPoint::new(2.0, 0.0);
        let center = Pnt2d::new(1.0, 0.0);
        p.rotate(&center, PI);
        // rotate (2,0) around (1,0) by pi => (0, 0)
        assert!((p.x()).abs() < 1e-13);
        assert!((p.y()).abs() < 1e-13);
    }

    #[test]
    fn test_point_rotate_full_circle() {
        let mut p = Geom2dCartesianPoint::new(3.0, 4.0);
        let center = Pnt2d::new(0.0, 0.0);
        p.rotate(&center, 2.0 * PI);
        assert!((p.x() - 3.0).abs() < 1e-12);
        assert!((p.y() - 4.0).abs() < 1e-12);
    }

    #[test]
    fn test_point_scale() {
        let mut p = Geom2dCartesianPoint::new(2.0, 3.0);
        let center = Pnt2d::new(0.0, 0.0);
        p.scale(&center, 2.0);
        assert!((p.x() - 4.0).abs() < 1e-14);
        assert!((p.y() - 6.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_scale_non_origin_center() {
        let mut p = Geom2dCartesianPoint::new(3.0, 0.0);
        let center = Pnt2d::new(1.0, 0.0);
        p.scale(&center, 3.0);
        // 1 + 3*(3-1) = 7
        assert!((p.x() - 7.0).abs() < 1e-14);
        assert!((p.y()).abs() < 1e-14);
    }

    #[test]
    fn test_point_scale_zero_factor() {
        let mut p = Geom2dCartesianPoint::new(5.0, 7.0);
        let center = Pnt2d::new(2.0, 3.0);
        p.scale(&center, 0.0);
        assert!((p.x() - 2.0).abs() < 1e-14);
        assert!((p.y() - 3.0).abs() < 1e-14);
    }

    #[test]
    fn test_point_scale_negative_factor() {
        let mut p = Geom2dCartesianPoint::new(3.0, 0.0);
        let center = Pnt2d::new(1.0, 0.0);
        p.scale(&center, -1.0);
        // 1 + (-1)*(3-1) = -1
        assert!((p.x() + 1.0).abs() < 1e-14);
        assert!((p.y()).abs() < 1e-14);
    }

    // ----------------------------------------------------------------
    // Geom2dAxisPlacement tests
    // ----------------------------------------------------------------

    #[test]
    fn test_axis_new_and_accessors() {
        let loc = Pnt2d::new(1.0, 2.0);
        let dir = Dir2d::new(1.0, 0.0);
        let ax = Geom2dAxisPlacement::new(loc.clone(), dir.clone());
        assert!((ax.location().x - 1.0).abs() < 1e-14);
        assert!((ax.location().y - 2.0).abs() < 1e-14);
        assert!((ax.direction().x - 1.0).abs() < 1e-14);
        assert!((ax.direction().y).abs() < 1e-14);
    }

    #[test]
    fn test_axis_angle_same_direction() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        assert!(ax1.angle(&ax2).abs() < 1e-12);
    }

    #[test]
    fn test_axis_angle_perpendicular() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(0.0, 1.0));
        let angle = ax1.angle(&ax2);
        assert!((angle - FRAC_PI_2).abs() < 1e-12);
    }

    #[test]
    fn test_axis_angle_opposite() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(-1.0, 0.0));
        let angle = ax1.angle(&ax2);
        assert!((angle - PI).abs() < 1e-12);
    }

    #[test]
    fn test_axis_angle_45_degrees() {
        let loc = Pnt2d::new(0.0, 0.0);
        let sq2 = 2.0_f64.sqrt();
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0 / sq2, 1.0 / sq2));
        let angle = ax1.angle(&ax2);
        assert!((angle - FRAC_PI_4).abs() < 1e-12);
    }

    #[test]
    fn test_axis_is_normal_true() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(0.0, 1.0));
        assert!(ax1.is_normal(&ax2, 1e-10));
    }

    #[test]
    fn test_axis_is_normal_false() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        assert!(!ax1.is_normal(&ax2, 1e-10));
    }

    #[test]
    fn test_axis_is_opposite_true() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(-1.0, 0.0));
        assert!(ax1.is_opposite(&ax2, 1e-10));
    }

    #[test]
    fn test_axis_is_opposite_false() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        assert!(!ax1.is_opposite(&ax2, 1e-10));
    }

    #[test]
    fn test_axis_is_parallel_same() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(Pnt2d::new(5.0, 0.0), Dir2d::new(1.0, 0.0));
        assert!(ax1.is_parallel(&ax2, 1e-10));
    }

    #[test]
    fn test_axis_is_parallel_opposite() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(-1.0, 0.0));
        assert!(ax1.is_parallel(&ax2, 1e-10));
    }

    #[test]
    fn test_axis_is_parallel_false() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(0.0, 1.0));
        assert!(!ax1.is_parallel(&ax2, 1e-10));
    }

    #[test]
    fn test_axis_is_coaxial_true() {
        let loc = Pnt2d::new(1.0, 2.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        assert!(ax1.is_coaxial(&ax2, 1e-7, 1e-10));
    }

    #[test]
    fn test_axis_is_coaxial_far_apart() {
        let ax1 = Geom2dAxisPlacement::new(Pnt2d::new(0.0, 0.0), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(Pnt2d::new(1.0, 0.0), Dir2d::new(1.0, 0.0));
        assert!(!ax1.is_coaxial(&ax2, 1e-7, 1e-10));
    }

    #[test]
    fn test_axis_is_coaxial_different_direction() {
        let loc = Pnt2d::new(0.0, 0.0);
        let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(0.0, 1.0));
        assert!(!ax1.is_coaxial(&ax2, 1e-7, 1e-10));
    }

    #[test]
    fn test_axis_reverse() {
        let mut ax = Geom2dAxisPlacement::new(Pnt2d::new(0.0, 0.0), Dir2d::new(1.0, 0.0));
        ax.reverse();
        assert!((ax.direction().x + 1.0).abs() < 1e-14);
        assert!((ax.direction().y).abs() < 1e-14);
    }

    #[test]
    fn test_axis_reversed_result_does_not_modify_self() {
        let ax = Geom2dAxisPlacement::new(Pnt2d::new(0.0, 0.0), Dir2d::new(1.0, 0.0));
        let rev = ax.reversed_result();
        // Original unchanged
        assert!((ax.direction().x - 1.0).abs() < 1e-14);
        // Reversed is correct
        assert!((rev.direction().x + 1.0).abs() < 1e-14);
        assert!((rev.direction().y).abs() < 1e-14);
    }

    #[test]
    fn test_axis_reverse_double_reversal() {
        let ax = Geom2dAxisPlacement::new(Pnt2d::new(3.0, -1.0), Dir2d::new(0.0, 1.0));
        let rev = ax.reversed_result();
        let rev2 = rev.reversed_result();
        assert!((rev2.direction().x - ax.direction().x).abs() < 1e-14);
        assert!((rev2.direction().y - ax.direction().y).abs() < 1e-14);
    }

    #[test]
    fn test_axis_location_preserved_after_reverse() {
        let loc = Pnt2d::new(5.0, -3.0);
        let mut ax = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
        ax.reverse();
        assert!((ax.location().x - 5.0).abs() < 1e-14);
        assert!((ax.location().y + 3.0).abs() < 1e-14);
    }

    #[test]
    fn test_axis_from_ax2d() {
        let a = Ax2d::new(Pnt2d::new(2.0, 3.0), Dir2d::new(0.0, 1.0));
        let ax = Geom2dAxisPlacement::new_from_ax2d(a);
        assert!((ax.location().x - 2.0).abs() < 1e-14);
        assert!((ax.location().y - 3.0).abs() < 1e-14);
        assert!((ax.direction().y - 1.0).abs() < 1e-14);
    }
}
