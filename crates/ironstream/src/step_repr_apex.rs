// FILE: step_repr_apex.rs
// occt: StepRepr_Apex

/// Representation of STEP entity Apex.
#[derive(Clone, Debug, Default)]
pub struct StepReprApex;

impl StepReprApex {
    pub fn new() -> Self {
        StepReprApex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _apex = StepReprApex::new();
    }
}
