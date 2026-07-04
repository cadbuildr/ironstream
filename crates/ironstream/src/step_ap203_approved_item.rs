// FILE: step_ap203_approved_item.rs
// occt: StepAP203_ApprovedItem

/// Approved item for STEP AP203
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAP203_ApprovedItem {
    ProductDefinition,
    Product,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approved_item_variants() {
        assert_eq!(StepAP203_ApprovedItem::ProductDefinition, StepAP203_ApprovedItem::ProductDefinition);
        assert_ne!(StepAP203_ApprovedItem::ProductDefinition, StepAP203_ApprovedItem::Product);
    }
}
