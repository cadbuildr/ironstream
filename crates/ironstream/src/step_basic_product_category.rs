// FILE: step_basic_product_category.rs
// occt: StepBasic_ProductCategory

#[derive(Clone, Debug)]
pub struct StepBasicProductCategory {
    name: String,
}

impl StepBasicProductCategory {
    pub fn new() -> Self { Self { name: String::new() } }
    pub fn init(&mut self, name: String) { self.name = name; }
    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name: String) { self.name = name; }
}

impl Default for StepBasicProductCategory {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut c = StepBasicProductCategory::new();
        c.init("Hardware".into());
        assert_eq!(c.name(), "Hardware");
    }
}
