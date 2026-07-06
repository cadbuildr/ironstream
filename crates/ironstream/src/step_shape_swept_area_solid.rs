// FILE: step_shape_swept_area_solid.rs
// occt: StepShape_SweptAreaSolid

use std::sync::Arc;

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

/// Represents a swept area solid in STEP format.
/// Inherits from StepShape_SolidModel.
pub struct SweptAreaSolid {
    name: Arc<str>,
    swept_area: Option<Arc<CurveBoundedSurface>>,
}

impl SweptAreaSolid {
    /// Create a new SweptAreaSolid
    pub fn new() -> Self {
        SweptAreaSolid {
            name: Arc::from(""),
            swept_area: None,
        }
    }

    /// Initialize with name and swept area
    pub fn init(&mut self, name: Arc<str>, swept_area: Arc<CurveBoundedSurface>) {
        self.name = name;
        self.swept_area = Some(swept_area);
    }

    /// Set the swept area
    pub fn set_swept_area(&mut self, swept_area: Arc<CurveBoundedSurface>) {
        self.swept_area = Some(swept_area);
    }

    /// Get the swept area
    pub fn swept_area(&self) -> Option<&Arc<CurveBoundedSurface>> {
        self.swept_area.as_ref()
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }
}

impl Default for SweptAreaSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swept_area_solid_creation() {
        let sas = SweptAreaSolid::new();
        assert_eq!(sas.name(), "");
        assert!(sas.swept_area().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut sas = SweptAreaSolid::new();
        let swept_area = Arc::new(CurveBoundedSurface::new(1));
        let name: Arc<str> = Arc::from("swept_solid_1");

        sas.init(name.clone(), swept_area);

        assert_eq!(sas.name(), "swept_solid_1");
        assert!(sas.swept_area().is_some());
    }

    #[test]
    fn test_set_swept_area() {
        let mut sas = SweptAreaSolid::new();
        let swept_area = Arc::new(CurveBoundedSurface::new(42));

        sas.set_swept_area(swept_area);

        assert!(sas.swept_area().is_some());
        assert_eq!(sas.swept_area().unwrap().id(), 42);
    }

    #[test]
    fn test_set_name() {
        let mut sas = SweptAreaSolid::new();
        sas.set_name(Arc::from("named_solid"));

        assert_eq!(sas.name(), "named_solid");
    }
}
