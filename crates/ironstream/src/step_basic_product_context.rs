// FILE: step_basic_product_context.rs
// occt: StepBasic_ProductContext

#[derive(Clone, Debug)]
pub struct StepBasicProductContext {
    name: String,
    discipline: String,
}

impl StepBasicProductContext {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            discipline: String::new(),
        }
    }

    pub fn init(&mut self, name: String, discipline: String) {
        self.name = name;
        self.discipline = discipline;
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name: String) { self.name = name; }
    pub fn discipline(&self) -> &str { &self.discipline }
    pub fn set_discipline(&mut self, d: String) { self.discipline = d; }
}

impl Default for StepBasicProductContext {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init() {
        let mut c = StepBasicProductContext::new();
        c.init("Product".into(), "Mechanical".into());
        assert_eq!(c.name(), "Product");
        assert_eq!(c.discipline(), "Mechanical");
    }
}
