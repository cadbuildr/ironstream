// FILE: geom2d_hatch_elements.rs
// occt: Geom2dHatch_Elements

use std::collections::BTreeMap;

/// Element representing a hatch curve
#[derive(Clone, Debug)]
pub struct HatchElement {
    index: usize,
    curve_id: usize,
}

/// A collection of hatch elements
#[derive(Clone, Debug)]
pub struct Geom2dHatchElements {
    elements: BTreeMap<usize, HatchElement>,
    next_id: usize,
}

impl Geom2dHatchElements {
    pub fn new() -> Self {
        Geom2dHatchElements {
            elements: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, curve_id: usize) -> usize {
        let idx = self.next_id;
        self.elements.insert(idx, HatchElement { index: idx, curve_id });
        self.next_id += 1;
        idx
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.next_id = 1;
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn element(&self, index: usize) -> Option<&HatchElement> {
        self.elements.get(&index)
    }
}

impl Default for Geom2dHatchElements {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let elements = Geom2dHatchElements::new();
        assert_eq!(elements.size(), 0);
    }

    #[test]
    fn test_add_element() {
        let mut elements = Geom2dHatchElements::new();
        let idx = elements.add(1);
        assert_eq!(idx, 1);
        assert_eq!(elements.size(), 1);
    }

    #[test]
    fn test_clear() {
        let mut elements = Geom2dHatchElements::new();
        elements.add(1);
        elements.add(2);
        assert_eq!(elements.size(), 2);
        elements.clear();
        assert_eq!(elements.size(), 0);
    }

    #[test]
    fn test_retrieve_element() {
        let mut elements = Geom2dHatchElements::new();
        let idx = elements.add(42);
        let elem = elements.element(idx);
        assert!(elem.is_some());
        assert_eq!(elem.unwrap().curve_id, 42);
    }
}
