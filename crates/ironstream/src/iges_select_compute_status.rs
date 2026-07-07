// FILE: iges_select_compute_status.rs
// occt: IGESSelect_ComputeStatus

pub struct IGESSelectComputeStatus;

impl IGESSelectComputeStatus {
    pub fn new() -> Self {
        IGESSelectComputeStatus
    }
}

impl Default for IGESSelectComputeStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectComputeStatus::new();
    }
}
