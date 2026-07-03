// FILE: math_gauss_multiple_integration.rs
// occt: math_GaussMultipleIntegration

/// Multi-dimensional Gauss integration.
pub struct GaussMultipleIntegration {
    order: usize,
}

impl GaussMultipleIntegration {
    pub fn new(order: usize) -> Self {
        GaussMultipleIntegration { order }
    }

    pub fn get_order(&self) -> usize {
        self.order
    }

    pub fn set_order(&mut self, order: usize) {
        self.order = order;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_multiple_integration() {
        let int = GaussMultipleIntegration::new(3);
        assert_eq!(int.get_order(), 3);
    }
}
