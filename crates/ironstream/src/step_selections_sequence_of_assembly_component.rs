// FILE: step_selections_sequence_of_assembly_component.rs
// occt: STEPSelections_SequenceOfAssemblyComponent

pub struct StepSelectionsSequenceOfAssemblyComponent {
    data: Vec<Option<String>>,
}

impl StepSelectionsSequenceOfAssemblyComponent {
    pub fn new() -> Self {
        StepSelectionsSequenceOfAssemblyComponent {
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

    pub fn set_value(&mut self, index: usize, value: String) -> bool {
        if index > 0 && index < self.data.len() {
            self.data[index] = Some(value);
            true
        } else {
            false
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

    pub fn lower(&self) -> usize {
        1
    }

    pub fn upper(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }
}

impl Default for StepSelectionsSequenceOfAssemblyComponent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut seq = StepSelectionsSequenceOfAssemblyComponent::new();
        seq.append("comp1".to_string());
        assert_eq!(seq.len(), 1);
    }
}
