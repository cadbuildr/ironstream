// FILE: step_ap203_cc_design_specification_reference.rs
// occt: StepAP203_CcDesignSpecificationReference

/// CC Design Specification Reference for STEP AP203
pub struct StepAP203_CcDesignSpecificationReference {
    spec_id: i32,
}

impl StepAP203_CcDesignSpecificationReference {
    pub fn new() -> Self {
        StepAP203_CcDesignSpecificationReference { spec_id: 0 }
    }

    pub fn set_spec_id(&mut self, id: i32) {
        self.spec_id = id;
    }

    pub fn get_spec_id(&self) -> i32 {
        self.spec_id
    }
}

impl Default for StepAP203_CcDesignSpecificationReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let spec = StepAP203_CcDesignSpecificationReference::new();
        assert_eq!(spec.get_spec_id(), 0);
    }
}
