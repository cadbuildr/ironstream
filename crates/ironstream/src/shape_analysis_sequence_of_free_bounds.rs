// FILE: shape_analysis_sequence_of_free_bounds.rs
// occt: ShapeAnalysis_SequenceOfFreeBounds

pub struct ShapeAnalysisSequenceOfFreeBounds {
    data: Vec<Option<String>>,
}

impl ShapeAnalysisSequenceOfFreeBounds {
    pub fn new() -> Self {
        ShapeAnalysisSequenceOfFreeBounds {
            data: vec![None],
        }
    }

    pub fn append(&mut self, value: String) {
        self.data.push(Some(value));
    }

    pub fn value(&self, index: usize) -> Option<&String> {
        if index > 0 && index < self.data.len() {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for ShapeAnalysisSequenceOfFreeBounds {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut seq = ShapeAnalysisSequenceOfFreeBounds::new();
        seq.append("bound".to_string());
        assert_eq!(seq.len(), 1);
    }
}
