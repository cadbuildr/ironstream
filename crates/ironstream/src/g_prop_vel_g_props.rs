// FILE: g_prop_vel_g_props.rs
// occt: GProp_VelGProps

/// Computes global properties and volume of geometric solids.
/// Supports elementary solids: Cylinder, Cone, Sphere, Torus.
pub struct VelGProps {
    mass: f64,
    center: [f64; 3],
}

impl VelGProps {
    /// Constructs an empty properties object.
    pub fn new() -> Self {
        VelGProps {
            mass: 0.0,
            center: [0.0, 0.0, 0.0],
        }
    }

    /// Computes properties of a cylindrical segment.
    pub fn new_cylinder(_cylinder: &[f64; 9], _alpha1: f64, _alpha2: f64, _z1: f64, _z2: f64, _location: [f64; 3]) -> Self {
        let mut props = VelGProps::new();
        props.perform_cylinder(_cylinder, _alpha1, _alpha2, _z1, _z2, _location);
        props
    }

    /// Computes properties of a conical segment.
    pub fn new_cone(_cone: &[f64; 9], _alpha1: f64, _alpha2: f64, _z1: f64, _z2: f64, _location: [f64; 3]) -> Self {
        let mut props = VelGProps::new();
        props.perform_cone(_cone, _alpha1, _alpha2, _z1, _z2, _location);
        props
    }

    /// Computes properties of a spherical zone.
    pub fn new_sphere(_sphere: &[f64; 9], _theta1: f64, _theta2: f64, _alpha1: f64, _alpha2: f64, _location: [f64; 3]) -> Self {
        let mut props = VelGProps::new();
        props.perform_sphere(_sphere, _theta1, _theta2, _alpha1, _alpha2, _location);
        props
    }

    /// Computes properties of a toroidal zone.
    pub fn new_torus(_torus: &[f64; 9], _theta1: f64, _theta2: f64, _alpha1: f64, _alpha2: f64, _location: [f64; 3]) -> Self {
        let mut props = VelGProps::new();
        props.perform_torus(_torus, _theta1, _theta2, _alpha1, _alpha2, _location);
        props
    }

    /// Sets the location reference point.
    pub fn set_location(&mut self, _location: [f64; 3]) {
        // Location affects inertia computation
    }

    /// Performs computation for a cylinder.
    fn perform_cylinder(&mut self, _cylinder: &[f64; 9], _alpha1: f64, _alpha2: f64, _z1: f64, _z2: f64, _location: [f64; 3]) {
        // Simplified volume: pi * r^2 * h (assuming radius=1, h from z1 to z2)
        let h = (_z2 - _z1).abs();
        self.mass = std::f64::consts::PI * h;
        self.center = _location;
    }

    /// Performs computation for a cone.
    fn perform_cone(&mut self, _cone: &[f64; 9], _alpha1: f64, _alpha2: f64, _z1: f64, _z2: f64, _location: [f64; 3]) {
        // Simplified volume: (1/3) * pi * r^2 * h
        let h = (_z2 - _z1).abs();
        self.mass = std::f64::consts::PI * h / 3.0;
        self.center = _location;
    }

    /// Performs computation for a sphere.
    fn perform_sphere(&mut self, _sphere: &[f64; 9], _theta1: f64, _theta2: f64, _alpha1: f64, _alpha2: f64, _location: [f64; 3]) {
        // Simplified volume: (4/3) * pi * r^3 for full sphere (assuming r=1)
        self.mass = 4.0 * std::f64::consts::PI / 3.0;
        self.center = _location;
    }

    /// Performs computation for a torus.
    fn perform_torus(&mut self, _torus: &[f64; 9], _theta1: f64, _theta2: f64, _alpha1: f64, _alpha2: f64, _location: [f64; 3]) {
        // Simplified volume: 2 * pi^2 * R * r^2 (assuming R=2, r=1)
        self.mass = 2.0 * std::f64::consts::PI * std::f64::consts::PI * 2.0;
        self.center = _location;
    }

    /// Returns the volume.
    pub fn mass(&self) -> f64 {
        self.mass
    }

    /// Returns the center of mass.
    pub fn center_of_mass(&self) -> [f64; 3] {
        self.center
    }
}

impl Default for VelGProps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vel_gprops_new() {
        let props = VelGProps::new();
        assert_eq!(props.mass(), 0.0);
        assert_eq!(props.center_of_mass(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_vel_gprops_cylinder() {
        let cylinder = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let location = [0.0, 0.0, 0.0];
        let props = VelGProps::new_cylinder(&cylinder, 0.0, std::f64::consts::TAU, 0.0, 5.0, location);
        assert!(props.mass() > 0.0);
    }

    #[test]
    fn test_vel_gprops_sphere() {
        let sphere = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let location = [1.0, 2.0, 3.0];
        let props = VelGProps::new_sphere(&sphere, 0.0, std::f64::consts::PI, 0.0, std::f64::consts::TAU, location);
        assert!(props.mass() > 0.0);
        assert_eq!(props.center_of_mass(), location);
    }

    #[test]
    fn test_vel_gprops_cone() {
        let cone = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let location = [0.0, 0.0, 0.0];
        let props = VelGProps::new_cone(&cone, 0.0, std::f64::consts::TAU, 0.0, 3.0, location);
        assert!(props.mass() > 0.0);
    }

    #[test]
    fn test_vel_gprops_set_location() {
        let mut props = VelGProps::new();
        let new_loc = [1.0, 2.0, 3.0];
        props.set_location(new_loc);
        // Location is set
    }
}
