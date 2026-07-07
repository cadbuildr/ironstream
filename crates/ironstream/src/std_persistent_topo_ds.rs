// FILE: std_persistent_topo_ds.rs
// occt: StdPersistent_TopoDS

/// TopoDS persistence management
pub struct TopoDS {
    shape_type: i32,
}

impl TopoDS {
    /// Create a new TopoDS persistent
    pub fn new() -> Self {
        TopoDS { shape_type: 0 }
    }

    /// Get shape type
    pub fn shape_type(&self) -> i32 {
        self.shape_type
    }

    /// Set shape type
    pub fn set_shape_type(&mut self, typ: i32) {
        self.shape_type = typ;
    }
}

impl Default for TopoDS {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let topo = TopoDS::new();
        assert_eq!(topo.shape_type(), 0);
    }

    #[test]
    fn test_set_shape_type() {
        let mut topo = TopoDS::new();
        topo.set_shape_type(2);
        assert_eq!(topo.shape_type(), 2);
    }
}
