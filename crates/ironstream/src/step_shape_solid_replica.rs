// FILE: step_shape_solid_replica.rs
// occt: StepShape_SolidReplica

use std::sync::Arc;

/// Placeholder for StepGeom_CartesianTransformationOperator3d
pub struct CartesianTransformationOperator3d {
    matrix: [[f64; 4]; 4],
}

impl CartesianTransformationOperator3d {
    pub fn new() -> Self {
        CartesianTransformationOperator3d {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn matrix(&self) -> &[[f64; 4]; 4] {
        &self.matrix
    }
}

impl Default for CartesianTransformationOperator3d {
    fn default() -> Self {
        Self::new()
    }
}

/// Placeholder for StepShape_SolidModel
pub struct SolidModel {
    name: Arc<str>,
}

impl SolidModel {
    pub fn new(name: Arc<str>) -> Self {
        SolidModel { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Represents a solid replica in STEP format.
/// Inherits from StepShape_SolidModel.
pub struct SolidReplica {
    name: Arc<str>,
    parent_solid: Option<Arc<SolidModel>>,
    transformation: Option<Arc<CartesianTransformationOperator3d>>,
}

impl SolidReplica {
    /// Create a new SolidReplica
    pub fn new() -> Self {
        SolidReplica {
            name: Arc::from(""),
            parent_solid: None,
            transformation: None,
        }
    }

    /// Initialize with name, parent solid, and transformation
    pub fn init(
        &mut self,
        name: Arc<str>,
        parent_solid: Arc<SolidModel>,
        transformation: Arc<CartesianTransformationOperator3d>,
    ) {
        self.name = name;
        self.parent_solid = Some(parent_solid);
        self.transformation = Some(transformation);
    }

    /// Set the parent solid
    pub fn set_parent_solid(&mut self, parent_solid: Arc<SolidModel>) {
        self.parent_solid = Some(parent_solid);
    }

    /// Get the parent solid
    pub fn parent_solid(&self) -> Option<&Arc<SolidModel>> {
        self.parent_solid.as_ref()
    }

    /// Set the transformation
    pub fn set_transformation(&mut self, transformation: Arc<CartesianTransformationOperator3d>) {
        self.transformation = Some(transformation);
    }

    /// Get the transformation
    pub fn transformation(&self) -> Option<&Arc<CartesianTransformationOperator3d>> {
        self.transformation.as_ref()
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

impl Default for SolidReplica {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_replica_creation() {
        let sr = SolidReplica::new();
        assert_eq!(sr.name(), "");
        assert!(sr.parent_solid().is_none());
        assert!(sr.transformation().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut sr = SolidReplica::new();
        let parent = Arc::new(SolidModel::new(Arc::from("parent_solid")));
        let transform = Arc::new(CartesianTransformationOperator3d::new());
        let name: Arc<str> = Arc::from("replica_1");

        sr.init(name.clone(), parent.clone(), transform.clone());

        assert_eq!(sr.name(), "replica_1");
        assert!(sr.parent_solid().is_some());
        assert!(sr.transformation().is_some());
    }

    #[test]
    fn test_set_parent_solid() {
        let mut sr = SolidReplica::new();
        let parent = Arc::new(SolidModel::new(Arc::from("my_solid")));

        sr.set_parent_solid(parent);

        assert!(sr.parent_solid().is_some());
    }

    #[test]
    fn test_set_transformation() {
        let mut sr = SolidReplica::new();
        let transform = Arc::new(CartesianTransformationOperator3d::new());

        sr.set_transformation(transform);

        assert!(sr.transformation().is_some());
    }

    #[test]
    fn test_transformation_matrix() {
        let transform = CartesianTransformationOperator3d::new();
        let matrix = transform.matrix();

        // Check identity matrix
        assert_eq!(matrix[0][0], 1.0);
        assert_eq!(matrix[1][1], 1.0);
        assert_eq!(matrix[2][2], 1.0);
        assert_eq!(matrix[3][3], 1.0);
    }
}
