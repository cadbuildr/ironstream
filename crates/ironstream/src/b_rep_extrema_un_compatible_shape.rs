// FILE: b_rep_extrema_un_compatible_shape.rs
// occt: BRepExtrema_UnCompatibleShape

/// Exception for incompatible shapes
#[derive(Debug, Clone)]
pub struct IncompatibleShape {
    message: String,
}

impl IncompatibleShape {
    pub fn new(msg: &str) -> Self {
        IncompatibleShape {
            message: msg.to_string(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for IncompatibleShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BRepExtrema_UnCompatibleShape: {}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incompatible_shape_creation() {
        let exc = IncompatibleShape::new("test message");
        assert_eq!(exc.message(), "test message");
    }

    #[test]
    fn test_display() {
        let exc = IncompatibleShape::new("shapes incompatible");
        let s = format!("{}", exc);
        assert!(s.contains("incompatible"));
    }
}
