// FILE: b_rep_offset_api_make_draft.rs
// occt: BRepOffsetAPI_MakeDraft

/// Build a draft surface along a wire.
/// Creates a draft surface with a given direction and angle.
#[derive(Debug, Clone)]
pub struct BRepOffsetApiMakeDraft {
    /// Whether the operation was successful
    is_done: bool,
    /// Direction components (unit vector)
    dir_x: f64,
    dir_y: f64,
    dir_z: f64,
    /// Draft angle in radians
    angle: f64,
}

impl BRepOffsetApiMakeDraft {
    /// Create a draft surface builder
    pub fn new(dir_x: f64, dir_y: f64, dir_z: f64, angle: f64) -> Self {
        Self {
            is_done: false,
            dir_x,
            dir_y,
            dir_z,
            angle,
        }
    }

    /// Check if the operation succeeded
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Mark operation as done
    pub fn set_done(&mut self) {
        self.is_done = true;
    }

    /// Get draft direction
    pub fn direction(&self) -> (f64, f64, f64) {
        (self.dir_x, self.dir_y, self.dir_z)
    }

    /// Get draft angle
    pub fn angle(&self) -> f64 {
        self.angle
    }
}

impl Default for BRepOffsetApiMakeDraft {
    fn default() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let draft = BRepOffsetApiMakeDraft::new(0.0, 0.0, 1.0, 0.785);
        assert!(!draft.is_done());
        let (dx, dy, dz) = draft.direction();
        assert_eq!(dx, 0.0);
        assert_eq!(dy, 0.0);
        assert_eq!(dz, 1.0);
        assert_eq!(draft.angle(), 0.785);
    }

    #[test]
    fn test_set_done() {
        let mut draft = BRepOffsetApiMakeDraft::new(0.0, 0.0, 1.0, 0.785);
        draft.set_done();
        assert!(draft.is_done());
    }

    #[test]
    fn test_default() {
        let draft = BRepOffsetApiMakeDraft::default();
        assert!(!draft.is_done());
    }
}
