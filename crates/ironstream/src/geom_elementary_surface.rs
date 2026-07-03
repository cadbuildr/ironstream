// FILE: geom_elementary_surface.rs
// occt: Geom_ElementarySurface

//! Abstract class for elementary surfaces (plane, cylinder, cone, sphere, torus).

pub trait ElementarySurface {
    /// Returns the axis of the elementary surface.
    fn axis(&self) -> (f64, f64, f64);

    /// Check if the surface is a plane.
    fn is_plane(&self) -> bool;

    /// Check if the surface is a cylinder.
    fn is_cylinder(&self) -> bool;

    /// Check if the surface is a cone.
    fn is_cone(&self) -> bool;

    /// Check if the surface is a sphere.
    fn is_sphere(&self) -> bool;

    /// Check if the surface is a torus.
    fn is_torus(&self) -> bool;
}

pub struct Plane {
    normal: (f64, f64, f64),
}

pub struct Cylinder {
    axis: (f64, f64, f64),
    radius: f64,
}

pub struct Cone {
    axis: (f64, f64, f64),
    half_angle: f64,
}

pub struct Sphere {
    center: (f64, f64, f64),
    radius: f64,
}

pub struct Torus {
    axis: (f64, f64, f64),
    major_radius: f64,
    minor_radius: f64,
}

impl ElementarySurface for Plane {
    fn axis(&self) -> (f64, f64, f64) {
        self.normal
    }

    fn is_plane(&self) -> bool {
        true
    }

    fn is_cylinder(&self) -> bool {
        false
    }

    fn is_cone(&self) -> bool {
        false
    }

    fn is_sphere(&self) -> bool {
        false
    }

    fn is_torus(&self) -> bool {
        false
    }
}

impl ElementarySurface for Cylinder {
    fn axis(&self) -> (f64, f64, f64) {
        self.axis
    }

    fn is_plane(&self) -> bool {
        false
    }

    fn is_cylinder(&self) -> bool {
        true
    }

    fn is_cone(&self) -> bool {
        false
    }

    fn is_sphere(&self) -> bool {
        false
    }

    fn is_torus(&self) -> bool {
        false
    }
}

impl ElementarySurface for Cone {
    fn axis(&self) -> (f64, f64, f64) {
        self.axis
    }

    fn is_plane(&self) -> bool {
        false
    }

    fn is_cylinder(&self) -> bool {
        false
    }

    fn is_cone(&self) -> bool {
        true
    }

    fn is_sphere(&self) -> bool {
        false
    }

    fn is_torus(&self) -> bool {
        false
    }
}

impl ElementarySurface for Sphere {
    fn axis(&self) -> (f64, f64, f64) {
        (0.0, 0.0, 1.0)
    }

    fn is_plane(&self) -> bool {
        false
    }

    fn is_cylinder(&self) -> bool {
        false
    }

    fn is_cone(&self) -> bool {
        false
    }

    fn is_sphere(&self) -> bool {
        true
    }

    fn is_torus(&self) -> bool {
        false
    }
}

impl ElementarySurface for Torus {
    fn axis(&self) -> (f64, f64, f64) {
        self.axis
    }

    fn is_plane(&self) -> bool {
        false
    }

    fn is_cylinder(&self) -> bool {
        false
    }

    fn is_cone(&self) -> bool {
        false
    }

    fn is_sphere(&self) -> bool {
        false
    }

    fn is_torus(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane() {
        let plane = Plane {
            normal: (0.0, 0.0, 1.0),
        };
        assert!(plane.is_plane());
        assert!(!plane.is_cylinder());
    }

    #[test]
    fn test_cylinder() {
        let cyl = Cylinder {
            axis: (0.0, 0.0, 1.0),
            radius: 1.0,
        };
        assert!(cyl.is_cylinder());
        assert!(!cyl.is_plane());
    }

    #[test]
    fn test_sphere() {
        let sphere = Sphere {
            center: (0.0, 0.0, 0.0),
            radius: 1.0,
        };
        assert!(sphere.is_sphere());
        assert!(!sphere.is_cone());
    }

    #[test]
    fn test_torus() {
        let torus = Torus {
            axis: (0.0, 0.0, 1.0),
            major_radius: 5.0,
            minor_radius: 1.0,
        };
        assert!(torus.is_torus());
    }
}
