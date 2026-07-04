// FILE: step_basic_ahead_or_behind.rs
// occt: StepBasic_AheadOrBehind

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepBasic_AheadOrBehind {
    Ahead,
    Exact,
    Behind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ahead_or_behind_variants() {
        assert_eq!(StepBasic_AheadOrBehind::Ahead, StepBasic_AheadOrBehind::Ahead);
        assert_eq!(StepBasic_AheadOrBehind::Exact, StepBasic_AheadOrBehind::Exact);
        assert_eq!(StepBasic_AheadOrBehind::Behind, StepBasic_AheadOrBehind::Behind);
        assert_ne!(StepBasic_AheadOrBehind::Ahead, StepBasic_AheadOrBehind::Exact);
    }

    #[test]
    fn test_ahead_or_behind_copy() {
        let aob = StepBasic_AheadOrBehind::Ahead;
        let aob2 = aob;
        assert_eq!(aob, aob2);
    }
}
