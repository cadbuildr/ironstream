// FILE: top_ope_b_rep_list_of_bipoint.rs
// occt: TopOpeBRep_ListOfBipoint, TopOpeBRep_Bipoint

/// Bipoint: Pair of point indices.
#[derive(Clone, Debug)]
pub struct Bipoint {
    point1: usize,
    point2: usize,
}

impl Bipoint {
    pub fn new(point1: usize, point2: usize) -> Self {
        Bipoint { point1, point2 }
    }

    pub fn point1(&self) -> usize {
        self.point1
    }

    pub fn point2(&self) -> usize {
        self.point2
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.point1, &mut self.point2);
    }
}

/// ListOfBipoint: OCCT list container.
#[derive(Clone, Debug)]
pub struct ListOfBipoint {
    items: Vec<Bipoint>,
}

impl ListOfBipoint {
    pub fn new() -> Self {
        ListOfBipoint { items: Vec::new() }
    }

    pub fn append(&mut self, item: Bipoint) {
        self.items.push(item);
    }

    pub fn prepend(&mut self, item: Bipoint) {
        self.items.insert(0, item);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Bipoint> {
        self.items.iter()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for ListOfBipoint {
    fn default() -> Self {
        Self::new()
    }
}

/// ListIterator: Iterator for ListOfBipoint.
pub struct ListIterator {
    items: Vec<Bipoint>,
    index: usize,
}

impl ListIterator {
    pub fn new(list: &ListOfBipoint) -> Self {
        ListIterator {
            items: list.items.clone(),
            index: 0,
        }
    }

    pub fn is_more(&self) -> bool {
        self.index < self.items.len()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn current(&self) -> Option<&Bipoint> {
        self.items.get(self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bipoint_new() {
        let bp = Bipoint::new(1, 2);
        assert_eq!(bp.point1(), 1);
        assert_eq!(bp.point2(), 2);
    }

    #[test]
    fn test_bipoint_swap() {
        let mut bp = Bipoint::new(1, 2);
        bp.swap();
        assert_eq!(bp.point1(), 2);
        assert_eq!(bp.point2(), 1);
    }

    #[test]
    fn test_list_append() {
        let mut list = ListOfBipoint::new();
        list.append(Bipoint::new(1, 2));
        list.append(Bipoint::new(3, 4));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_list_iterator() {
        let mut list = ListOfBipoint::new();
        list.append(Bipoint::new(1, 2));
        list.append(Bipoint::new(3, 4));

        let mut iter = ListIterator::new(&list);
        assert!(iter.is_more());
        assert_eq!(iter.current().unwrap().point1(), 1);
        iter.next();
        assert!(iter.is_more());
        assert_eq!(iter.current().unwrap().point1(), 3);
    }
}
