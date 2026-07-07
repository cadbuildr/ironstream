// FILE: step_ap203_start_work.rs
// occt: StepAP203_StartWork

/// Start Work for STEP AP203
pub struct StepAP203_StartWork {
    work_id: i32,
}

impl StepAP203_StartWork {
    pub fn new() -> Self {
        StepAP203_StartWork { work_id: 0 }
    }

    pub fn set_work_id(&mut self, id: i32) {
        self.work_id = id;
    }

    pub fn get_work_id(&self) -> i32 {
        self.work_id
    }
}

impl Default for StepAP203_StartWork {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let work = StepAP203_StartWork::new();
        assert_eq!(work.get_work_id(), 0);
    }
}
