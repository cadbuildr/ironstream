// FILE: geom_bounded_surface.rs
// occt: Geom_BoundedSurface

//! Abstract class for bounded surfaces.
//! Represents a surface with defined parameter bounds.

pub trait BoundedSurface {
    /// Returns the first U parameter of the bounded surface.
    fn first_u_parameter(&self) -> f64;

    /// Returns the last U parameter of the bounded surface.
    fn last_u_parameter(&self) -> f64;

    /// Returns the first V parameter of the bounded surface.
    fn first_v_parameter(&self) -> f64;

    /// Returns the last V parameter of the bounded surface.
    fn last_v_parameter(&self) -> f64;

    /// Check if surface is U-closed.
    fn is_u_closed(&self) -> bool;

    /// Check if surface is V-closed.
    fn is_v_closed(&self) -> bool;
}

pub struct BoundedSurfaceImpl {
    u_first: f64,
    u_last: f64,
    v_first: f64,
    v_last: f64,
    u_closed: bool,
    v_closed: bool,
}

impl BoundedSurfaceImpl {
    pub fn new(
        u_first: f64,
        u_last: f64,
        v_first: f64,
        v_last: f64,
        u_closed: bool,
        v_closed: bool,
    ) -> Self {
        BoundedSurfaceImpl {
            u_first,
            u_last,
            v_first,
            v_last,
            u_closed,
            v_closed,
        }
    }
}

impl BoundedSurface for BoundedSurfaceImpl {
    fn first_u_parameter(&self) -> f64 {
        self.u_first
    }

    fn last_u_parameter(&self) -> f64 {
        self.u_last
    }

    fn first_v_parameter(&self) -> f64 {
        self.v_first
    }

    fn last_v_parameter(&self) -> f64 {
        self.v_last
    }

    fn is_u_closed(&self) -> bool {
        self.u_closed
    }

    fn is_v_closed(&self) -> bool {
        self.v_closed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_surface_impl() {
        let surface = BoundedSurfaceImpl::new(0.0, 1.0, 0.0, 2.0, false, true);
        assert_eq!(surface.first_u_parameter(), 0.0);
        assert_eq!(surface.last_u_parameter(), 1.0);
        assert_eq!(surface.first_v_parameter(), 0.0);
        assert_eq!(surface.last_v_parameter(), 2.0);
        assert!(!surface.is_u_closed());
        assert!(surface.is_v_closed());
    }

    #[test]
    fn test_bounded_surface_all_open() {
        let surface = BoundedSurfaceImpl::new(0.0, 10.0, 0.0, 10.0, false, false);
        assert!(!surface.is_u_closed());
        assert!(!surface.is_v_closed());
    }

    #[test]
    fn test_bounded_surface_all_closed() {
        let surface = BoundedSurfaceImpl::new(0.0, 10.0, 0.0, 10.0, true, true);
        assert!(surface.is_u_closed());
        assert!(surface.is_v_closed());
    }
}
