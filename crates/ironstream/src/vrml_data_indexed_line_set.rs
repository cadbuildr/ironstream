// FILE: vrml_data_indexed_line_set.rs
// occt: VrmlData_IndexedLineSet

#[derive(Clone, Debug)]
pub struct VrmlDataIndexedLineSet {
    line_indices: Vec<Vec<usize>>,
}

impl VrmlDataIndexedLineSet {
    pub fn new() -> Self {
        VrmlDataIndexedLineSet {
            line_indices: Vec::new(),
        }
    }

    pub fn add_line(&mut self, indices: Vec<usize>) {
        self.line_indices.push(indices);
    }

    pub fn line_count(&self) -> usize {
        self.line_indices.len()
    }

    pub fn get_line(&self, idx: usize) -> Option<&Vec<usize>> {
        self.line_indices.get(idx)
    }
}

impl Default for VrmlDataIndexedLineSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let lines = VrmlDataIndexedLineSet::new();
        assert_eq!(lines.line_count(), 0);
    }

    #[test]
    fn test_add_line() {
        let mut lines = VrmlDataIndexedLineSet::new();
        lines.add_line(vec![0, 1]);
        assert_eq!(lines.line_count(), 1);
    }
}
