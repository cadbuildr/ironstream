// FILE: step_ap203_specified_item.rs
// occt: StepAP203_SpecifiedItem

/// Specified Item for STEP AP203
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAP203_SpecifiedItem {
    ProductDefinition,
    Product,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specified_item() {
        assert_eq!(StepAP203_SpecifiedItem::ProductDefinition, StepAP203_SpecifiedItem::ProductDefinition);
    }
}
