// FILE: step_basic_product.rs
// occt: StepBasic_Product

#[derive(Clone, Debug)]
pub struct StepBasicProduct {
    id: String,
    name: String,
    description: Option<String>,
    has_description: bool,
}

impl StepBasicProduct {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: None,
            has_description: false,
        }
    }

    pub fn init(&mut self, id: String, name: String, has_desc: bool, desc: Option<String>) {
        self.id = id;
        self.name = name;
        self.has_description = has_desc;
        self.description = if has_desc { desc } else { None };
    }

    pub fn id(&self) -> &str { &self.id }
    pub fn set_id(&mut self, id: String) { self.id = id; }
    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name: String) { self.name = name; }
    pub fn description(&self) -> Option<&str> { self.description.as_deref() }
    pub fn set_description(&mut self, desc: String) { self.description = Some(desc); }
    pub fn has_description(&self) -> bool { self.has_description }
}

impl Default for StepBasicProduct {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init() {
        let mut p = StepBasicProduct::new();
        p.init("P-1".into(), "Widget".into(), true, Some("A widget".into()));
        assert_eq!(p.id(), "P-1");
        assert_eq!(p.name(), "Widget");
        assert!(p.has_description());
    }
}
