// FILE: step_shape_revolved_face_solid.rs
// occt: StepShape_RevolvedFaceSolid

use std::sync::Arc;

/// Placeholder for StepGeom_Axis1Placement
pub struct Axis1Placement {
    x: f64,
    y: f64,
    z: f64,
}

impl Axis1Placement {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Axis1Placement { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

/// Placeholder for StepShape_FaceSurface
pub struct FaceSurface {
    id: usize,
}

impl FaceSurface {
    pub fn new(id: usize) -> Self {
        FaceSurface { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Represents a revolved face solid in STEP format.
/// Inherits from StepShape_SweptFaceSolid.
pub struct RevolvedFaceSolid {
    name: Arc<str>,
    swept_area: Option<Arc<FaceSurface>>,
    axis: Option<Arc<Axis1Placement>>,
    angle: Option<f64>,
}

impl RevolvedFaceSolid {
    /// Create a new RevolvedFaceSolid
    pub fn new() -> Self {
        RevolvedFaceSolid {
            name: Arc::from(""),
            swept_area: None,
            axis: None,
            angle: None,
        }
    }

    /// Initialize with name and swept area (without rotation)
    pub fn init_basic(&mut self, name: Arc<str>, swept_area: Arc<FaceSurface>) {
        self.name = name;
        self.swept_area = Some(swept_area);
        self.axis = None;
        self.angle = None;
    }

    /// Initialize with name, swept area, axis, and angle
    pub fn init_full(
        &mut self,
        name: Arc<str>,
        swept_area: Arc<FaceSurface>,
        axis: Arc<Axis1Placement>,
        angle: f64,
    ) {
        self.name = name;
        self.swept_area = Some(swept_area);
        self.axis = Some(axis);
        self.angle = Some(angle);
    }

    /// Set the revolution axis
    pub fn set_axis(&mut self, axis: Arc<Axis1Placement>) {
        self.axis = Some(axis);
    }

    /// Get the revolution axis
    pub fn axis(&self) -> Option<&Arc<Axis1Placement>> {
        self.axis.as_ref()
    }

    /// Set the revolution angle (in radians)
    pub fn set_angle(&mut self, angle: f64) {
        self.angle = Some(angle);
    }

    /// Get the revolution angle
    pub fn angle(&self) -> Option<f64> {
        self.angle
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }

    /// Get the swept area
    pub fn swept_area(&self) -> Option<&Arc<FaceSurface>> {
        self.swept_area.as_ref()
    }

    /// Set the swept area
    pub fn set_swept_area(&mut self, swept_area: Arc<FaceSurface>) {
        self.swept_area = Some(swept_area);
    }
}

impl Default for RevolvedFaceSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_revolved_face_solid_creation() {
        let rfs = RevolvedFaceSolid::new();
        assert_eq!(rfs.name(), "");
        assert!(rfs.axis().is_none());
        assert!(rfs.angle().is_none());
    }

    #[test]
    fn test_init_basic() {
        let mut rfs = RevolvedFaceSolid::new();
        let swept_area = Arc::new(FaceSurface::new(1));
        let name: Arc<str> = Arc::from("revolved_face_1");

        rfs.init_basic(name.clone(), swept_area);

        assert_eq!(rfs.name(), "revolved_face_1");
        assert!(rfs.swept_area().is_some());
        assert!(rfs.axis().is_none());
        assert!(rfs.angle().is_none());
    }

    #[test]
    fn test_init_full() {
        let mut rfs = RevolvedFaceSolid::new();
        let swept_area = Arc::new(FaceSurface::new(1));
        let axis = Arc::new(Axis1Placement::new(0.0, 0.0, 1.0));
        let name: Arc<str> = Arc::from("revolved_face_2");

        rfs.init_full(name.clone(), swept_area, axis.clone(), PI / 2.0);

        assert_eq!(rfs.name(), "revolved_face_2");
        assert!(rfs.axis().is_some());
        assert!(rfs.angle().is_some());
        assert!((rfs.angle().unwrap() - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_set_axis() {
        let mut rfs = RevolvedFaceSolid::new();
        let axis = Arc::new(Axis1Placement::new(1.0, 2.0, 3.0));

        rfs.set_axis(axis.clone());

        assert!(rfs.axis().is_some());
        let ax = rfs.axis().unwrap();
        assert_eq!(ax.x(), 1.0);
        assert_eq!(ax.y(), 2.0);
        assert_eq!(ax.z(), 3.0);
    }

    #[test]
    fn test_set_angle() {
        let mut rfs = RevolvedFaceSolid::new();
        let test_angle = PI;

        rfs.set_angle(test_angle);

        assert!(rfs.angle().is_some());
        assert_eq!(rfs.angle().unwrap(), test_angle);
    }

    #[test]
    fn test_set_swept_area() {
        let mut rfs = RevolvedFaceSolid::new();
        let swept_area = Arc::new(FaceSurface::new(99));

        rfs.set_swept_area(swept_area);

        assert!(rfs.swept_area().is_some());
        assert_eq!(rfs.swept_area().unwrap().id(), 99);
    }
}
