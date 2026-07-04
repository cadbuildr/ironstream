// FILE: step_element_element_order.rs
// occt: StepElement_ElementOrder

/// Enumeration for element order (topology).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementOrder {
    Linear,
    Quadratic,
    Cubic,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_ne!(ElementOrder::Linear, ElementOrder::Quadratic);
        assert_ne!(ElementOrder::Quadratic, ElementOrder::Cubic);
        assert_ne!(ElementOrder::Linear, ElementOrder::Cubic);
    }

    #[test]
    fn test_copy() {
        let order = ElementOrder::Quadratic;
        let order2 = order;
        assert_eq!(order, order2);
    }

    #[test]
    fn test_debug() {
        let order = ElementOrder::Cubic;
        assert_eq!(format!("{:?}", order), "Cubic");
    }
}
