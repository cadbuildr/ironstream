// FILE: mesh_vs_array1_of_sequence_of_integer.rs
// occt: MeshVS_Array1OfSequenceOfInteger

pub struct MeshVSArray1OfSequenceOfInteger {
    items: Vec<Vec<i32>>,
    lower: usize,
}

impl MeshVSArray1OfSequenceOfInteger {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        Self {
            items: vec![Vec::new(); size],
            lower,
        }
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> usize {
        if self.items.is_empty() {
            self.lower - 1
        } else {
            self.lower + self.items.len() - 1
        }
    }

    pub fn set_value(&mut self, index: usize, value: Vec<i32>) {
        if index >= self.lower && index <= self.upper() {
            let idx = index - self.lower;
            if idx < self.items.len() {
                self.items[idx] = value;
            }
        }
    }

    pub fn value_at(&self, index: usize) -> Option<Vec<i32>> {
        if index >= self.lower && index <= self.upper() {
            let idx = index - self.lower;
            if idx < self.items.len() {
                return Some(self.items[idx].clone());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let mut arr = MeshVSArray1OfSequenceOfInteger::new(1, 3);
        arr.set_value(1, vec![1, 2, 3]);
        assert_eq!(arr.value_at(1), Some(vec![1, 2, 3]));
    }
}
