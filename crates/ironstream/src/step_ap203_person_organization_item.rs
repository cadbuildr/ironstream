// FILE: step_ap203_person_organization_item.rs
// occt: StepAP203_PersonOrganizationItem

/// Person Organization Item for STEP AP203
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAP203_PersonOrganizationItem {
    ProductDefinition,
    Product,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_organization_item() {
        assert_eq!(StepAP203_PersonOrganizationItem::ProductDefinition, StepAP203_PersonOrganizationItem::ProductDefinition);
    }
}
