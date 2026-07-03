// FILE: b_rep_fillet_api_local_operation.rs
// occt: BRepFilletAPI_LocalOperation

#[derive(Clone)]
pub struct BRepFilletAPILocalOperation {
    radius: f64,
    vertex_index: i32,
    is_active: bool,
}

impl BRepFilletAPILocalOperation {
    pub fn new() -> Self {
        Self {
            radius: 0.0,
            vertex_index: 0,
            is_active: false,
        }
    }

    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_vertex_index(&mut self, idx: i32) {
        self.vertex_index = idx;
    }

    pub fn vertex_index(&self) -> i32 {
        self.vertex_index
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl Default for BRepFilletAPILocalOperation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let op = BRepFilletAPILocalOperation::new();
        assert_eq!(op.radius(), 0.0);
        assert!(!op.is_active());
    }

    #[test]
    fn test_set_radius() {
        let mut op = BRepFilletAPILocalOperation::new();
        op.set_radius(5.0);
        assert_eq!(op.radius(), 5.0);
    }

    #[test]
    fn test_activate() {
        let mut op = BRepFilletAPILocalOperation::new();
        op.activate();
        assert!(op.is_active());
        op.deactivate();
        assert!(!op.is_active());
    }
}
