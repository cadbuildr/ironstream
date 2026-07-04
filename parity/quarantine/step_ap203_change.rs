// FILE: step_ap203_change.rs
// occt: StepAP203_Change

/// Change for STEP AP203
pub struct StepAP203_Change {
    change_id: i32,
}

impl StepAP203_Change {
    pub fn new() -> Self {
        StepAP203_Change { change_id: 0 }
    }

    pub fn set_change_id(&mut self, id: i32) {
        self.change_id = id;
    }

    pub fn get_change_id(&self) -> i32 {
        self.change_id
    }
}

impl Default for StepAP203_Change {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let change = StepAP203_Change::new();
        assert_eq!(change.get_change_id(), 0);
    }
}
