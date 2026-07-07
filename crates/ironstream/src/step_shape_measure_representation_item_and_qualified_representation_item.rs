// FILE: step_shape_measure_representation_item_and_qualified_representation_item.rs
// occt: StepShape_MeasureRepresentationItemAndQualifiedRepresentationItem

//! Added for Dimensional Tolerances
//! Complex Type between MeasureRepresentationItem and
//! QualifiedRepresentationItem

#[derive(Clone, Debug)]
pub struct MeasureRepresentationItemAndQualifiedRepresentationItem {
    name: String,
    measure: Option<String>,
    qualifiers: Vec<String>,
}

impl MeasureRepresentationItemAndQualifiedRepresentationItem {
    /// Constructor
    pub fn new() -> Self {
        MeasureRepresentationItemAndQualifiedRepresentationItem {
            name: String::new(),
            measure: None,
            qualifiers: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, measure: Option<String>, qualifiers: Vec<String>) {
        self.name = name;
        self.measure = measure;
        self.qualifiers = qualifiers;
    }

    /// Set Measure
    pub fn set_measure(&mut self, measure: Option<String>) {
        self.measure = measure;
    }

    /// Returns Measure
    pub fn measure(&self) -> &Option<String> {
        &self.measure
    }

    /// Returns Qualifiers
    pub fn qualifiers(&self) -> &[String] {
        &self.qualifiers
    }

    /// Returns number of qualifiers
    pub fn nb_qualifiers(&self) -> usize {
        self.qualifiers.len()
    }

    /// Set Qualifiers
    pub fn set_qualifiers(&mut self, qualifiers: Vec<String>) {
        self.qualifiers = qualifiers;
    }

    /// Returns qualifier at index (1-based)
    pub fn qualifiers_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.qualifiers.len() {
            Some(&self.qualifiers[num - 1])
        } else {
            None
        }
    }

    /// Set qualifier at index (1-based)
    pub fn set_qualifiers_value(&mut self, num: usize, qualifier: String) {
        if num > 0 && num <= self.qualifiers.len() {
            self.qualifiers[num - 1] = qualifier;
        }
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for MeasureRepresentationItemAndQualifiedRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mraqr = MeasureRepresentationItemAndQualifiedRepresentationItem::new();
        assert_eq!(mraqr.name(), "");
        assert!(mraqr.measure().is_none());
        assert_eq!(mraqr.nb_qualifiers(), 0);
    }

    #[test]
    fn test_init() {
        let mut mraqr = MeasureRepresentationItemAndQualifiedRepresentationItem::new();
        mraqr.init(
            "Item1".to_string(),
            Some("measure1".to_string()),
            vec!["q1".to_string(), "q2".to_string()],
        );
        assert_eq!(mraqr.name(), "Item1");
        assert_eq!(mraqr.nb_qualifiers(), 2);
    }

    #[test]
    fn test_set_measure() {
        let mut mraqr = MeasureRepresentationItemAndQualifiedRepresentationItem::new();
        mraqr.set_measure(Some("measure1".to_string()));
        assert_eq!(mraqr.measure(), &Some("measure1".to_string()));
    }
}
