// FILE: step_geom_point_replica.rs
// occt: StepGeom_PointReplica

/// Represents a replica (copy) of a point
pub struct StepGeomPointReplica {
    name: String,
    parent_point_id: i32,
    /// Transform matrix as [x, y, z, angle] or similar
    transform: [f64; 4],
}

impl StepGeomPointReplica {
    pub fn new(name: String, parent_id: i32) -> Self {
        StepGeomPointReplica {
            name,
            parent_point_id: parent_id,
            transform: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent_point_id(&self) -> i32 {
        self.parent_point_id
    }

    pub fn transform(&self) -> [f64; 4] {
        self.transform
    }

    pub fn set_transform(&mut self, t: [f64; 4]) {
        self.transform = t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_point_replica() {
        let replica = StepGeomPointReplica::new("PointReplica1".to_string(), 5);
        assert_eq!(replica.name(), "PointReplica1");
        assert_eq!(replica.parent_point_id(), 5);
    }

    #[test]
    fn test_set_transform() {
        let mut replica = StepGeomPointReplica::new("PointReplica1".to_string(), 5);
        replica.set_transform([1.0, 2.0, 3.0, 45.0]);
        assert_eq!(replica.transform(), [1.0, 2.0, 3.0, 45.0]);
    }
}
