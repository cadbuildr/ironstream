// FILE: toptools_list.rs
// occt-ref: TopTools_ListOfShape, TopTools_MapOfShape, TopTools_IndexedMapOfShape

/// List of shapes.
// occt-ref: TopTools_ListOfShape
#[derive(Clone, Debug)]
pub struct TopToolsListOfShape {
    pub shapes: Vec<u32>,
}

impl TopToolsListOfShape {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }

    pub fn append(&mut self, shape_id: u32) {
        self.shapes.push(shape_id);
    }

    pub fn prepend(&mut self, shape_id: u32) {
        self.shapes.insert(0, shape_id);
    }

    pub fn remove_first(&mut self) -> Option<u32> {
        if !self.shapes.is_empty() {
            Some(self.shapes.remove(0))
        } else {
            None
        }
    }

    pub fn first(&self) -> Option<u32> {
        self.shapes.first().copied()
    }

    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

impl Default for TopToolsListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Map of shapes (no duplicates via linear search).
// occt-ref: TopTools_MapOfShape
#[derive(Clone, Debug)]
pub struct TopToolsMapOfShape {
    pub shapes: Vec<u32>,
}

impl TopToolsMapOfShape {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }

    pub fn add(&mut self, shape_id: u32) -> bool {
        if !self.shapes.contains(&shape_id) {
            self.shapes.push(shape_id);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, shape_id: u32) -> bool {
        self.shapes.contains(&shape_id)
    }

    pub fn remove(&mut self, shape_id: u32) -> bool {
        let before = self.shapes.len();
        self.shapes.retain(|&s| s != shape_id);
        self.shapes.len() < before
    }

    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

impl Default for TopToolsMapOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Indexed map of shapes (1-indexed like OCCT).
// occt-ref: TopTools_IndexedMapOfShape
#[derive(Clone, Debug)]
pub struct TopToolsIndexedMapOfShape {
    pub shapes: Vec<u32>,
}

impl TopToolsIndexedMapOfShape {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }

    pub fn add(&mut self, shape_id: u32) -> usize {
        if let Some(pos) = self.shapes.iter().position(|&s| s == shape_id) {
            pos + 1  // 1-indexed
        } else {
            self.shapes.push(shape_id);
            self.shapes.len()  // 1-indexed
        }
    }

    pub fn find_index(&self, shape_id: u32) -> Option<usize> {
        self.shapes
            .iter()
            .position(|&s| s == shape_id)
            .map(|pos| pos + 1)  // 1-indexed
    }

    pub fn value(&self, index: usize) -> Option<u32> {
        if index > 0 && index <= self.shapes.len() {
            Some(self.shapes[index - 1])
        } else {
            None
        }
    }

    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

impl Default for TopToolsIndexedMapOfShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toptools_list_of_shape() {
        let mut list = TopToolsListOfShape::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.nb_shapes(), 3);
        assert_eq!(list.first(), Some(1));
    }

    #[test]
    fn toptools_list_prepend() {
        let mut list = TopToolsListOfShape::new();
        list.append(2);
        list.prepend(1);
        assert_eq!(list.first(), Some(1));
    }

    #[test]
    fn toptools_list_remove() {
        let mut list = TopToolsListOfShape::new();
        list.append(1);
        list.append(2);
        assert_eq!(list.remove_first(), Some(1));
        assert_eq!(list.nb_shapes(), 1);
    }

    #[test]
    fn toptools_map_of_shape() {
        let mut map = TopToolsMapOfShape::new();
        assert!(map.add(1));
        assert!(!map.add(1));  // duplicate
        assert_eq!(map.nb_shapes(), 1);
        assert!(map.contains(1));
    }

    #[test]
    fn toptools_map_remove() {
        let mut map = TopToolsMapOfShape::new();
        map.add(1);
        map.add(2);
        assert!(map.remove(1));
        assert_eq!(map.nb_shapes(), 1);
        assert!(!map.contains(1));
    }

    #[test]
    fn toptools_indexed_map() {
        let mut imap = TopToolsIndexedMapOfShape::new();
        let idx1 = imap.add(10);
        let idx2 = imap.add(20);
        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
        assert_eq!(imap.value(1), Some(10));
        assert_eq!(imap.find_index(20), Some(2));
    }

    #[test]
    fn toptools_indexed_map_duplicate() {
        let mut imap = TopToolsIndexedMapOfShape::new();
        let idx1 = imap.add(5);
        let idx2 = imap.add(5);
        assert_eq!(idx1, idx2);  // same shape returns same index
        assert_eq!(imap.nb_shapes(), 1);
    }
}
