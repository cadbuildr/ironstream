// FILE: geom_int_sequence_of_parameter_and_orientation.rs
// occt: GeomInt_SequenceOfParameterAndOrientation

#[derive(Clone, Debug)]
pub struct ParameterAndOrientation {
    pub parameter: f64,
    pub orientation: i32,
}

#[derive(Clone, Debug)]
pub struct SequenceOfParameterAndOrientation {
    items: Vec<ParameterAndOrientation>,
}

impl SequenceOfParameterAndOrientation {
    pub fn new() -> Self {
        SequenceOfParameterAndOrientation { items: Vec::new() }
    }

    pub fn append(&mut self, item: ParameterAndOrientation) {
        self.items.push(item);
    }

    pub fn at(&self, i: usize) -> Option<&ParameterAndOrientation> {
        if i > 0 && i <= self.items.len() {
            Some(&self.items[i - 1])
        } else { None }
    }

    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

impl Default for SequenceOfParameterAndOrientation {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = SequenceOfParameterAndOrientation::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = SequenceOfParameterAndOrientation::new();
        seq.append(ParameterAndOrientation {
            parameter: 1.0,
            orientation: 1,
        });
        assert_eq!(seq.len(), 1);
    }
}
