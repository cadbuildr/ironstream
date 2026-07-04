// FILE: step_ap203_cc_design_certification.rs
// occt: StepAP203_CcDesignCertification

/// CC Design Certification for STEP AP203
pub struct StepAP203_CcDesignCertification {
    cert_id: i32,
}

impl StepAP203_CcDesignCertification {
    pub fn new() -> Self {
        StepAP203_CcDesignCertification { cert_id: 0 }
    }

    pub fn set_cert_id(&mut self, id: i32) {
        self.cert_id = id;
    }

    pub fn get_cert_id(&self) -> i32 {
        self.cert_id
    }
}

impl Default for StepAP203_CcDesignCertification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let cert = StepAP203_CcDesignCertification::new();
        assert_eq!(cert.get_cert_id(), 0);
    }
}
