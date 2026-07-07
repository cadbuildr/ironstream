// FILE: step_data_logical.rs
// occt: StepData_Logical

//! A Standard Definition for STEP (which knows Boolean too)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepDataLogical {
    False,
    True,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_variants() {
        assert_ne!(StepDataLogical::False, StepDataLogical::True);
        assert_ne!(StepDataLogical::True, StepDataLogical::Unknown);
        assert_ne!(StepDataLogical::False, StepDataLogical::Unknown);
    }

    #[test]
    fn test_logical_clone() {
        let val = StepDataLogical::True;
        let cloned = val.clone();
        assert_eq!(val, cloned);
    }
}
