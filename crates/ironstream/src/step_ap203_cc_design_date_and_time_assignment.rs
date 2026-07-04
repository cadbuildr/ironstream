// FILE: step_ap203_cc_design_date_and_time_assignment.rs
// occt: StepAP203_CcDesignDateAndTimeAssignment

/// CC Design Date and Time Assignment for STEP AP203
pub struct StepAP203_CcDesignDateAndTimeAssignment {
    timestamp: i64,
}

impl StepAP203_CcDesignDateAndTimeAssignment {
    pub fn new() -> Self {
        StepAP203_CcDesignDateAndTimeAssignment { timestamp: 0 }
    }

    pub fn set_timestamp(&mut self, ts: i64) {
        self.timestamp = ts;
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl Default for StepAP203_CcDesignDateAndTimeAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let assign = StepAP203_CcDesignDateAndTimeAssignment::new();
        assert_eq!(assign.get_timestamp(), 0);
    }
}
