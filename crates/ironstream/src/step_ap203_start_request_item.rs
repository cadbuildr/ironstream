// FILE: step_ap203_start_request_item.rs
// occt: StepAP203_StartRequestItem

/// Start Request Item for STEP AP203
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAP203_StartRequestItem {
    ProductDefinition,
    Product,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_request_item() {
        assert_eq!(StepAP203_StartRequestItem::ProductDefinition, StepAP203_StartRequestItem::ProductDefinition);
    }
}
