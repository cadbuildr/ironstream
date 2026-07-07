// FILE: shape_analysis_data_map_of_shape_list_of_real.rs
// occt: ShapeAnalysis_DataMapOfShapeListOfReal

use std::collections::BTreeMap;

pub struct ShapeAnalysisDataMapOfShapeListOfReal {
    data: BTreeMap<String, Vec<f64>>,
}

impl ShapeAnalysisDataMapOfShapeListOfReal {
    pub fn new() -> Self {
        ShapeAnalysisDataMapOfShapeListOfReal {
            data: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: String, value: Vec<f64>) {
        self.data.insert(key, value);
    }

    pub fn find(&self, key: &str) -> Option<Vec<f64>> {
        self.data.get(key).cloned()
    }

    pub fn remove(&mut self, key: &str) -> Option<Vec<f64>> {
        self.data.remove(key)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for ShapeAnalysisDataMapOfShapeListOfReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut map = ShapeAnalysisDataMapOfShapeListOfReal::new();
        map.bind("shape1".to_string(), vec![1.0, 2.0, 3.0]);
        assert_eq!(map.find("shape1"), Some(vec![1.0, 2.0, 3.0]));
    }
}
