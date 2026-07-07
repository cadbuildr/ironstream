// FILE: b_rep_list_of_curve_representation.rs
// occt: BRep_ListOfCurveRepresentation

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct CurveRepresentation {
    curve_id: usize,
    representation_type: String,
    param_range: (f64, f64),
}

impl CurveRepresentation {
    pub fn new(curve_id: usize, representation_type: String, start: f64, end: f64) -> Self {
        CurveRepresentation {
            curve_id,
            representation_type,
            param_range: (start, end),
        }
    }

    pub fn curve_id(&self) -> usize {
        self.curve_id
    }

    pub fn representation_type(&self) -> &str {
        &self.representation_type
    }

    pub fn param_range(&self) -> (f64, f64) {
        self.param_range
    }
}

pub struct BrepListOfCurveRepresentation {
    data: VecDeque<CurveRepresentation>,
}

impl BrepListOfCurveRepresentation {
    pub fn new() -> Self {
        BrepListOfCurveRepresentation {
            data: VecDeque::new(),
        }
    }

    pub fn append(&mut self, representation: CurveRepresentation) {
        self.data.push_back(representation);
    }

    pub fn prepend(&mut self, representation: CurveRepresentation) {
        self.data.push_front(representation);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&CurveRepresentation> {
        self.data.get(index)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &CurveRepresentation> {
        self.data.iter()
    }
}

impl Default for BrepListOfCurveRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_representation_creation() {
        let rep = CurveRepresentation::new(1, "3D".to_string(), 0.0, 1.0);
        assert_eq!(rep.curve_id(), 1);
        assert_eq!(rep.representation_type(), "3D");
    }

    #[test]
    fn test_list_append() {
        let mut list = BrepListOfCurveRepresentation::new();
        let rep = CurveRepresentation::new(1, "3D".to_string(), 0.0, 1.0);
        list.append(rep);
        assert_eq!(list.len(), 1);
    }
}
