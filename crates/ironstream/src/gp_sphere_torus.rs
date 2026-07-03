// FILE: gp_sphere_torus.rs
// occt: gp_Sphere, gp_Torus, gp_Cone, gp_Cylinder (analytical surface params)

/// Axis placement: origin + main direction + reference direction.
/// occt: gp_Ax3 (minimal)
#[derive(Clone, Debug)]
pub struct GpAx3 {
    pub location: [f64; 3],
    pub direction: [f64; 3],   // Z axis
    pub x_direction: [f64; 3], // X axis
}

impl Default for GpAx3 {
    fn default() -> Self {
        Self {
            location: [0.0; 3],
            direction: [0.0, 0.0, 1.0],
            x_direction: [1.0, 0.0, 0.0],
        }
    }
}

impl GpAx3 {
    pub fn new(location: [f64; 3], direction: [f64; 3], x_direction: [f64; 3]) -> Self {
        Self { location, direction, x_direction }
    }

    pub fn y_direction(&self) -> [f64; 3] {
        // Y = Z × X
        let z = self.direction;
        let x = self.x_direction;
        [
            z[1]*x[2] - z[2]*x[1],
            z[2]*x[0] - z[0]*x[2],
            z[0]*x[1] - z[1]*x[0],
        ]
    }
}

/// Analytical sphere: position (Ax3) + radius.
/// occt: gp_Sphere
#[derive(Clone, Debug)]
pub struct GpSphere {
    pub position: GpAx3,
    pub radius: f64,
}

impl Default for GpSphere {
    fn default() -> Self { Self { position: GpAx3::default(), radius: 1.0 } }
}

impl GpSphere {
    pub fn new(position: GpAx3, radius: f64) -> Self {
        assert!(radius >= 0.0, "Sphere radius must be non-negative");
        Self { position, radius }
    }

    pub fn location(&self) -> [f64; 3] { self.position.location }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn set_radius(&mut self, r: f64) { self.radius = r.max(0.0); }

    pub fn volume(&self) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * self.radius.powi(3)
    }

    pub fn area(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn distance(&self, p: [f64; 3]) -> f64 {
        let l = self.position.location;
        let d = ((p[0]-l[0]).powi(2) + (p[1]-l[1]).powi(2) + (p[2]-l[2]).powi(2)).sqrt();
        (d - self.radius).abs()
    }

    pub fn contains(&self, p: [f64; 3]) -> bool {
        let l = self.position.location;
        let d2 = (p[0]-l[0]).powi(2) + (p[1]-l[1]).powi(2) + (p[2]-l[2]).powi(2);
        d2 <= self.radius * self.radius
    }

    /// Mirrored sphere (same shape, mirrored position).
    pub fn mirrored(&self) -> Self {
        let mut pos = self.position.clone();
        pos.location[0] = -pos.location[0];
        pos.location[1] = -pos.location[1];
        pos.location[2] = -pos.location[2];
        Self { position: pos, radius: self.radius }
    }
}

/// Analytical torus: position (Ax3) + major radius R + minor radius r.
/// occt: gp_Torus
#[derive(Clone, Debug)]
pub struct GpTorus {
    pub position: GpAx3,
    pub major_radius: f64,
    pub minor_radius: f64,
}

impl Default for GpTorus {
    fn default() -> Self {
        Self { position: GpAx3::default(), major_radius: 2.0, minor_radius: 0.5 }
    }
}

impl GpTorus {
    pub fn new(position: GpAx3, major_radius: f64, minor_radius: f64) -> Self {
        assert!(major_radius >= 0.0 && minor_radius >= 0.0);
        Self { position, major_radius, minor_radius }
    }

    pub fn location(&self) -> [f64; 3] { self.position.location }
    pub fn major_radius(&self) -> f64 { self.major_radius }
    pub fn minor_radius(&self) -> f64 { self.minor_radius }

    pub fn set_major_radius(&mut self, r: f64) { self.major_radius = r.max(0.0); }
    pub fn set_minor_radius(&mut self, r: f64) { self.minor_radius = r.max(0.0); }

    pub fn volume(&self) -> f64 {
        2.0 * std::f64::consts::PI.powi(2) * self.major_radius * self.minor_radius.powi(2)
    }

    pub fn area(&self) -> f64 {
        4.0 * std::f64::consts::PI.powi(2) * self.major_radius * self.minor_radius
    }

    /// Shortest distance from a point to the torus surface (approximate, assumes Z-axis torus).
    pub fn distance(&self, p: [f64; 3]) -> f64 {
        let c = self.position.location;
        let rxy = ((p[0]-c[0]).powi(2) + (p[1]-c[1]).powi(2)).sqrt();
        let dz  = p[2] - c[2];
        let d_to_ring = ((rxy - self.major_radius).powi(2) + dz.powi(2)).sqrt();
        (d_to_ring - self.minor_radius).abs()
    }
}

/// Analytical cylinder: position (Ax3) + radius + height.
/// occt: gp_Cylinder
#[derive(Clone, Debug)]
pub struct GpCylinder {
    pub position: GpAx3,
    pub radius: f64,
}

impl Default for GpCylinder {
    fn default() -> Self { Self { position: GpAx3::default(), radius: 1.0 } }
}

impl GpCylinder {
    pub fn new(position: GpAx3, radius: f64) -> Self {
        Self { position, radius: radius.max(0.0) }
    }

    pub fn radius(&self) -> f64 { self.radius }
    pub fn set_radius(&mut self, r: f64) { self.radius = r.max(0.0); }

    /// Distance from a 3D point to the cylinder axis (ignoring height).
    pub fn distance_to_axis(&self, p: [f64; 3]) -> f64 {
        let c = self.position.location;
        ((p[0]-c[0]).powi(2) + (p[1]-c[1]).powi(2)).sqrt()
    }

    pub fn is_on_surface(&self, p: [f64; 3], tol: f64) -> bool {
        (self.distance_to_axis(p) - self.radius).abs() <= tol
    }
}

/// Analytical cone: position (Ax3) + two radii + half-angle.
/// occt: gp_Cone
#[derive(Clone, Debug)]
pub struct GpCone {
    pub position: GpAx3,
    pub semi_angle: f64,  // radians
    pub radius: f64,      // reference radius at apex plane
}

impl Default for GpCone {
    fn default() -> Self {
        Self { position: GpAx3::default(), semi_angle: std::f64::consts::PI / 4.0, radius: 0.0 }
    }
}

impl GpCone {
    pub fn new(position: GpAx3, semi_angle: f64, radius: f64) -> Self {
        Self { position, semi_angle: semi_angle.clamp(0.0, std::f64::consts::PI / 2.0), radius: radius.max(0.0) }
    }

    pub fn semi_angle(&self) -> f64 { self.semi_angle }
    pub fn reference_radius(&self) -> f64 { self.radius }

    /// Radius at given height along the axis.
    pub fn radius_at(&self, height: f64) -> f64 {
        (self.radius + height * self.semi_angle.tan()).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_volume_area() {
        let s = GpSphere::new(GpAx3::default(), 1.0);
        let v = s.volume();
        let a = s.area();
        assert!((v - 4.0 / 3.0 * std::f64::consts::PI).abs() < 1e-10);
        assert!((a - 4.0 * std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn sphere_contains() {
        let s = GpSphere::new(GpAx3::default(), 2.0);
        assert!(s.contains([1.0, 1.0, 0.0]));
        assert!(!s.contains([2.0, 2.0, 0.0]));
    }

    #[test]
    fn torus_volume_area() {
        let t = GpTorus::new(GpAx3::default(), 3.0, 1.0);
        let v = t.volume();
        let expected_v = 2.0 * std::f64::consts::PI.powi(2) * 3.0 * 1.0;
        assert!((v - expected_v).abs() < 1e-8);
        let a = t.area();
        let expected_a = 4.0 * std::f64::consts::PI.powi(2) * 3.0 * 1.0;
        assert!((a - expected_a).abs() < 1e-8);
    }

    #[test]
    fn cylinder_surface() {
        let c = GpCylinder::new(GpAx3::default(), 2.0);
        assert!(c.is_on_surface([2.0, 0.0, 5.0], 1e-9));
        assert!(!c.is_on_surface([1.0, 0.0, 0.0], 1e-9));
    }

    #[test]
    fn cone_radius_at() {
        let cone = GpCone::new(GpAx3::default(), std::f64::consts::PI / 4.0, 0.0);
        let r = cone.radius_at(1.0);
        assert!((r - 1.0).abs() < 1e-10);
    }

    #[test]
    fn ax3_y_direction() {
        let ax = GpAx3::default();
        let y = ax.y_direction();
        // Z(0,0,1) × X(1,0,0) = (0*0-1*0, 1*1-0*0, 0*0-0*1) = (0,1,0)
        assert!((y[0]).abs() < 1e-10);
        assert!((y[1] - 1.0).abs() < 1e-10);
        assert!((y[2]).abs() < 1e-10);
    }
}
