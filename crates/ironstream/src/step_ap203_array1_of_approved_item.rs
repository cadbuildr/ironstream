// FILE: step_ap203_array1_of_approved_item.rs
// occt: StepAP203_Array1OfApprovedItem

/// Deprecated typedef for backward compatibility.
/// A 1-based fixed-size array of approved items.
/// Corresponds to NCollection_Array1<StepAP203_ApprovedItem>
pub struct StepAp203Array1OfApprovedItem {
    // 1-based indexing: store data starting at index 1, leave index 0 empty
    data: Vec<Option<String>>,
    lower: usize,
    upper: usize,
}

impl StepAp203Array1OfApprovedItem {
    /// Create a new array with specified bounds (1-based indexing).
    pub fn new(lower: usize, upper: usize) -> Self {
        assert!(lower > 0 && upper >= lower, "Invalid array bounds");
        let mut data = vec![None; upper + 1]; // +1 to accommodate 1-based indexing
        StepAp203Array1OfApprovedItem {
            data,
            lower,
            upper,
        }
    }

    /// Get an element by 1-based index.
    pub fn value(&self, index: usize) -> Option<&String> {
        if index >= self.lower && index <= self.upper {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    /// Set an element at a 1-based index.
    pub fn set_value(&mut self, index: usize, value: String) -> bool {
        if index >= self.lower && index <= self.upper {
            self.data[index] = Some(value);
            true
        } else {
            false
        }
    }

    /// Get the lower bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Get the upper bound.
    pub fn upper(&self) -> usize {
        self.upper
    }

    /// Get the length (upper - lower + 1).
    pub fn len(&self) -> usize {
        if self.upper >= self.lower {
            self.upper - self.lower + 1
        } else {
            0
        }
    }

    /// Check if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.upper < self.lower
    }

    /// Create an iterator over the array elements.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        (self.lower..=self.upper)
            .filter_map(move |i| self.data[i].as_ref())
    }
}

/// Iterator for the deprecated array type.
pub struct StepAp203Array1OfApprovedItemIterator {
    data: Vec<String>,
    index: usize,
}

impl StepAp203Array1OfApprovedItemIterator {
    /// Create a new iterator from an array.
    pub fn new(arr: &StepAp203Array1OfApprovedItem) -> Self {
        let mut data = Vec::new();
        for i in arr.lower..=arr.upper {
            if let Some(ref val) = arr.data[i] {
                data.push(val.clone());
            }
        }
        StepAp203Array1OfApprovedItemIterator { data, index: 0 }
    }

    /// Check if there are more elements.
    pub fn more(&self) -> bool {
        self.index < self.data.len()
    }

    /// Move to the next element.
    pub fn next(&mut self) {
        if self.more() {
            self.index += 1;
        }
    }

    /// Get the current element.
    pub fn value(&self) -> Option<&String> {
        if self.more() {
            Some(&self.data[self.index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation_and_bounds() {
        let arr = StepAp203Array1OfApprovedItem::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
        assert!(!arr.is_empty());
    }

    #[test]
    fn test_set_and_get_values() {
        let mut arr = StepAp203Array1OfApprovedItem::new(1, 3);

        assert!(arr.set_value(1, "item1".to_string()));
        assert!(arr.set_value(2, "item2".to_string()));
        assert!(arr.set_value(3, "item3".to_string()));

        assert_eq!(arr.value(1), Some(&"item1".to_string()));
        assert_eq!(arr.value(2), Some(&"item2".to_string()));
        assert_eq!(arr.value(3), Some(&"item3".to_string()));
        assert_eq!(arr.value(4), None); // Out of bounds
    }

    #[test]
    fn test_out_of_bounds_access() {
        let mut arr = StepAp203Array1OfApprovedItem::new(2, 5);

        assert!(!arr.set_value(1, "bad".to_string())); // Index 1 is before lower bound
        assert!(!arr.set_value(6, "bad".to_string())); // Index 6 is after upper bound
        assert_eq!(arr.value(1), None);
        assert_eq!(arr.value(6), None);
    }

    #[test]
    fn test_iterator() {
        let mut arr = StepAp203Array1OfApprovedItem::new(1, 3);
        arr.set_value(1, "a".to_string());
        arr.set_value(2, "b".to_string());
        arr.set_value(3, "c".to_string());

        let mut iter = StepAp203Array1OfApprovedItemIterator::new(&arr);
        let mut count = 0;

        while iter.more() {
            assert!(iter.value().is_some());
            count += 1;
            iter.next();
        }

        assert_eq!(count, 3);
    }
}
