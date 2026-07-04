// FILE: step_element_analysis_item_within_representation.rs
// occt: StepElement_AnalysisItemWithinRepresentation

pub struct AnalysisItemWithinRepresentation {
    pub name: Option<String>,
    pub description: Option<String>,
    pub item: Option<String>,
    pub rep: Option<String>,
}

impl AnalysisItemWithinRepresentation {
    pub fn new() -> Self {
        AnalysisItemWithinRepresentation {
            name: None,
            description: None,
            item: None,
            rep: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        item: Option<String>,
        rep: Option<String>,
    ) {
        self.name = name;
        self.description = description;
        self.item = item;
        self.rep = rep;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_item(&mut self, item: String) {
        self.item = Some(item);
    }

    pub fn get_item(&self) -> Option<&str> {
        self.item.as_deref()
    }

    pub fn set_rep(&mut self, rep: String) {
        self.rep = Some(rep);
    }

    pub fn get_rep(&self) -> Option<&str> {
        self.rep.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let item = AnalysisItemWithinRepresentation::new();
        assert!(item.name.is_none());
    }

    #[test]
    fn test_init() {
        let mut item = AnalysisItemWithinRepresentation::new();
        item.init(
            Some("name".to_string()),
            Some("desc".to_string()),
            Some("itm".to_string()),
            Some("rep".to_string()),
        );
        assert_eq!(item.get_name(), Some("name"));
        assert_eq!(item.get_item(), Some("itm"));
        assert_eq!(item.get_rep(), Some("rep"));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut item = AnalysisItemWithinRepresentation::new();
        item.set_name("analysis".to_string());
        assert_eq!(item.get_name(), Some("analysis"));
    }
}
