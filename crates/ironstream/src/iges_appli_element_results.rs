// FILE: iges_appli_element_results.rs
// occt: IGESAppli_ElementResults

/// Represents finite element analysis results for an element.
///
/// IGES Type 146 Form 1-12
/// Stores analysis result data indexed by subcase and result type.
#[derive(Clone, Debug)]
pub struct IgesAppliElementResults {
    test_case_id: i32,
    analysis_type: i32,
    data_count: i32,
    values: Vec<f64>,
}

impl IgesAppliElementResults {
    /// Creates a new ElementResults entity.
    pub fn new() -> Self {
        Self {
            test_case_id: 0,
            analysis_type: 0,
            data_count: 0,
            values: Vec::new(),
        }
    }

    /// Initializes with test case, analysis type, and result values.
    pub fn init(&mut self, test_case: i32, analysis: i32, data: Vec<f64>) {
        self.test_case_id = test_case;
        self.analysis_type = analysis;
        self.data_count = data.len() as i32;
        self.values = data;
    }

    /// Returns the test case ID.
    pub fn test_case_id(&self) -> i32 {
        self.test_case_id
    }

    /// Returns the analysis type.
    pub fn analysis_type(&self) -> i32 {
        self.analysis_type
    }

    /// Returns the number of result values.
    pub fn data_count(&self) -> i32 {
        self.data_count
    }

    /// Returns reference to the result values.
    pub fn values(&self) -> &[f64] {
        &self.values
    }
}

impl Default for IgesAppliElementResults {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let results = IgesAppliElementResults::new();
        assert_eq!(results.test_case_id(), 0);
        assert_eq!(results.analysis_type(), 0);
        assert_eq!(results.data_count(), 0);
        assert!(results.values().is_empty());
    }

    #[test]
    fn test_init() {
        let mut results = IgesAppliElementResults::new();
        results.init(1, 2, vec![1.0, 2.0, 3.0]);

        assert_eq!(results.test_case_id(), 1);
        assert_eq!(results.analysis_type(), 2);
        assert_eq!(results.data_count(), 3);
        assert_eq!(results.values(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_clone() {
        let mut results1 = IgesAppliElementResults::new();
        results1.init(5, 3, vec![10.0, 20.0]);

        let results2 = results1.clone();
        assert_eq!(results2.test_case_id(), 5);
        assert_eq!(results2.analysis_type(), 3);
        assert_eq!(results2.values(), &[10.0, 20.0]);
    }
}
