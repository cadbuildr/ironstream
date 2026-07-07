// FILE: step_geom_surface_replica.rs
// occt: StepGeom_SurfaceReplica

/// Represents a replica (copy/instance) of a surface
pub struct StepGeomSurfaceReplica {
    name: String,
    parent_surface_id: i32,
    /// Transform matrix representation [scale, rotation...]
    transform: Vec<f64>,
}

impl StepGeomSurfaceReplica {
    pub fn new(name: String, parent_surface_id: i32) -> Self {
        StepGeomSurfaceReplica {
            name,
            parent_surface_id,
            transform: vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent_surface_id(&self) -> i32 {
        self.parent_surface_id
    }

    pub fn transform(&self) -> &[f64] {
        &self.transform
    }

    pub fn set_transform(&mut self, t: Vec<f64>) {
        self.transform = t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_replica() {
        let replica = StepGeomSurfaceReplica::new("SurfaceReplica1".to_string(), 1);
        assert_eq!(replica.name(), "SurfaceReplica1");
        assert_eq!(replica.parent_surface_id(), 1);
    }

    #[test]
    fn test_transform() {
        let replica = StepGeomSurfaceReplica::new("SurfaceReplica1".to_string(), 1);
        assert_eq!(replica.transform().len(), 9);
    }
}
