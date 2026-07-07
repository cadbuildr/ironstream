// FILE: shape_analysis_h_sequence_of_free_bounds.rs
// occt: ShapeAnalysis_HSequenceOfFreeBounds

pub struct ShapeAnalysisHSequenceOfFreeBounds {
    data: Vec<Option<String>>,
}

impl ShapeAnalysisHSequenceOfFreeBounds {
    pub fn new() -> Self {
        ShapeAnalysisHSequenceOfFreeBounds {
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

impl Default for ShapeAnalysisHSequenceOfFreeBounds {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut seq = ShapeAnalysisHSequenceOfFreeBounds::new();
        seq.append("bound".to_string());
        assert!(!seq.is_empty());
    }
}
