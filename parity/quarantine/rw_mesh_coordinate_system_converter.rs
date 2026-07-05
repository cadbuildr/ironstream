// FILE: rw_mesh_coordinate_system_converter.rs
// occt: RWMesh_CoordinateSystemConverter

use super::rw_mesh_coordinate_system::RwMeshCoordinateSystem;

/// Converts between coordinate systems for mesh data
#[derive(Clone, Debug)]
pub struct RwMeshCoordinateSystemConverter {
    source: RwMeshCoordinateSystem,
    target: RwMeshCoordinateSystem,
}

impl RwMeshCoordinateSystemConverter {
    /// Creates a converter between two coordinate systems
    pub fn new(source: RwMeshCoordinateSystem, target: RwMeshCoordinateSystem) -> Self {
        Self { source, target }
    }

    /// Returns the source coordinate system
    pub fn source(&self) -> &RwMeshCoordinateSystem {
        &self.source
    }

    /// Returns the target coordinate system
    pub fn target(&self) -> &RwMeshCoordinateSystem {
        &self.target
    }

    /// Converts a point from source to target coordinate system
    pub fn convert_point(&self, point: [f64; 3]) -> [f64; 3] {
        // Simple translation: subtract source origin, then add target origin
        let relative = [
            point[0] - self.source.origin()[0],
            point[1] - self.source.origin()[1],
            point[2] - self.source.origin()[2],
        ];

        [
            relative[0] + self.target.origin()[0],
            relative[1] + self.target.origin()[1],
            relative[2] + self.target.origin()[2],
        ]
    }
}

impl Default for RwMeshCoordinateSystemConverter {
    fn default() -> Self {
        Self::new(
            RwMeshCoordinateSystem::default(),
            RwMeshCoordinateSystem::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_conversion() {
        let sys = RwMeshCoordinateSystem::default();
        let converter = RwMeshCoordinateSystemConverter::new(sys.clone(), sys);
        let point = [1.0, 2.0, 3.0];
        let converted = converter.convert_point(point);
        assert_eq!(converted, point);
    }

    #[test]
    fn test_translation() {
        let mut target = RwMeshCoordinateSystem::default();
        target.origin = [1.0, 2.0, 3.0];

        let source = RwMeshCoordinateSystem::default();
        let converter = RwMeshCoordinateSystemConverter::new(source, target);

        let point = [0.0, 0.0, 0.0];
        let converted = converter.convert_point(point);
        assert_eq!(converted, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_clone() {
        let sys = RwMeshCoordinateSystem::default();
        let converter = RwMeshCoordinateSystemConverter::new(sys.clone(), sys);
        let cloned = converter.clone();
        assert_eq!(cloned.source().origin(), [0.0, 0.0, 0.0]);
    }
}
