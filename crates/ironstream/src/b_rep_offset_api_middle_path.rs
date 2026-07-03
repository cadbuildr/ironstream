// FILE: b_rep_offset_api_middle_path.rs
// occt: BRepOffsetAPI_MiddlePath

/// Computes the middle path between two parallel edges.
#[derive(Debug, Clone)]
pub struct BRepOffsetApiMiddlePath {
    /// Whether the operation was successful
    is_done: bool,
}

impl BRepOffsetApiMiddlePath {
    /// Create a new MiddlePath builder
    pub fn new() -> Self {
        Self { is_done: false }
    }

    /// Check if the operation succeeded
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Mark operation as done
    pub fn set_done(&mut self) {
        self.is_done = true;
    }
}

impl Default for BRepOffsetApiMiddlePath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let path = BRepOffsetApiMiddlePath::new();
        assert!(!path.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut path = BRepOffsetApiMiddlePath::new();
        path.set_done();
        assert!(path.is_done());
    }

    #[test]
    fn test_default() {
        let path = BRepOffsetApiMiddlePath::default();
        assert!(!path.is_done());
    }
}
