// FILE: top_bas_list_of_test_interference.rs
// occt: TopBas_ListOfTestInterference, TopBas_TestInterference, TopBas_ListIteratorOfListOfTestInterference

/// TopBas_TestInterference: Holds interference data for hidden line removal.
#[derive(Clone, Debug)]
pub struct TestInterference {
    intersection: f64,
    boundary: i32,
    orientation: TopAbsOrientation,
    transition: TopAbsOrientation,
    b_transition: TopAbsOrientation,
}

/// TopAbsOrientation: Simplified enum for orientation (avoiding external deps).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsOrientation {
    Forward,
    Reversed,
    Internal,
    External,
}

impl TestInterference {
    /// Creates a new empty TestInterference.
    pub fn new() -> Self {
        TestInterference {
            intersection: 0.0,
            boundary: 0,
            orientation: TopAbsOrientation::Forward,
            transition: TopAbsOrientation::Forward,
            b_transition: TopAbsOrientation::Forward,
        }
    }

    /// Creates a TestInterference with all parameters.
    pub fn with_values(
        intersection: f64,
        boundary: i32,
        orientation: TopAbsOrientation,
        transition: TopAbsOrientation,
        b_transition: TopAbsOrientation,
    ) -> Self {
        TestInterference {
            intersection,
            boundary,
            orientation,
            transition,
            b_transition,
        }
    }

    pub fn set_intersection(&mut self, value: f64) {
        self.intersection = value;
    }

    pub fn set_boundary(&mut self, value: i32) {
        self.boundary = value;
    }

    pub fn set_orientation(&mut self, value: TopAbsOrientation) {
        self.orientation = value;
    }

    pub fn set_transition(&mut self, value: TopAbsOrientation) {
        self.transition = value;
    }

    pub fn set_boundary_transition(&mut self, value: TopAbsOrientation) {
        self.b_transition = value;
    }

    pub fn intersection(&self) -> f64 {
        self.intersection
    }

    pub fn intersection_mut(&mut self) -> &mut f64 {
        &mut self.intersection
    }

    pub fn boundary(&self) -> i32 {
        self.boundary
    }

    pub fn boundary_mut(&mut self) -> &mut i32 {
        &mut self.boundary
    }

    pub fn orientation(&self) -> TopAbsOrientation {
        self.orientation
    }

    pub fn transition(&self) -> TopAbsOrientation {
        self.transition
    }

    pub fn boundary_transition(&self) -> TopAbsOrientation {
        self.b_transition
    }
}

impl Default for TestInterference {
    fn default() -> Self {
        Self::new()
    }
}

/// TopBas_ListOfTestInterference: List container (OCCT deprecated typedef).
#[derive(Clone, Debug)]
pub struct ListOfTestInterference {
    data: Vec<TestInterference>,
}

impl ListOfTestInterference {
    /// Creates a new empty list.
    pub fn new() -> Self {
        ListOfTestInterference { data: Vec::new() }
    }

    /// Appends a value to the list.
    pub fn append(&mut self, value: TestInterference) {
        self.data.push(value);
    }

    /// Prepends a value to the list.
    pub fn prepend(&mut self, value: TestInterference) {
        self.data.insert(0, value);
    }

    /// Returns the size of the list.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Returns the size of the list (OCCT alias).
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Clears the list.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns an iterator over references.
    pub fn iter(&self) -> impl Iterator<Item = &TestInterference> {
        self.data.iter()
    }

    /// Returns a mutable iterator.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TestInterference> {
        self.data.iter_mut()
    }

    /// Gets a reference to an element by 0-based index.
    pub fn get(&self, index: usize) -> Option<&TestInterference> {
        self.data.get(index)
    }

    /// Gets a mutable reference to an element by 0-based index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut TestInterference> {
        self.data.get_mut(index)
    }
}

impl Default for ListOfTestInterference {
    fn default() -> Self {
        Self::new()
    }
}

/// TopBas_ListIteratorOfListOfTestInterference: Iterator for the list (alias).
pub struct ListIterator {
    index: usize,
    data: Vec<TestInterference>,
}

impl ListIterator {
    /// Creates a new iterator over the list.
    pub fn new(list: &ListOfTestInterference) -> Self {
        ListIterator {
            index: 0,
            data: list.data.clone(),
        }
    }

    /// Returns true if there is a next element.
    pub fn is_more(&self) -> bool {
        self.index < self.data.len()
    }

    /// Advances to the next element.
    pub fn next(&mut self) {
        self.index += 1;
    }

    /// Returns the current element.
    pub fn current(&self) -> Option<&TestInterference> {
        self.data.get(self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_new() {
        let interf = TestInterference::new();
        assert_eq!(interf.intersection(), 0.0);
        assert_eq!(interf.boundary(), 0);
        assert_eq!(interf.orientation(), TopAbsOrientation::Forward);
    }

    #[test]
    fn test_interference_with_values() {
        let interf = TestInterference::with_values(
            1.5,
            42,
            TopAbsOrientation::Reversed,
            TopAbsOrientation::Internal,
            TopAbsOrientation::External,
        );
        assert_eq!(interf.intersection(), 1.5);
        assert_eq!(interf.boundary(), 42);
        assert_eq!(interf.orientation(), TopAbsOrientation::Reversed);
        assert_eq!(interf.transition(), TopAbsOrientation::Internal);
        assert_eq!(interf.boundary_transition(), TopAbsOrientation::External);
    }

    #[test]
    fn test_interference_setters() {
        let mut interf = TestInterference::new();
        interf.set_intersection(3.14);
        interf.set_boundary(10);
        interf.set_orientation(TopAbsOrientation::Reversed);
        assert_eq!(interf.intersection(), 3.14);
        assert_eq!(interf.boundary(), 10);
        assert_eq!(interf.orientation(), TopAbsOrientation::Reversed);
    }

    #[test]
    fn test_list_append() {
        let mut list = ListOfTestInterference::new();
        list.append(TestInterference::with_values(
            1.0,
            1,
            TopAbsOrientation::Forward,
            TopAbsOrientation::Forward,
            TopAbsOrientation::Forward,
        ));
        list.append(TestInterference::with_values(
            2.0,
            2,
            TopAbsOrientation::Reversed,
            TopAbsOrientation::Reversed,
            TopAbsOrientation::Reversed,
        ));
        assert_eq!(list.size(), 2);
        assert_eq!(list.length(), 2);
    }

    #[test]
    fn test_list_prepend() {
        let mut list = ListOfTestInterference::new();
        list.append(TestInterference::new());
        list.prepend(TestInterference::new());
        assert_eq!(list.size(), 2);
        assert_eq!(list.get(0).unwrap().intersection(), 0.0);
    }

    #[test]
    fn test_list_clear() {
        let mut list = ListOfTestInterference::new();
        list.append(TestInterference::new());
        list.append(TestInterference::new());
        list.clear();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_list_iterator() {
        let mut list = ListOfTestInterference::new();
        list.append(TestInterference::with_values(
            1.0,
            1,
            TopAbsOrientation::Forward,
            TopAbsOrientation::Forward,
            TopAbsOrientation::Forward,
        ));
        list.append(TestInterference::with_values(
            2.0,
            2,
            TopAbsOrientation::Reversed,
            TopAbsOrientation::Reversed,
            TopAbsOrientation::Reversed,
        ));

        let mut iter = ListIterator::new(&list);
        assert!(iter.is_more());
        let first = iter.current().unwrap();
        assert_eq!(first.intersection(), 1.0);
        iter.next();
        assert!(iter.is_more());
        let second = iter.current().unwrap();
        assert_eq!(second.intersection(), 2.0);
        iter.next();
        assert!(!iter.is_more());
    }
}
