// FILE: mesh_vs_deformed_data_source.rs
// occt: MeshVS_DeformedDataSource

//! Data source for deformed mesh visualization.

#[derive(Clone, Debug)]
pub struct DeformedDataSource {
    pub source_id: u32,
    pub deformation_factor: f64,
    pub is_deformed: bool,
}

impl DeformedDataSource {
    pub fn new(source_id: u32) -> Self {
        DeformedDataSource {
            source_id,
            deformation_factor: 1.0,
            is_deformed: false,
        }
    }

    pub fn with_deformation_factor(mut self, factor: f64) -> Self {
        self.deformation_factor = factor.max(0.0);
        self
    }

    pub fn set_deformation_factor(&mut self, factor: f64) {
        self.deformation_factor = factor.max(0.0);
    }

    pub fn enable_deformation(&mut self) {
        self.is_deformed = true;
    }

    pub fn disable_deformation(&mut self) {
        self.is_deformed = false;
    }

    pub fn is_deformed(&self) -> bool {
        self.is_deformed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deformed_data_source_creation() {
        let src = DeformedDataSource::new(1);
        assert_eq!(src.source_id, 1);
        assert_eq!(src.deformation_factor, 1.0);
        assert!(!src.is_deformed());
    }

    #[test]
    fn deformed_data_source_builder() {
        let src = DeformedDataSource::new(1)
            .with_deformation_factor(2.5);

        assert_eq!(src.deformation_factor, 2.5);
    }

    #[test]
    fn deformed_data_source_control() {
        let mut src = DeformedDataSource::new(1);
        src.enable_deformation();
        assert!(src.is_deformed());
        src.disable_deformation();
        assert!(!src.is_deformed());
    }
}
