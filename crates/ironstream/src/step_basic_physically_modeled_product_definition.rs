// FILE: step_basic_physically_modeled_product_definition.rs
// occt: StepBasic_PhysicallyModeledProductDefinition

#[derive(Clone, Debug)]
pub struct StepBasicPhysicallyModeledProductDefinition {
    product_definition_id: String,
}

impl StepBasicPhysicallyModeledProductDefinition {
    pub fn new() -> Self {
        Self { product_definition_id: String::new() }
    }

    pub fn init(&mut self, id: String) {
        self.product_definition_id = id;
    }

    pub fn product_definition(&self) -> &str {
        &self.product_definition_id
    }
}

impl Default for StepBasicPhysicallyModeledProductDefinition {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut p = StepBasicPhysicallyModeledProductDefinition::new();
        p.init("PD-1".into());
        assert_eq!(p.product_definition(), "PD-1");
    }
}
