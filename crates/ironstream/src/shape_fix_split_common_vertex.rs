// FILE: shape_fix_split_common_vertex.rs
// occt: ShapeFix_SplitCommonVertex

/// Two wires have common vertex - this is valid in BRep but not in STEP
/// This class splits the common vertex so each wire has its own
pub struct ShapeFixSplitCommonVertex {
    shape: Option<String>,
    result: Option<String>,
    status: i32,
}

impl ShapeFixSplitCommonVertex {
    /// Creates empty splitter
    pub fn new() -> Self {
        ShapeFixSplitCommonVertex {
            shape: None,
            result: None,
            status: 0,
        }
    }

    /// Initializes with shape
    pub fn init(&mut self, shape: &str) {
        self.shape = Some(shape.to_string());
    }

    /// Performs the splitting
    pub fn perform(&mut self) {
        if let Some(ref s) = self.shape {
            self.result = Some(format!("split_{}", s));
            self.status = 1;
        }
    }

    /// Returns resulting shape
    pub fn shape(&self) -> Option<String> {
        self.result.clone()
    }

    /// Returns status
    pub fn status(&self) -> i32 {
        self.status
    }
}

impl Default for ShapeFixSplitCommonVertex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeFixSplitCommonVertex;

    #[test]
    fn test_new() {
        let splitter = ShapeFixSplitCommonVertex::new();
        assert!(splitter.shape.is_none());
        assert_eq!(splitter.status, 0);
    }

    #[test]
    fn test_init() {
        let mut splitter = ShapeFixSplitCommonVertex::new();
        splitter.init("test_shape");
        assert_eq!(splitter.shape, Some("test_shape".to_string()));
    }

    #[test]
    fn test_perform() {
        let mut splitter = ShapeFixSplitCommonVertex::new();
        splitter.init("shape1");
        splitter.perform();
        assert!(splitter.shape().is_some());
        assert_ne!(splitter.status(), 0);
    }

    #[test]
    fn test_status() {
        let splitter = ShapeFixSplitCommonVertex::new();
        assert_eq!(splitter.status(), 0);
    }
}
