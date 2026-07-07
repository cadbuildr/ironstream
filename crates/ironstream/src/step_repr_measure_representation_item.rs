// FILE: step_repr_measure_representation_item.rs
// occt: StepRepr_MeasureRepresentationItem

/// StepRepr_MeasureRepresentationItem: A measure representation item
/// Implements a measure_representation_item entity which is used for storing
/// validation properties (e.g. area) for shapes
/// Inherits from StepRepr_RepresentationItem
#[derive(Clone, Debug)]
pub struct StepReprMeasureRepresentationItem {
    name: String,
    measure: f64, // Simplified: storing measure value
}

impl StepReprMeasureRepresentationItem {
    /// Creates empty object
    pub fn new() -> Self {
        StepReprMeasureRepresentationItem {
            name: String::new(),
            measure: 0.0,
        }
    }

    /// Init all fields
    pub fn init(&mut self, name: String, measure: f64) {
        self.name = name;
        self.measure = measure;
    }

    /// Set measure
    pub fn set_measure(&mut self, measure: f64) {
        self.measure = measure;
    }

    /// Get measure
    pub fn measure(&self) -> f64 {
        self.measure
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for StepReprMeasureRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = StepReprMeasureRepresentationItem::new();
        assert_eq!(item.name(), "");
        assert_eq!(item.measure(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut item = StepReprMeasureRepresentationItem::new();
        item.init("area".to_string(), 25.5);
        assert_eq!(item.name(), "area");
        assert_eq!(item.measure(), 25.5);
    }

    #[test]
    fn test_set_measure() {
        let mut item = StepReprMeasureRepresentationItem::new();
        item.set_measure(100.0);
        assert_eq!(item.measure(), 100.0);
    }

    #[test]
    fn test_set_name() {
        let mut item = StepReprMeasureRepresentationItem::new();
        item.set_name("volume".to_string());
        assert_eq!(item.name(), "volume");
    }
}
