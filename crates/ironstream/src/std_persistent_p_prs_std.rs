// FILE: std_persistent_p_prs_std.rs
// occt: StdPersistent_PPrsStd

/// Presentation standard persistence
pub struct PPrsStd {
    presentation_type: i32,
}

impl PPrsStd {
    /// Create a new presentation
    pub fn new(presentation_type: i32) -> Self {
        PPrsStd { presentation_type }
    }

    /// Get presentation type
    pub fn presentation_type(&self) -> i32 {
        self.presentation_type
    }

    /// Set presentation type
    pub fn set_presentation_type(&mut self, typ: i32) {
        self.presentation_type = typ;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let prs = PPrsStd::new(1);
        assert_eq!(prs.presentation_type(), 1);
    }

    #[test]
    fn test_set_type() {
        let mut prs = PPrsStd::new(1);
        prs.set_presentation_type(2);
        assert_eq!(prs.presentation_type(), 2);
    }
}
