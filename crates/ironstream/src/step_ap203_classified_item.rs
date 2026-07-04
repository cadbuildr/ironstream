// FILE: step_ap203_classified_item.rs
// occt: StepAP203_ClassifiedItem

/// Classified Item for STEP AP203
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAP203_ClassifiedItem {
    ProductDefinition,
    Product,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classified_item() {
        assert_eq!(StepAP203_ClassifiedItem::ProductDefinition, StepAP203_ClassifiedItem::ProductDefinition);
    }
}
