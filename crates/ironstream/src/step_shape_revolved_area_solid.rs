// FILE: step_shape_revolved_area_solid.rs
// occt: StepShape_RevolvedAreaSolid

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

/// Placeholder for StepGeom_CurveBoundedSurface
pub struct CurveBoundedSurface {
    id: usize,
}

impl CurveBoundedSurface {
    pub fn new(id: usize) -> Self {
        CurveBoundedSurface { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Represents a revolved area solid in STEP format.
/// Inherits from StepShape_SweptAreaSolid.
pub struct RevolvedAreaSolid {
    name: Arc<str>,
    swept_area: Option<Arc<CurveBoundedSurface>>,
    axis: Option<Arc<Axis1Placement>>,
    angle: f64,
}

impl RevolvedAreaSolid {
    /// Create a new RevolvedAreaSolid
    pub fn new() -> Self {
        RevolvedAreaSolid {
            name: Arc::from(""),
            swept_area: None,
            axis: None,
            angle: 0.0,
        }
    }

    /// Initialize with name, swept area, axis, and angle
    pub fn init(
        &mut self,
        name: Arc<str>,
        swept_area: Arc<CurveBoundedSurface>,
        axis: Arc<Axis1Placement>,
        angle: f64,
    ) {
        self.name = name;
        self.swept_area = Some(swept_area);
        self.axis = Some(axis);
        self.angle = angle;
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
        self.angle = angle;
    }

    /// Get the revolution angle
    pub fn angle(&self) -> f64 {
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
    pub fn swept_area(&self) -> Option<&Arc<CurveBoundedSurface>> {
        self.swept_area.as_ref()
    }

    /// Set the swept area
    pub fn set_swept_area(&mut self, swept_area: Arc<CurveBoundedSurface>) {
        self.swept_area = Some(swept_area);
    }
}

impl Default for RevolvedAreaSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_revolved_area_solid_creation() {
        let ras = RevolvedAreaSolid::new();
        assert_eq!(ras.name(), "");
        assert_eq!(ras.angle(), 0.0);
        assert!(ras.axis().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut ras = RevolvedAreaSolid::new();
        let swept_area = Arc::new(CurveBoundedSurface::new(1));
        let axis = Arc::new(Axis1Placement::new(0.0, 0.0, 1.0));
        let name: Arc<str> = Arc::from("revolved_solid_1");

        ras.init(name.clone(), swept_area, axis.clone(), PI / 2.0);

        assert_eq!(ras.name(), "revolved_solid_1");
        assert!((ras.angle() - PI / 2.0).abs() < 1e-10);
        assert!(ras.axis().is_some());
    }

    #[test]
    fn test_set_axis() {
        let mut ras = RevolvedAreaSolid::new();
        let axis = Arc::new(Axis1Placement::new(1.0, 2.0, 3.0));

        ras.set_axis(axis.clone());

        assert!(ras.axis().is_some());
        let ax = ras.axis().unwrap();
        assert_eq!(ax.x(), 1.0);
        assert_eq!(ax.y(), 2.0);
        assert_eq!(ax.z(), 3.0);
    }

    #[test]
    fn test_set_angle() {
        let mut ras = RevolvedAreaSolid::new();
        let test_angle = PI;

        ras.set_angle(test_angle);

        assert_eq!(ras.angle(), test_angle);
    }

    #[test]
    fn test_set_swept_area() {
        let mut ras = RevolvedAreaSolid::new();
        let swept_area = Arc::new(CurveBoundedSurface::new(42));

        ras.set_swept_area(swept_area);

        assert!(ras.swept_area().is_some());
        assert_eq!(ras.swept_area().unwrap().id(), 42);
    }
}
