// FILE: iges_dimen_h_array1_of_general_note.rs
// occt: IGESDimen_HArray1OfGeneralNote

/// Represents a general note for dimensions.
#[derive(Clone, Debug)]
pub struct IGESDimenGeneralNote {
    pub id: usize,
}

/// Handle-based Array1 of IGESDimen_GeneralNote objects.
/// In OCCT, this was NCollection_HArray1<opencascade::handle<IGESDimen_GeneralNote>>.
/// This Rust newtype wraps a Vec with 1-indexed access for faithful handle semantics.
pub struct IGESDimenHArray1OfGeneralNote {
    items: Vec<IGESDimenGeneralNote>,
    lower: usize,
}

impl IGESDimenHArray1OfGeneralNote {
    /// Creates an array with a given lower bound and size.
    pub fn new(lower: usize, size: usize) -> Self {
        IGESDimenHArray1OfGeneralNote {
            items: vec![IGESDimenGeneralNote { id: 0 }; size],
            lower,
        }
    }

    /// Returns the lower bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the upper bound.
    pub fn upper(&self) -> usize {
        self.lower + self.items.len() - 1
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns a reference to the element at the given index.
    pub fn value(&self, index: usize) -> Option<&IGESDimenGeneralNote> {
        if index >= self.lower && index <= self.upper() {
            self.items.get(index - self.lower)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index.
    pub fn value_mut(&mut self, index: usize) -> Option<&mut IGESDimenGeneralNote> {
        if index >= self.lower && index <= self.upper() {
            self.items.get_mut(index - self.lower)
        } else {
            None
        }
    }

    /// Sets the value at the given index.
    pub fn set_value(&mut self, index: usize, value: IGESDimenGeneralNote) -> bool {
        if index >= self.lower && index <= self.upper() {
            self.items[index - self.lower] = value;
            true
        } else {
            false
        }
    }

    /// Returns an iterator over the array.
    pub fn iter(&self) -> std::slice::Iter<IGESDimenGeneralNote> {
        self.items.iter()
    }
}

impl Default for IGESDimenHArray1OfGeneralNote {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array() {
        let arr = IGESDimenHArray1OfGeneralNote::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
    }

    #[test]
    fn test_value_access() {
        let mut arr = IGESDimenHArray1OfGeneralNote::new(1, 3);
        let elem = IGESDimenGeneralNote { id: 42 };
        arr.set_value(1, elem.clone());

        assert_eq!(arr.value(1).unwrap().id, 42);
    }

    #[test]
    fn test_value_mut() {
        let mut arr = IGESDimenHArray1OfGeneralNote::new(1, 3);
        arr.set_value(2, IGESDimenGeneralNote { id: 10 });

        if let Some(val) = arr.value_mut(2) {
            val.id = 99;
        }

        assert_eq!(arr.value(2).unwrap().id, 99);
    }

    #[test]
    fn test_iterator() {
        let mut arr = IGESDimenHArray1OfGeneralNote::new(1, 2);
        arr.set_value(1, IGESDimenGeneralNote { id: 1 });
        arr.set_value(2, IGESDimenGeneralNote { id: 2 });

        let ids: Vec<usize> = arr.iter().map(|e| e.id).collect();
        assert_eq!(ids, vec![1, 2]);
    }
}
