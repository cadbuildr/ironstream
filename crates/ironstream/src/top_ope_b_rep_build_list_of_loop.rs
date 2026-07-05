// FILE: top_ope_b_rep_build_list_of_loop.rs
// occt: TopOpeBRepBuild_ListOfLoop

/// Loop: Represents a topological loop in face/shell processing.
#[derive(Clone, Debug)]
pub struct Loop {
    id: usize,
    orientation: i32,
}

impl Loop {
    pub fn new(id: usize) -> Self {
        Loop {
            id,
            orientation: 0,
        }
    }

    pub fn with_orientation(id: usize, orientation: i32) -> Self {
        Loop { id, orientation }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    pub fn set_orientation(&mut self, orientation: i32) {
        self.orientation = orientation;
    }
}

/// ListOfLoop: OCCT list container for loops (deprecated typedef).
#[derive(Clone, Debug)]
pub struct ListOfLoop {
    loops: Vec<Loop>,
}

impl ListOfLoop {
    pub fn new() -> Self {
        ListOfLoop { loops: Vec::new() }
    }

    pub fn append(&mut self, loop_item: Loop) {
        self.loops.push(loop_item);
    }

    pub fn prepend(&mut self, loop_item: Loop) {
        self.loops.insert(0, loop_item);
    }

    pub fn size(&self) -> usize {
        self.loops.len()
    }

    pub fn length(&self) -> usize {
        self.loops.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Loop> {
        self.loops.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Loop> {
        self.loops.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&Loop> {
        self.loops.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Loop> {
        self.loops.get_mut(index)
    }

    pub fn clear(&mut self) {
        self.loops.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.loops.is_empty()
    }
}

impl Default for ListOfLoop {
    fn default() -> Self {
        Self::new()
    }
}

/// ListIterator: Iterator for ListOfLoop.
pub struct ListIterator {
    loops: Vec<Loop>,
    index: usize,
}

impl ListIterator {
    pub fn new(list: &ListOfLoop) -> Self {
        ListIterator {
            loops: list.loops.clone(),
            index: 0,
        }
    }

    pub fn is_more(&self) -> bool {
        self.index < self.loops.len()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn current(&self) -> Option<&Loop> {
        self.loops.get(self.index)
    }

    pub fn value(&self) -> Option<&Loop> {
        self.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_new() {
        let loop_item = Loop::new(5);
        assert_eq!(loop_item.id(), 5);
        assert_eq!(loop_item.orientation(), 0);
    }

    #[test]
    fn test_loop_with_orientation() {
        let loop_item = Loop::with_orientation(10, 1);
        assert_eq!(loop_item.id(), 10);
        assert_eq!(loop_item.orientation(), 1);
    }

    #[test]
    fn test_list_of_loop_append() {
        let mut list = ListOfLoop::new();
        list.append(Loop::new(1));
        list.append(Loop::new(2));
        assert_eq!(list.size(), 2);
        assert_eq!(list.length(), 2);
    }

    #[test]
    fn test_list_of_loop_prepend() {
        let mut list = ListOfLoop::new();
        list.append(Loop::new(2));
        list.prepend(Loop::new(1));
        assert_eq!(list.size(), 2);
        assert_eq!(list.get(0).unwrap().id(), 1);
        assert_eq!(list.get(1).unwrap().id(), 2);
    }

    #[test]
    fn test_list_of_loop_clear() {
        let mut list = ListOfLoop::new();
        list.append(Loop::new(1));
        list.append(Loop::new(2));
        list.clear();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_of_loop_get_mut() {
        let mut list = ListOfLoop::new();
        list.append(Loop::new(99));
        if let Some(loop_item) = list.get_mut(0) {
            loop_item.set_orientation(42);
        }
        assert_eq!(list.get(0).unwrap().orientation(), 42);
    }

    #[test]
    fn test_list_iterator() {
        let mut list = ListOfLoop::new();
        list.append(Loop::with_orientation(1, 0));
        list.append(Loop::with_orientation(2, 1));

        let mut iter = ListIterator::new(&list);
        assert!(iter.is_more());
        let first = iter.current().unwrap();
        assert_eq!(first.id(), 1);
        iter.next();
        assert!(iter.is_more());
        let second = iter.current().unwrap();
        assert_eq!(second.id(), 2);
        iter.next();
        assert!(!iter.is_more());
    }

    #[test]
    fn test_list_of_loop_iter() {
        let mut list = ListOfLoop::new();
        list.append(Loop::new(10));
        list.append(Loop::new(20));

        let ids: Vec<_> = list.iter().map(|l| l.id()).collect();
        assert_eq!(ids, vec![10, 20]);
    }
}
