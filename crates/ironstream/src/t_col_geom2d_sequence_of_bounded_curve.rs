// FILE: t_col_geom2d_sequence_of_bounded_curve.rs
// occt: TColGeom2d_SequenceOfBoundedCurve

/// TColGeom2d_SequenceOfBoundedCurve: a sequence of 2D bounded curve handles.
#[derive(Debug, Clone)]
pub struct TColGeom2d_SequenceOfBoundedCurve {
    elements: Vec<u64>,
}

impl TColGeom2d_SequenceOfBoundedCurve {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn append(&mut self, value: u64) {
        self.elements.push(value);
    }

    pub fn prepend(&mut self, value: u64) {
        self.elements.insert(0, value);
    }

    pub fn length(&self) -> usize {
        self.elements.len()
    }

    pub fn first(&self) -> Option<u64> {
        self.elements.first().copied()
    }

    pub fn last(&self) -> Option<u64> {
        self.elements.last().copied()
    }

    pub fn value_at(&self, idx: usize) -> Option<u64> {
        if idx == 0 || idx > self.elements.len() {
            None
        } else {
            self.elements.get(idx - 1).copied()
        }
    }

    pub fn remove(&mut self, idx: usize) -> Option<u64> {
        if idx == 0 || idx > self.elements.len() {
            None
        } else {
            Some(self.elements.remove(idx - 1))
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl Default for TColGeom2d_SequenceOfBoundedCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_append() {
        let mut seq = TColGeom2d_SequenceOfBoundedCurve::new();
        seq.append(100);
        seq.append(200);

        assert_eq!(seq.length(), 2);
        assert_eq!(seq.first(), Some(100));
        assert_eq!(seq.last(), Some(200));
    }

    #[test]
    fn test_sequence_value_at() {
        let mut seq = TColGeom2d_SequenceOfBoundedCurve::new();
        seq.append(111);
        seq.append(222);

        assert_eq!(seq.value_at(1), Some(111));
        assert_eq!(seq.value_at(2), Some(222));
    }

    #[test]
    fn test_sequence_is_empty() {
        let mut seq = TColGeom2d_SequenceOfBoundedCurve::new();
        assert!(seq.is_empty());

        seq.append(1);
        assert!(!seq.is_empty());
    }
}
