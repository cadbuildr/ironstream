// FILE: step_ap203_contracted_item.rs
// occt: StepAP203_ContractedItem

/// Contracted Item for STEP AP203
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAP203_ContractedItem {
    ProductDefinition,
    Product,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contracted_item() {
        assert_eq!(StepAP203_ContractedItem::ProductDefinition, StepAP203_ContractedItem::ProductDefinition);
    }
}
