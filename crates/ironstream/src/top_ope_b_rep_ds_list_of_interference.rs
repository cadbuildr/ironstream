// FILE: top_ope_b_rep_ds_list_of_interference.rs
// occt: TopOpeBRepDS_ListOfInterference

/// Interference: Interference data object.
#[derive(Clone, Debug)]
pub struct Interference {
    id: usize,
    param: f64,
}

impl Interference {
    pub fn new(id: usize, param: f64) -> Self {
        Interference { id, param }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn param(&self) -> f64 {
        self.param
    }
}

impl Default for Interference {
    fn default() -> Self {
        Interference::new(0, 0.0)
    }
}

/// ListOfInterference: OCCT list container.
#[derive(Clone, Debug)]
pub struct ListOfInterference {
    items: Vec<Interference>,
}

impl ListOfInterference {
    pub fn new() -> Self {
        ListOfInterference {
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: Interference) {
        self.items.push(item);
    }

    pub fn prepend(&mut self, item: Interference) {
        self.items.insert(0, item);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn length(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Interference> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Interference> {
        self.items.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&Interference> {
        self.items.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Interference> {
        self.items.get_mut(index)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn remove(&mut self, index: usize) -> Option<Interference> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }
}

impl Default for ListOfInterference {
    fn default() -> Self {
        Self::new()
    }
}

/// ListIterator: Iterator for ListOfInterference.
pub struct ListIterator {
    items: Vec<Interference>,
    index: usize,
}

impl ListIterator {
    pub fn new(list: &ListOfInterference) -> Self {
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

    pub fn current(&self) -> Option<&Interference> {
        self.items.get(self.index)
    }

    pub fn value(&self) -> Option<&Interference> {
        self.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_new() {
        let interf = Interference::new(42, 0.5);
        assert_eq!(interf.id(), 42);
        assert_eq!(interf.param(), 0.5);
    }

    #[test]
    fn test_list_append() {
        let mut list = ListOfInterference::new();
        list.append(Interference::new(1, 0.1));
        list.append(Interference::new(2, 0.2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_list_prepend() {
        let mut list = ListOfInterference::new();
        list.append(Interference::new(2, 0.2));
        list.prepend(Interference::new(1, 0.1));
        assert_eq!(list.get(0).unwrap().id(), 1);
        assert_eq!(list.get(1).unwrap().id(), 2);
    }

    #[test]
    fn test_list_remove() {
        let mut list = ListOfInterference::new();
        list.append(Interference::new(1, 0.1));
        list.append(Interference::new(2, 0.2));
        list.append(Interference::new(3, 0.3));

        let removed = list.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id(), 2);
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_list_clear() {
        let mut list = ListOfInterference::new();
        list.append(Interference::new(1, 0.1));
        list.append(Interference::new(2, 0.2));
        list.clear();
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut list = ListOfInterference::new();
        list.append(Interference::new(1, 0.1));
        list.append(Interference::new(2, 0.2));

        let mut iter = ListIterator::new(&list);
        assert!(iter.is_more());
        assert_eq!(iter.current().unwrap().id(), 1);
        iter.next();
        assert!(iter.is_more());
        assert_eq!(iter.current().unwrap().id(), 2);
        iter.next();
        assert!(!iter.is_more());
    }
}
