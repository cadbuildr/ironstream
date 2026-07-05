// FILE: top_ope_b_rep_build_list_of_list_of_loop.rs
// occt: TopOpeBRepBuild_ListOfListOfLoop

/// Loop: Simplified loop representation.
#[derive(Clone, Debug)]
pub struct Loop {
    id: usize,
}

impl Loop {
    pub fn new(id: usize) -> Self {
        Loop { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// ListOfLoop: List of loops.
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

    pub fn size(&self) -> usize {
        self.loops.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Loop> {
        self.loops.iter()
    }

    pub fn clear(&mut self) {
        self.loops.clear();
    }
}

impl Default for ListOfLoop {
    fn default() -> Self {
        Self::new()
    }
}

/// ListOfListOfLoop: List of lists of loops.
#[derive(Clone, Debug)]
pub struct ListOfListOfLoop {
    lists: Vec<ListOfLoop>,
}

impl ListOfListOfLoop {
    pub fn new() -> Self {
        ListOfListOfLoop {
            lists: Vec::new(),
        }
    }

    pub fn append(&mut self, list: ListOfLoop) {
        self.lists.push(list);
    }

    pub fn size(&self) -> usize {
        self.lists.len()
    }

    pub fn length(&self) -> usize {
        self.lists.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ListOfLoop> {
        self.lists.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ListOfLoop> {
        self.lists.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&ListOfLoop> {
        self.lists.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut ListOfLoop> {
        self.lists.get_mut(index)
    }

    pub fn clear(&mut self) {
        self.lists.clear();
    }
}

impl Default for ListOfListOfLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_new() {
        let loop_item = Loop::new(42);
        assert_eq!(loop_item.id(), 42);
    }

    #[test]
    fn test_list_of_loop() {
        let mut list = ListOfLoop::new();
        list.append(Loop::new(1));
        list.append(Loop::new(2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_list_of_list_of_loop() {
        let mut outer = ListOfListOfLoop::new();
        let mut inner1 = ListOfLoop::new();
        inner1.append(Loop::new(10));
        inner1.append(Loop::new(11));
        outer.append(inner1);

        let mut inner2 = ListOfLoop::new();
        inner2.append(Loop::new(20));
        outer.append(inner2);

        assert_eq!(outer.size(), 2);
        assert_eq!(outer.get(0).unwrap().size(), 2);
        assert_eq!(outer.get(1).unwrap().size(), 1);
    }

    #[test]
    fn test_list_of_list_of_loop_clear() {
        let mut outer = ListOfListOfLoop::new();
        outer.append(ListOfLoop::new());
        outer.append(ListOfLoop::new());
        outer.clear();
        assert_eq!(outer.size(), 0);
    }

    #[test]
    fn test_list_of_list_of_loop_iter() {
        let mut outer = ListOfListOfLoop::new();
        let inner1 = ListOfLoop::new();
        outer.append(inner1);
        outer.append(ListOfLoop::new());

        let count = outer.iter().count();
        assert_eq!(count, 2);
    }
}
