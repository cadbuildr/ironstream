// FILE: extrema_array1_of_p_on_surf.rs
// occt: Extrema_Array1OfPOnSurf

#[derive(Clone, Debug)]
pub struct POnSurf {}

#[derive(Clone, Debug)]
pub struct Array1OfPOnSurf {
    items: Vec<POnSurf>,
    lower: usize,
}

impl Array1OfPOnSurf {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        Array1OfPOnSurf { items: vec![POnSurf {}; size], lower }
    }

    pub fn at(&self, i: usize) -> Option<&POnSurf> {
        if i >= self.lower && i < self.lower + self.items.len() {
            Some(&self.items[i - self.lower])
        } else { None }
    }

    pub fn set(&mut self, i: usize, val: POnSurf) -> bool {
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
        let arr = Array1OfPOnSurf::new(1, 3);
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_array_access() {
        let arr = Array1OfPOnSurf::new(1, 3);
        assert!(arr.at(1).is_some());
    }
}
