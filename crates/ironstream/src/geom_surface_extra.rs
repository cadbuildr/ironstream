// FILE: geom_surface_extra.rs
// occt: Geom_Plane, Geom_CylindricalSurface, Geom_ConicalSurface, Geom_SphericalSurface

/// Plane surface: infinite flat plane.
/// occt: Geom_Plane
#[derive(Clone, Debug)]
pub struct GeomPlane {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
}

impl GeomPlane {
    pub fn new(origin: [f64; 3], normal: [f64; 3]) -> Self {
        Self { origin, normal }
    }

    pub fn is_point_on_plane(&self, pt: [f64; 3]) -> bool {
        let dx = pt[0] - self.origin[0];
        let dy = pt[1] - self.origin[1];
        let dz = pt[2] - self.origin[2];
        let dot = dx * self.normal[0] + dy * self.normal[1] + dz * self.normal[2];
        dot.abs() < 1e-6
    }
}

impl Default for GeomPlane {
    fn default() -> Self {
        Self {
            origin: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        }
    }
}

/// Cylindrical surface: radius + axis.
/// occt: Geom_CylindricalSurface
#[derive(Clone, Debug)]
pub struct GeomCylindricalSurface {
    pub origin: [f64; 3],
    pub axis: [f64; 3],
    pub radius: f64,
}

impl GeomCylindricalSurface {
    pub fn new(origin: [f64; 3], axis: [f64; 3], radius: f64) -> Self {
        Self { origin, axis, radius: radius.max(0.001) }
    }

    pub fn distance_to_axis(&self, pt: [f64; 3]) -> f64 {
        let dx = pt[0] - self.origin[0];
        let dy = pt[1] - self.origin[1];
        let dz = pt[2] - self.origin[2];
        let ax_sq = self.axis[0] * self.axis[0] + self.axis[1] * self.axis[1] + self.axis[2] * self.axis[2];
        if ax_sq < 1e-10 { return 0.0; }
        let dot = dx * self.axis[0] + dy * self.axis[1] + dz * self.axis[2];
        let proj_x = self.origin[0] + (dot / ax_sq) * self.axis[0];
        let proj_y = self.origin[1] + (dot / ax_sq) * self.axis[1];
        let proj_z = self.origin[2] + (dot / ax_sq) * self.axis[2];
        let dx2 = pt[0] - proj_x;
        let dy2 = pt[1] - proj_y;
        let dz2 = pt[2] - proj_z;
        (dx2 * dx2 + dy2 * dy2 + dz2 * dz2).sqrt()
    }

    pub fn is_point_on_surface(&self, pt: [f64; 3]) -> bool {
        (self.distance_to_axis(pt) - self.radius).abs() < 1e-6
    }
}

impl Default for GeomCylindricalSurface {
    fn default() -> Self {
        Self::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0)
    }
}

/// Conical surface: half angle + axis + apex.
/// occt: Geom_ConicalSurface
#[derive(Clone, Debug)]
pub struct GeomConicalSurface {
    pub apex: [f64; 3],
    pub axis: [f64; 3],
    pub half_angle: f64,  // radians
}

impl GeomConicalSurface {
    pub fn new(apex: [f64; 3], axis: [f64; 3], half_angle: f64) -> Self {
        Self {
            apex,
            axis,
            half_angle: half_angle.max(0.01).min(std::f64::consts::PI / 2.0 - 0.01),
        }
    }

    pub fn radius_at_distance(&self, dist: f64) -> f64 {
        dist * self.half_angle.tan()
    }

    pub fn is_point_on_surface(&self, pt: [f64; 3]) -> bool {
        let dx = pt[0] - self.apex[0];
        let dy = pt[1] - self.apex[1];
        let dz = pt[2] - self.apex[2];
        let dot = dx * self.axis[0] + dy * self.axis[1] + dz * self.axis[2];
        let ax_len = (self.axis[0] * self.axis[0] + self.axis[1] * self.axis[1] + self.axis[2] * self.axis[2]).sqrt();
        if ax_len < 1e-10 { return false; }
        let projection_distance = dot / ax_len;
        let expected_radius = self.radius_at_distance(projection_distance);
        let radial_dist = self.distance_to_axis(pt);
        (radial_dist - expected_radius).abs() < 1e-6
    }

    fn distance_to_axis(&self, pt: [f64; 3]) -> f64 {
        let dx = pt[0] - self.apex[0];
        let dy = pt[1] - self.apex[1];
        let dz = pt[2] - self.apex[2];
        let ax_sq = self.axis[0] * self.axis[0] + self.axis[1] * self.axis[1] + self.axis[2] * self.axis[2];
        if ax_sq < 1e-10 { return 0.0; }
        let dot = dx * self.axis[0] + dy * self.axis[1] + dz * self.axis[2];
        let proj_x = (dot / ax_sq) * self.axis[0];
        let proj_y = (dot / ax_sq) * self.axis[1];
        let proj_z = (dot / ax_sq) * self.axis[2];
        let dx2 = dx - proj_x;
        let dy2 = dy - proj_y;
        let dz2 = dz - proj_z;
        (dx2 * dx2 + dy2 * dy2 + dz2 * dz2).sqrt()
    }
}

impl Default for GeomConicalSurface {
    fn default() -> Self {
        Self::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.2)
    }
}

/// Spherical surface: center + radius.
/// occt: Geom_SphericalSurface
#[derive(Clone, Debug)]
pub struct GeomSphericalSurface {
    pub center: [f64; 3],
    pub radius: f64,
}

impl GeomSphericalSurface {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.001),
        }
    }

    pub fn is_point_on_surface(&self, pt: [f64; 3]) -> bool {
        let dx = pt[0] - self.center[0];
        let dy = pt[1] - self.center[1];
        let dz = pt[2] - self.center[2];
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        (dist - self.radius).abs() < 1e-6
    }

    pub fn distance_to_center(&self, pt: [f64; 3]) -> f64 {
        let dx = pt[0] - self.center[0];
        let dy = pt[1] - self.center[1];
        let dz = pt[2] - self.center[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn surface_area(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius * self.radius
    }

    pub fn volume(&self) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * self.radius * self.radius * self.radius
    }
}

impl Default for GeomSphericalSurface {
    fn default() -> Self {
        Self::new([0.0, 0.0, 0.0], 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geom_plane() {
        let p = GeomPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(p.is_point_on_plane([0.0, 0.0, 0.0]));
        assert!(p.is_point_on_plane([1.0, 2.0, 0.0]));
        assert!(!p.is_point_on_plane([0.0, 0.0, 1.0]));
    }

    #[test]
    fn geom_cylindrical() {
        let c = GeomCylindricalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
        assert_eq!(c.distance_to_axis([1.0, 0.0, 5.0]), 1.0);
        assert!(c.is_point_on_surface([1.0, 0.0, 5.0]));
    }

    #[test]
    fn geom_conical() {
        let co = GeomConicalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.2);
        let r = co.radius_at_distance(5.0);
        assert!(r > 0.0);
    }

    #[test]
    fn geom_spherical() {
        let s = GeomSphericalSurface::new([0.0, 0.0, 0.0], 2.0);
        assert!(s.is_point_on_surface([2.0, 0.0, 0.0]));
        assert_eq!(s.distance_to_center([2.0, 0.0, 0.0]), 2.0);
        assert!(s.surface_area() > 0.0);
    }

    #[test]
    fn geom_spherical_volume() {
        let s = GeomSphericalSurface::new([0.0, 0.0, 0.0], 1.0);
        let vol = s.volume();
        let expected = (4.0 / 3.0) * std::f64::consts::PI;
        assert!((vol - expected).abs() < 0.01);
    }
}
