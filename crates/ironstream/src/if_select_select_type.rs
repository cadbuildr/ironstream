// FILE: if_select_select_type.rs
// occt: IFSelect_SelectType

/// Selects or rejects entities based on type matching.
/// Keeps entities whose type is kind of a given type.
#[derive(Clone, Debug)]
pub struct IFSelectSelectType {
    type_name: Option<String>,
}

impl IFSelectSelectType {
    /// Creates a SelectType with no filter
    pub fn new() -> Self {
        Self { type_name: None }
    }

    /// Creates a SelectType for a given type
    pub fn with_type(type_name: String) -> Self {
        Self {
            type_name: Some(type_name),
        }
    }

    /// Sets a type for filter
    pub fn set_type(&mut self, type_name: String) {
        self.type_name = Some(type_name);
    }

    /// Returns the type to be matched for selection
    pub fn type_for_match(&self) -> Option<&str> {
        self.type_name.as_deref()
    }

    /// Returns a text defining the criterium
    pub fn extract_label(&self) -> String {
        match &self.type_name {
            Some(t) => format!("Type = {}", t),
            None => "No Type Filter".to_string(),
        }
    }
}

impl Default for IFSelectSelectType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sel = IFSelectSelectType::new();
        assert_eq!(sel.type_for_match(), None);
    }

    #[test]
    fn test_with_type() {
        let sel = IFSelectSelectType::with_type("MyType".to_string());
        assert_eq!(sel.type_for_match(), Some("MyType"));
    }

    #[test]
    fn test_set_type() {
        let mut sel = IFSelectSelectType::new();
        sel.set_type("Type1".to_string());
        assert_eq!(sel.type_for_match(), Some("Type1"));

        sel.set_type("Type2".to_string());
        assert_eq!(sel.type_for_match(), Some("Type2"));
    }

    #[test]
    fn test_extract_label_no_filter() {
        let sel = IFSelectSelectType::new();
        assert_eq!(sel.extract_label(), "No Type Filter");
    }

    #[test]
    fn test_extract_label_with_type() {
        let sel = IFSelectSelectType::with_type("MyType".to_string());
        assert_eq!(sel.extract_label(), "Type = MyType");
    }

    #[test]
    fn test_default() {
        let sel = IFSelectSelectType::default();
        assert_eq!(sel.type_for_match(), None);
    }
}
