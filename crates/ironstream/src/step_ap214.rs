// FILE: step_ap214.rs
// occt: StepAP214

/// STEP AP214 utilities
pub struct StepAP214;

impl StepAP214 {
    pub fn new() -> Self {
        StepAP214
    }
}

impl Default for StepAP214 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ap214 = StepAP214::new();
    }
}
