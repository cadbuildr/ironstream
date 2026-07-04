// FILE: step_ap209_construct.rs
// occt: StepAP209_Construct

/// Constructor for STEP AP209
pub struct StepAP209_Construct;

impl StepAP209_Construct {
    pub fn new() -> Self {
        StepAP209_Construct
    }
}

impl Default for StepAP209_Construct {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _construct = StepAP209_Construct::new();
    }
}
