// FILE: top_ope_b_rep_build_list_of_pave.rs
// occt: TopOpeBRepBuild_ListOfPave

/// Pave: Represents a point on an edge (used in intersection/split algorithms).
#[derive(Clone, Debug)]
pub struct Pave {
    parameter: f64,
    vertex_id: usize,
    orientation: i32,
}

impl Pave {
    pub fn new() -> Self {
        Pave {
            parameter: 0.0,
            vertex_id: 0,
            orientation: 0,
        }
    }

    pub fn with_values(parameter: f64, vertex_id: usize, orientation: i32) -> Self {
        Pave {
            parameter,
            vertex_id,
            orientation,
        }
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    pub fn set_parameter(&mut self, parameter: f64) {
        self.parameter = parameter;
    }

    pub fn vertex_id(&self) -> usize {
        self.vertex_id
    }

    pub fn set_vertex_id(&mut self, vertex_id: usize) {
        self.vertex_id = vertex_id;
    }

    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    pub fn set_orientation(&mut self, orientation: i32) {
        self.orientation = orientation;
    }
}

impl Default for Pave {
    fn default() -> Self {
        Self::new()
    }
}

/// ListOfPave: OCCT list container for paves (deprecated typedef).
#[derive(Clone, Debug)]
pub struct ListOfPave {
    paves: Vec<Pave>,
}

impl ListOfPave {
    pub fn new() -> Self {
        ListOfPave {
            paves: Vec::new(),
        }
    }

    pub fn append(&mut self, pave: Pave) {
        self.paves.push(pave);
    }

    pub fn prepend(&mut self, pave: Pave) {
        self.paves.insert(0, pave);
    }

    pub fn size(&self) -> usize {
        self.paves.len()
    }

    pub fn length(&self) -> usize {
        self.paves.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pave> {
        self.paves.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Pave> {
        self.paves.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&Pave> {
        self.paves.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Pave> {
        self.paves.get_mut(index)
    }

    pub fn clear(&mut self) {
        self.paves.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.paves.is_empty()
    }

    /// Remove element at given index
    pub fn remove(&mut self, index: usize) -> Option<Pave> {
        if index < self.paves.len() {
            Some(self.paves.remove(index))
        } else {
            None
        }
    }
}

impl Default for ListOfPave {
    fn default() -> Self {
        Self::new()
    }
}

/// ListIterator: Iterator for ListOfPave.
pub struct ListIterator {
    paves: Vec<Pave>,
    index: usize,
}

impl ListIterator {
    pub fn new(list: &ListOfPave) -> Self {
        ListIterator {
            paves: list.paves.clone(),
            index: 0,
        }
    }

    pub fn is_more(&self) -> bool {
        self.index < self.paves.len()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn current(&self) -> Option<&Pave> {
        self.paves.get(self.index)
    }

    pub fn value(&self) -> Option<&Pave> {
        self.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pave_new() {
        let pave = Pave::new();
        assert_eq!(pave.parameter(), 0.0);
        assert_eq!(pave.vertex_id(), 0);
        assert_eq!(pave.orientation(), 0);
    }

    #[test]
    fn test_pave_with_values() {
        let pave = Pave::with_values(0.5, 42, 1);
        assert_eq!(pave.parameter(), 0.5);
        assert_eq!(pave.vertex_id(), 42);
        assert_eq!(pave.orientation(), 1);
    }

    #[test]
    fn test_pave_setters() {
        let mut pave = Pave::new();
        pave.set_parameter(1.5);
        pave.set_vertex_id(10);
        pave.set_orientation(2);
        assert_eq!(pave.parameter(), 1.5);
        assert_eq!(pave.vertex_id(), 10);
        assert_eq!(pave.orientation(), 2);
    }

    #[test]
    fn test_list_of_pave_append() {
        let mut list = ListOfPave::new();
        list.append(Pave::with_values(0.1, 1, 0));
        list.append(Pave::with_values(0.5, 2, 1));
        list.append(Pave::with_values(0.9, 3, 0));
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn test_list_of_pave_prepend() {
        let mut list = ListOfPave::new();
        list.append(Pave::with_values(0.5, 2, 0));
        list.prepend(Pave::with_values(0.0, 1, 0));
        assert_eq!(list.get(0).unwrap().vertex_id(), 1);
        assert_eq!(list.get(1).unwrap().vertex_id(), 2);
    }

    #[test]
    fn test_list_of_pave_clear() {
        let mut list = ListOfPave::new();
        list.append(Pave::new());
        list.append(Pave::new());
        list.clear();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_of_pave_remove() {
        let mut list = ListOfPave::new();
        list.append(Pave::with_values(0.0, 1, 0));
        list.append(Pave::with_values(0.5, 2, 0));
        list.append(Pave::with_values(1.0, 3, 0));

        let removed = list.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().vertex_id(), 2);
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_list_iterator() {
        let mut list = ListOfPave::new();
        list.append(Pave::with_values(0.0, 1, 0));
        list.append(Pave::with_values(0.5, 2, 1));

        let mut iter = ListIterator::new(&list);
        assert!(iter.is_more());
        let first = iter.current().unwrap();
        assert_eq!(first.vertex_id(), 1);
        iter.next();
        assert!(iter.is_more());
        let second = iter.current().unwrap();
        assert_eq!(second.vertex_id(), 2);
        iter.next();
        assert!(!iter.is_more());
    }
}
