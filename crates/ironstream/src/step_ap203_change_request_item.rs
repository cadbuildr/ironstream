// FILE: step_ap203_change_request_item.rs
// occt: StepAP203_ChangeRequestItem

/// Change Request Item for STEP AP203
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAP203_ChangeRequestItem {
    ProductDefinition,
    Product,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_request_item() {
        assert_eq!(StepAP203_ChangeRequestItem::ProductDefinition, StepAP203_ChangeRequestItem::ProductDefinition);
    }
}
