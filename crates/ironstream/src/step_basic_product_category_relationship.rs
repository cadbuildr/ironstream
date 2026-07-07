// FILE: step_basic_product_category_relationship.rs
// occt: StepBasic_ProductCategoryRelationship

#[derive(Clone, Debug)]
pub struct StepBasicProductCategoryRelationship {
    name: String,
    category: String,
    sub_category: String,
}

impl StepBasicProductCategoryRelationship {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            category: String::new(),
            sub_category: String::new(),
        }
    }

    pub fn init(&mut self, name: String, category: String, sub_category: String) {
        self.name = name;
        self.category = category;
        self.sub_category = sub_category;
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name: String) { self.name = name; }
    pub fn category(&self) -> &str { &self.category }
    pub fn set_category(&mut self, c: String) { self.category = c; }
    pub fn sub_category(&self) -> &str { &self.sub_category }
    pub fn set_sub_category(&mut self, c: String) { self.sub_category = c; }
}

impl Default for StepBasicProductCategoryRelationship {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init() {
        let mut r = StepBasicProductCategoryRelationship::new();
        r.init("rel".into(), "cat".into(), "subcat".into());
        assert_eq!(r.name(), "rel");
        assert_eq!(r.category(), "cat");
        assert_eq!(r.sub_category(), "subcat");
    }
}
