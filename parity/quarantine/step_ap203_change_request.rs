// FILE: step_ap203_change_request.rs
// occt: StepAP203_ChangeRequest

/// Change Request for STEP AP203
pub struct StepAP203_ChangeRequest {
    request_id: i32,
}

impl StepAP203_ChangeRequest {
    pub fn new() -> Self {
        StepAP203_ChangeRequest { request_id: 0 }
    }

    pub fn set_request_id(&mut self, id: i32) {
        self.request_id = id;
    }

    pub fn get_request_id(&self) -> i32 {
        self.request_id
    }
}

impl Default for StepAP203_ChangeRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let req = StepAP203_ChangeRequest::new();
        assert_eq!(req.get_request_id(), 0);
    }
}
