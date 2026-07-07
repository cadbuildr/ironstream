// FILE: top_ope_b_rep_tool_list_of_c2_df.rs
// occt: TopOpeBRepTool_ListOfC2DF

/// C2DF: Curve and 2D-Face parameter pair.
#[derive(Clone, Debug)]
pub struct C2DF {
    curve_id: usize,
    param: f64,
    face_id: usize,
}

impl C2DF {
    pub fn new(curve_id: usize, param: f64, face_id: usize) -> Self {
        C2DF {
            curve_id,
            param,
            face_id,
        }
    }

    pub fn curve_id(&self) -> usize {
        self.curve_id
    }

    pub fn param(&self) -> f64 {
        self.param
    }

    pub fn face_id(&self) -> usize {
        self.face_id
    }

    pub fn set_param(&mut self, param: f64) {
        self.param = param;
    }
}

impl Default for C2DF {
    fn default() -> Self {
        C2DF::new(0, 0.0, 0)
    }
}

/// ListOfC2DF: List container for C2DF objects.
#[derive(Clone, Debug)]
pub struct ListOfC2DF {
    items: Vec<C2DF>,
}

impl ListOfC2DF {
    pub fn new() -> Self {
        ListOfC2DF { items: Vec::new() }
    }

    pub fn append(&mut self, item: C2DF) {
        self.items.push(item);
    }

    pub fn prepend(&mut self, item: C2DF) {
        self.items.insert(0, item);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn length(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &C2DF> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut C2DF> {
        self.items.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&C2DF> {
        self.items.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut C2DF> {
        self.items.get_mut(index)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn remove(&mut self, index: usize) -> Option<C2DF> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }
}

impl Default for ListOfC2DF {
    fn default() -> Self {
        Self::new()
    }
}

/// ListIterator: Iterator for ListOfC2DF.
pub struct ListIterator {
    items: Vec<C2DF>,
    index: usize,
}

impl ListIterator {
    pub fn new(list: &ListOfC2DF) -> Self {
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

    pub fn current(&self) -> Option<&C2DF> {
        self.items.get(self.index)
    }

    pub fn value(&self) -> Option<&C2DF> {
        self.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c2df_new() {
        let c2df = C2DF::new(10, 0.5, 20);
        assert_eq!(c2df.curve_id(), 10);
        assert_eq!(c2df.param(), 0.5);
        assert_eq!(c2df.face_id(), 20);
    }

    #[test]
    fn test_c2df_set_param() {
        let mut c2df = C2DF::new(1, 0.0, 2);
        c2df.set_param(1.5);
        assert_eq!(c2df.param(), 1.5);
    }

    #[test]
    fn test_list_append() {
        let mut list = ListOfC2DF::new();
        list.append(C2DF::new(1, 0.1, 1));
        list.append(C2DF::new(2, 0.2, 2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_list_prepend() {
        let mut list = ListOfC2DF::new();
        list.append(C2DF::new(2, 0.2, 2));
        list.prepend(C2DF::new(1, 0.1, 1));
        assert_eq!(list.get(0).unwrap().curve_id(), 1);
        assert_eq!(list.get(1).unwrap().curve_id(), 2);
    }

    #[test]
    fn test_list_remove() {
        let mut list = ListOfC2DF::new();
        list.append(C2DF::new(1, 0.1, 1));
        list.append(C2DF::new(2, 0.2, 2));
        list.append(C2DF::new(3, 0.3, 3));

        let removed = list.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().curve_id(), 2);
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_iterator() {
        let mut list = ListOfC2DF::new();
        list.append(C2DF::new(1, 0.1, 1));
        list.append(C2DF::new(2, 0.2, 2));

        let mut iter = ListIterator::new(&list);
        assert!(iter.is_more());
        assert_eq!(iter.current().unwrap().curve_id(), 1);
        iter.next();
        assert!(iter.is_more());
        assert_eq!(iter.current().unwrap().curve_id(), 2);
        iter.next();
        assert!(!iter.is_more());
    }
}
