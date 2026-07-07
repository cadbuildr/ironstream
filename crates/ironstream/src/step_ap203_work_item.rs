// FILE: step_ap203_work_item.rs
// occt: StepAP203_WorkItem

/// Work Item for STEP AP203
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAP203_WorkItem {
    ProductDefinition,
    Product,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_item() {
        assert_eq!(StepAP203_WorkItem::ProductDefinition, StepAP203_WorkItem::ProductDefinition);
    }
}
