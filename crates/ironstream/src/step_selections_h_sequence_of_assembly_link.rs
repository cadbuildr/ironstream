// FILE: step_selections_h_sequence_of_assembly_link.rs
// occt: STEPSelections_HSequenceOfAssemblyLink

pub struct StepSelectionsHSequenceOfAssemblyLink {
    data: Vec<Option<String>>,
}

impl StepSelectionsHSequenceOfAssemblyLink {
    pub fn new() -> Self {
        StepSelectionsHSequenceOfAssemblyLink {
            data: vec![None],
        }
    }

    pub fn append(&mut self, value: String) {
        self.data.push(Some(value));
    }

    pub fn prepend(&mut self, value: String) {
        self.data.insert(1, Some(value));
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

impl Default for StepSelectionsHSequenceOfAssemblyLink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut seq = StepSelectionsHSequenceOfAssemblyLink::new();
        seq.append("link".to_string());
        assert_eq!(seq.len(), 1);
    }
}
