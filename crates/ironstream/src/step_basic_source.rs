// FILE: step_basic_source.rs
// occt: StepBasic_Source

/// Enumeration representing the source of a product (whether it was made or bought).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepBasicSource {
    Made,
    Bought,
    NotKnown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_made() {
        let s = StepBasicSource::Made;
        assert_eq!(s, StepBasicSource::Made);
    }

    #[test]
    fn test_source_bought() {
        let s = StepBasicSource::Bought;
        assert_eq!(s, StepBasicSource::Bought);
    }

    #[test]
    fn test_source_not_known() {
        let s = StepBasicSource::NotKnown;
        assert_eq!(s, StepBasicSource::NotKnown);
    }
}
