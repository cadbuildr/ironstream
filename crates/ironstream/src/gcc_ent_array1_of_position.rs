// FILE: gcc_ent_array1_of_position.rs
// occt: GccEnt_Array1OfPosition

#[derive(Clone, Debug)]
pub struct Position {}

#[derive(Clone, Debug)]
pub struct Array1OfPosition {
    items: Vec<Position>,
    lower: usize,
}

impl Array1OfPosition {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        Array1OfPosition {
            items: vec![Position {}; size],
            lower,
        }
    }

    pub fn at(&self, i: usize) -> Option<&Position> {
        if i >= self.lower && i < self.lower + self.items.len() {
            Some(&self.items[i - self.lower])
        } else { None }
    }

    pub fn set(&mut self, i: usize, val: Position) -> bool {
        if i >= self.lower && i < self.lower + self.items.len() {
            self.items[i - self.lower] = val;
            true
        } else { false }
    }

    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.lower + self.items.len() - 1 }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let arr = Array1OfPosition::new(1, 3);
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_array_access() {
        let arr = Array1OfPosition::new(1, 3);
        assert!(arr.at(1).is_some());
    }
}
