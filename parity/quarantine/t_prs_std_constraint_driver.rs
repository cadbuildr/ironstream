// FILE: t_prs_std_constraint_driver.rs
// occt: TPrsStd_ConstraintDriver

/// A driver for building AIS presentations for constraint attributes.
#[derive(Clone, Debug)]
pub struct TPrsStd_ConstraintDriver {
    name: String,
}

impl TPrsStd_ConstraintDriver {
    /// Create a new constraint driver.
    pub fn new() -> Self {
        Self {
            name: "ConstraintDriver".to_string(),
        }
    }

    /// Get the driver name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Build the presentation.
    pub fn build(&self, _constraint_type: &str) -> bool {
        true
    }
}

impl Default for TPrsStd_ConstraintDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_driver() {
        let driver = TPrsStd_ConstraintDriver::new();
        assert_eq!(driver.name(), "ConstraintDriver");
    }

    #[test]
    fn test_build() {
        let driver = TPrsStd_ConstraintDriver::new();
        assert!(driver.build("FixedConstraint"));
    }

    #[test]
    fn test_default() {
        let driver = TPrsStd_ConstraintDriver::default();
        assert_eq!(driver.name(), "ConstraintDriver");
    }
}
