// FILE: step_basic_product_concept_context.rs
// occt: StepBasic_ProductConceptContext

#[derive(Clone, Debug)]
pub struct StepBasicProductConceptContext {
    name: String,
}

impl StepBasicProductConceptContext {
    pub fn new() -> Self { Self { name: String::new() } }
    pub fn init(&mut self, name: String) { self.name = name; }
    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name: String) { self.name = name; }
}

impl Default for StepBasicProductConceptContext {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut c = StepBasicProductConceptContext::new();
        c.init("context".into());
        assert_eq!(c.name(), "context");
    }
}
