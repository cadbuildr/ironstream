// FILE: b_rep_blend_sequence_of_point_on_rst.rs
// occt: BRepBlend_SequenceOfPointOnRst

/// A point on a restriction curve (edge or face edge).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointOnRst {
    u: f64,                    // U parameter
    v: f64,                    // V parameter (for surface)
    param: f64,                // Curve parameter
    point_id: usize,           // Point identifier
}

impl PointOnRst {
    /// Creates a new point on restriction.
    pub fn new(u: f64, v: f64, param: f64, point_id: usize) -> Self {
        PointOnRst { u, v, param, point_id }
    }

    /// Returns the U parameter.
    pub fn u(&self) -> f64 {
        self.u
    }

    /// Returns the V parameter.
    pub fn v(&self) -> f64 {
        self.v
    }

    /// Returns the curve parameter.
    pub fn param(&self) -> f64 {
        self.param
    }

    /// Returns the point ID.
    pub fn id(&self) -> usize {
        self.point_id
    }
}

/// Deprecated type alias: Sequence of BRepBlend_PointOnRst.
pub struct BrepBlendSequenceOfPointOnRst {
    data: Vec<PointOnRst>,
}

impl BrepBlendSequenceOfPointOnRst {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        BrepBlendSequenceOfPointOnRst {
            data: Vec::new(),
        }
    }

    /// Appends a point to the sequence.
    pub fn append(&mut self, point: PointOnRst) {
        self.data.push(point);
    }

    /// Prepends a point to the sequence.
    pub fn prepend(&mut self, point: PointOnRst) {
        self.data.insert(0, point);
    }

    /// Returns the number of points.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the first point.
    pub fn first(&self) -> Option<&PointOnRst> {
        self.data.first()
    }

    /// Returns the last point.
    pub fn last(&self) -> Option<&PointOnRst> {
        self.data.last()
    }

    /// Accesses a point by 1-based index.
    pub fn get(&self, index: usize) -> Option<&PointOnRst> {
        if index < 1 || index > self.data.len() {
            return None;
        }
        self.data.get(index - 1)
    }

    /// Mutably accesses a point by 1-based index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut PointOnRst> {
        if index < 1 || index > self.data.len() {
            return None;
        }
        self.data.get_mut(index - 1)
    }

    /// Returns an iterator.
    pub fn iter(&self) -> impl Iterator<Item = &PointOnRst> {
        self.data.iter()
    }

    /// Returns a mutable iterator.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PointOnRst> {
        self.data.iter_mut()
    }

    /// Clears the sequence.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Removes the first point.
    pub fn remove_first(&mut self) -> Option<PointOnRst> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    /// Removes the last point.
    pub fn remove_last(&mut self) -> Option<PointOnRst> {
        self.data.pop()
    }

    /// Removes a point by 1-based index.
    pub fn remove(&mut self, index: usize) -> Option<PointOnRst> {
        if index < 1 || index > self.data.len() {
            return None;
        }
        Some(self.data.remove(index - 1))
    }
}

impl Default for BrepBlendSequenceOfPointOnRst {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point = PointOnRst::new(0.5, 0.5, 1.0, 1);
        assert_eq!(point.u(), 0.5);
        assert_eq!(point.v(), 0.5);
        assert_eq!(point.param(), 1.0);
        assert_eq!(point.id(), 1);
    }

    #[test]
    fn test_sequence_creation() {
        let seq = BrepBlendSequenceOfPointOnRst::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        let point = PointOnRst::new(0.0, 0.0, 0.5, 1);
        seq.append(point);
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_sequence_prepend() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        let p1 = PointOnRst::new(0.0, 0.0, 0.5, 1);
        let p2 = PointOnRst::new(1.0, 1.0, 1.5, 2);
        seq.append(p1);
        seq.prepend(p2);
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.first().unwrap().id(), 2);
    }

    #[test]
    fn test_sequence_get() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        let point = PointOnRst::new(0.25, 0.75, 1.25, 42);
        seq.append(point);
        assert_eq!(seq.get(1).unwrap().id(), 42);
        assert_eq!(seq.get(1).unwrap().u(), 0.25);
    }

    #[test]
    fn test_sequence_first_last() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        let p1 = PointOnRst::new(0.0, 0.0, 0.5, 1);
        let p2 = PointOnRst::new(1.0, 1.0, 1.5, 2);
        seq.append(p1);
        seq.append(p2);
        assert_eq!(seq.first().unwrap().id(), 1);
        assert_eq!(seq.last().unwrap().id(), 2);
    }

    #[test]
    fn test_sequence_multiple() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        for i in 1..=5 {
            let point = PointOnRst::new(i as f64 * 0.1, i as f64 * 0.1, i as f64, i);
            seq.append(point);
        }
        assert_eq!(seq.len(), 5);
        assert_eq!(seq.get(3).unwrap().id(), 3);
    }

    #[test]
    fn test_sequence_get_mut() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        let point = PointOnRst::new(0.0, 0.0, 0.5, 1);
        seq.append(point);

        if let Some(p) = seq.get_mut(1) {
            // Safely create a new point with modified values
            *p = PointOnRst::new(0.5, 0.5, 1.5, 1);
        }
        assert_eq!(seq.get(1).unwrap().u(), 0.5);
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        seq.append(PointOnRst::new(0.0, 0.0, 0.5, 1));
        seq.append(PointOnRst::new(1.0, 1.0, 1.5, 2));
        assert_eq!(seq.len(), 2);
        seq.clear();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_sequence_remove() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        seq.append(PointOnRst::new(0.0, 0.0, 0.5, 1));
        seq.append(PointOnRst::new(1.0, 1.0, 1.5, 2));
        let removed = seq.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id(), 1);
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_sequence_remove_first() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        seq.append(PointOnRst::new(0.0, 0.0, 0.5, 1));
        seq.append(PointOnRst::new(1.0, 1.0, 1.5, 2));
        let removed = seq.remove_first();
        assert_eq!(removed.unwrap().id(), 1);
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_sequence_remove_last() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        seq.append(PointOnRst::new(0.0, 0.0, 0.5, 1));
        seq.append(PointOnRst::new(1.0, 1.0, 1.5, 2));
        let removed = seq.remove_last();
        assert_eq!(removed.unwrap().id(), 2);
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_sequence_iterator() {
        let mut seq = BrepBlendSequenceOfPointOnRst::new();
        for i in 1..=3 {
            seq.append(PointOnRst::new(0.0, 0.0, i as f64, i));
        }
        let count = seq.iter().count();
        assert_eq!(count, 3);
    }
}
