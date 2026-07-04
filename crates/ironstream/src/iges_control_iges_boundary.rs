// FILE: iges_control_iges_boundary.rs
// occt: IGESControl_IGESBoundary

/// IGES Boundary representation.
pub struct IgesControlIgesBoundary {
    boundary_type: i32,
}

impl IgesControlIgesBoundary {
    pub fn new() -> Self {
        Self { boundary_type: 0 }
    }

    pub fn create(&mut self, boundary_type: i32) {
        self.boundary_type = boundary_type;
    }

    pub fn boundary_type(&self) -> i32 {
        self.boundary_type
    }

    pub fn to_string(&self) -> String {
        format!("IGESBoundary(type={})", self.boundary_type)
    }
}

impl Default for IgesControlIgesBoundary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let boundary = IgesControlIgesBoundary::new();
        assert_eq!(boundary.boundary_type(), 0);
    }

    #[test]
    fn test_create() {
        let mut boundary = IgesControlIgesBoundary::new();
        boundary.create(1);
        assert_eq!(boundary.boundary_type(), 1);
    }

    #[test]
    fn test_to_string() {
        let mut boundary = IgesControlIgesBoundary::new();
        boundary.create(2);
        assert!(boundary.to_string().contains("type=2"));
    }
}
