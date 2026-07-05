// FILE: b_rep_list_of_point_representation.rs
// occt: BRep_ListOfPointRepresentation

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct PointRepresentation {
    point_id: usize,
    representation_type: String,
    coordinates: (f64, f64, f64),
}

impl PointRepresentation {
    pub fn new(point_id: usize, representation_type: String, x: f64, y: f64, z: f64) -> Self {
        PointRepresentation {
            point_id,
            representation_type,
            coordinates: (x, y, z),
        }
    }

    pub fn point_id(&self) -> usize {
        self.point_id
    }

    pub fn representation_type(&self) -> &str {
        &self.representation_type
    }

    pub fn coordinates(&self) -> (f64, f64, f64) {
        self.coordinates
    }
}

pub struct BrepListOfPointRepresentation {
    data: VecDeque<PointRepresentation>,
}

impl BrepListOfPointRepresentation {
    pub fn new() -> Self {
        BrepListOfPointRepresentation {
            data: VecDeque::new(),
        }
    }

    pub fn append(&mut self, representation: PointRepresentation) {
        self.data.push_back(representation);
    }

    pub fn prepend(&mut self, representation: PointRepresentation) {
        self.data.push_front(representation);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&PointRepresentation> {
        self.data.get(index)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &PointRepresentation> {
        self.data.iter()
    }
}

impl Default for BrepListOfPointRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_representation_creation() {
        let rep = PointRepresentation::new(1, "3D".to_string(), 0.5, 1.5, 2.5);
        assert_eq!(rep.point_id(), 1);
        assert_eq!(rep.coordinates(), (0.5, 1.5, 2.5));
    }

    #[test]
    fn test_list_append() {
        let mut list = BrepListOfPointRepresentation::new();
        let rep = PointRepresentation::new(1, "3D".to_string(), 0.0, 0.0, 0.0);
        list.append(rep);
        assert_eq!(list.len(), 1);
    }
}
