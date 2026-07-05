// FILE: iges_solid_cylindrical_surface.rs
// occt: IGESSolid_CylindricalSurface

//! Cylindrical Surface entity (IGES Type 192, Form 0 or 1).
//!
//! Right circular cylindrical surface defined by a point on the axis,
//! axis direction, radius, and optional reference direction for parametrization.

#[derive(Clone)]
pub struct IGESGeomPoint {
    id: usize,
}

impl IGESGeomPoint {
    pub fn new(id: usize) -> Self {
        IGESGeomPoint { id }
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

#[derive(Clone)]
pub struct IGESGeomDirection {
    id: usize,
}

impl IGESGeomDirection {
    pub fn new(id: usize) -> Self {
        IGESGeomDirection { id }
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// Cylindrical Surface entity
pub struct IGESSolidCylindricalSurface {
    location_point: Option<IGESGeomPoint>,
    axis: Option<IGESGeomDirection>,
    radius: f64,
    ref_dir: Option<IGESGeomDirection>,
    form_number: i32,
}

impl IGESSolidCylindricalSurface {
    /// Creates a new cylindrical surface
    pub fn new() -> Self {
        IGESSolidCylindricalSurface {
            location_point: None,
            axis: None,
            radius: 0.0,
            ref_dir: None,
            form_number: 0,
        }
    }

    /// Initializes the cylindrical surface
    pub fn init(
        &mut self,
        location: Option<IGESGeomPoint>,
        axis: Option<IGESGeomDirection>,
        radius: f64,
        ref_dir: Option<IGESGeomDirection>,
    ) {
        self.location_point = location;
        self.axis = axis;
        self.radius = radius;
        self.ref_dir = ref_dir.clone();
        // Form 1 if parametrised, Form 0 if not
        self.form_number = if ref_dir.is_some() { 1 } else { 0 };
    }

    /// Returns the location point on the axis
    pub fn location_point(&self) -> Option<&IGESGeomPoint> {
        self.location_point.as_ref()
    }

    /// Returns the axis direction
    pub fn axis(&self) -> Option<&IGESGeomDirection> {
        self.axis.as_ref()
    }

    /// Returns the radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Returns the reference direction (None if unparametrised)
    pub fn reference_dir(&self) -> Option<&IGESGeomDirection> {
        self.ref_dir.as_ref()
    }

    /// Returns true if the surface is parametrised (Form 1)
    pub fn is_parametrised(&self) -> bool {
        self.form_number == 1
    }

    pub fn form_number(&self) -> i32 {
        self.form_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylindrical_surface_creation() {
        let cs = IGESSolidCylindricalSurface::new();
        assert_eq!(cs.radius(), 0.0);
        assert!(!cs.is_parametrised());
    }

    #[test]
    fn test_cylindrical_surface_init_unparametrised() {
        let mut cs = IGESSolidCylindricalSurface::new();
        let loc = IGESGeomPoint::new(1);
        let axis = IGESGeomDirection::new(2);

        cs.init(Some(loc), Some(axis), 5.0, None);

        assert_eq!(cs.radius(), 5.0);
        assert!(!cs.is_parametrised());
        assert_eq!(cs.form_number(), 0);
    }

    #[test]
    fn test_cylindrical_surface_init_parametrised() {
        let mut cs = IGESSolidCylindricalSurface::new();
        let loc = IGESGeomPoint::new(1);
        let axis = IGESGeomDirection::new(2);
        let ref_dir = IGESGeomDirection::new(3);

        cs.init(Some(loc), Some(axis), 5.0, Some(ref_dir));

        assert!(cs.is_parametrised());
        assert_eq!(cs.form_number(), 1);
    }

    #[test]
    fn test_iges_point_null() {
        let p = IGESGeomPoint::new(0);
        assert!(p.is_null());

        let p2 = IGESGeomPoint::new(1);
        assert!(!p2.is_null());
    }

    #[test]
    fn test_iges_direction_null() {
        let d = IGESGeomDirection::new(0);
        assert!(d.is_null());

        let d2 = IGESGeomDirection::new(1);
        assert!(!d2.is_null());
    }
}
