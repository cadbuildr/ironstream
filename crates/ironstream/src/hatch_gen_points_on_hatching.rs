// FILE: hatch_gen_points_on_hatching.rs
// occt: HatchGen_PointsOnHatching

/// Deprecated type alias for a sequence of HatchGen_PointOnHatching.
/// In OCCT, this was NCollection_Sequence<HatchGen_PointOnHatching>.
/// This Rust newtype wraps a Vec for faithful behavior.
pub struct HatchGenPointsOnHatching {
    items: Vec<HatchGenPointOnHatching>,
}

/// Represents a point on a hatching line.
#[derive(Clone, Debug, PartialEq)]
pub struct HatchGenPointOnHatching {
    pub x: f64,
    pub y: f64,
    pub parameter: f64,
}

impl HatchGenPointsOnHatching {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        HatchGenPointsOnHatching { items: Vec::new() }
    }

    /// Returns the length of the sequence.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Appends an element to the end of the sequence.
    pub fn append(&mut self, value: HatchGenPointOnHatching) {
        self.items.push(value);
    }

    /// Returns a reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value(&self, index: usize) -> Option<&HatchGenPointOnHatching> {
        if index > 0 && index <= self.items.len() {
            self.items.get(index - 1)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value_mut(&mut self, index: usize) -> Option<&mut HatchGenPointOnHatching> {
        if index > 0 && index <= self.items.len() {
            self.items.get_mut(index - 1)
        } else {
            None
        }
    }

    /// Clears all elements from the sequence.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns an iterator over the sequence.
    pub fn iter(&self) -> std::slice::Iter<HatchGenPointOnHatching> {
        self.items.iter()
    }
}

impl Default for HatchGenPointsOnHatching {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = HatchGenPointsOnHatching::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_and_len() {
        let mut seq = HatchGenPointsOnHatching::new();
        let pt1 = HatchGenPointOnHatching { x: 1.0, y: 2.0, parameter: 0.5 };
        seq.append(pt1.clone());
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_value_1indexed() {
        let mut seq = HatchGenPointsOnHatching::new();
        let pt = HatchGenPointOnHatching { x: 3.0, y: 4.0, parameter: 0.75 };
        seq.append(pt.clone());

        // 1-indexed access
        assert_eq!(seq.value(1), Some(&pt));
        // 0-indexed should return None
        assert_eq!(seq.value(0), None);
        // Out of bounds
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn test_value_mut() {
        let mut seq = HatchGenPointsOnHatching::new();
        let pt = HatchGenPointOnHatching { x: 5.0, y: 6.0, parameter: 0.1 };
        seq.append(pt);

        if let Some(val) = seq.value_mut(1) {
            val.x = 10.0;
        }

        let retrieved = seq.value(1).unwrap();
        assert_eq!(retrieved.x, 10.0);
        assert_eq!(retrieved.y, 6.0);
    }

    #[test]
    fn test_clear() {
        let mut seq = HatchGenPointsOnHatching::new();
        seq.append(HatchGenPointOnHatching { x: 1.0, y: 2.0, parameter: 0.5 });
        seq.append(HatchGenPointOnHatching { x: 3.0, y: 4.0, parameter: 0.75 });

        assert_eq!(seq.len(), 2);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = HatchGenPointsOnHatching::new();
        let pt1 = HatchGenPointOnHatching { x: 1.0, y: 2.0, parameter: 0.1 };
        let pt2 = HatchGenPointOnHatching { x: 3.0, y: 4.0, parameter: 0.2 };
        seq.append(pt1.clone());
        seq.append(pt2.clone());

        let mut iter = seq.iter();
        assert_eq!(iter.next(), Some(&pt1));
        assert_eq!(iter.next(), Some(&pt2));
        assert_eq!(iter.next(), None);
    }
}
