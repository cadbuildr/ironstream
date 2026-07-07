// FILE: b_rep_offset_list_of_interval.rs
// occt: BRepOffset_ListOfInterval

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    first: f64,
    second: f64,
}

impl Interval {
    pub fn new(first: f64, second: f64) -> Self {
        Interval { first, second }
    }

    pub fn first(&self) -> f64 {
        self.first
    }

    pub fn second(&self) -> f64 {
        self.second
    }
}

pub struct BrepoffsetListOfInterval {
    data: VecDeque<Interval>,
}

impl BrepoffsetListOfInterval {
    pub fn new() -> Self {
        BrepoffsetListOfInterval {
            data: VecDeque::new(),
        }
    }

    pub fn append(&mut self, interval: Interval) {
        self.data.push_back(interval);
    }

    pub fn prepend(&mut self, interval: Interval) {
        self.data.push_front(interval);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Interval> {
        self.data.get(index)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Interval> {
        self.data.iter()
    }
}

impl Default for BrepoffsetListOfInterval {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_creation() {
        let interval = Interval::new(0.0, 1.0);
        assert_eq!(interval.first(), 0.0);
        assert_eq!(interval.second(), 1.0);
    }

    #[test]
    fn test_list_append() {
        let mut list = BrepoffsetListOfInterval::new();
        list.append(Interval::new(0.0, 1.0));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_get() {
        let mut list = BrepoffsetListOfInterval::new();
        let interval = Interval::new(0.5, 1.5);
        list.append(interval);
        assert_eq!(list.get(0).unwrap().first(), 0.5);
    }
}
