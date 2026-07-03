// FILE: n_collection_linear_vector.rs
// occt: NCollection_LinearVector

/// Linear (1-indexed) vector.
pub struct LinearVector<T: Clone> {
    data: Vec<T>,
    lower: usize,
}

impl<T: Clone + Default> LinearVector<T> {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper - lower + 1;
        LinearVector {
            data: vec![T::default(); size],
            lower,
        }
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> usize {
        self.lower + self.data.len() - 1
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.lower && idx <= self.upper() {
            Some(&self.data[idx - self.lower])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx >= self.lower && idx <= self.upper() {
            Some(&mut self.data[idx - self.lower])
        } else {
            None
        }
    }

    pub fn set(&mut self, idx: usize, val: T) {
        if let Some(entry) = self.get_mut(idx) {
            *entry = val;
        }
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_vector() {
        let mut vec = LinearVector::new(1, 3);
        assert_eq!(vec.lower(), 1);
        assert_eq!(vec.upper(), 3);
        vec.set(1, 10);
        assert_eq!(vec.get(1), Some(&10));
    }
}
