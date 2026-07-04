// FILE: step_ap214_applied_date_and_time_assignment.rs
// occt: StepAP214_AppliedDateAndTimeAssignment

/// Applied Date and Time Assignment for STEP AP214
pub struct StepAP214_AppliedDateAndTimeAssignment {
    timestamp: i64,
}

impl StepAP214_AppliedDateAndTimeAssignment {
    pub fn new() -> Self {
        StepAP214_AppliedDateAndTimeAssignment { timestamp: 0 }
    }

    pub fn set_timestamp(&mut self, ts: i64) {
        self.timestamp = ts;
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl Default for StepAP214_AppliedDateAndTimeAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let assign = StepAP214_AppliedDateAndTimeAssignment::new();
        assert_eq!(assign.get_timestamp(), 0);
    }
}
