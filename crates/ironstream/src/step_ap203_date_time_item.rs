// FILE: step_ap203_date_time_item.rs
// occt: StepAP203_DateTimeItem

/// Date Time Item for STEP AP203
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAP203_DateTimeItem {
    ProductDefinition,
    Product,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_time_item() {
        assert_eq!(StepAP203_DateTimeItem::ProductDefinition, StepAP203_DateTimeItem::ProductDefinition);
    }
}
