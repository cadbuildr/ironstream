// FILE: step_ap203_cc_design_security_classification.rs
// occt: StepAP203_CcDesignSecurityClassification

/// CC Design Security Classification for STEP AP203
pub struct StepAP203_CcDesignSecurityClassification {
    classification_level: i32,
}

impl StepAP203_CcDesignSecurityClassification {
    pub fn new() -> Self {
        StepAP203_CcDesignSecurityClassification {
            classification_level: 0,
        }
    }

    pub fn set_classification_level(&mut self, level: i32) {
        self.classification_level = level;
    }

    pub fn get_classification_level(&self) -> i32 {
        self.classification_level
    }
}

impl Default for StepAP203_CcDesignSecurityClassification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sec = StepAP203_CcDesignSecurityClassification::new();
        assert_eq!(sec.get_classification_level(), 0);
    }
}
