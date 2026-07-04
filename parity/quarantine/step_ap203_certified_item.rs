// FILE: step_ap203_certified_item.rs
// occt: StepAP203_CertifiedItem

/// Certified Item for STEP AP203
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAP203_CertifiedItem {
    ProductDefinition,
    Product,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certified_item_variants() {
        assert_eq!(StepAP203_CertifiedItem::ProductDefinition, StepAP203_CertifiedItem::ProductDefinition);
    }
}
