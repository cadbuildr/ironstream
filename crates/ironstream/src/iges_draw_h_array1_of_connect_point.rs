// FILE: iges_draw_h_array1_of_connect_point.rs
// occt: IGESDraw_HArray1OfConnectPoint

/// Represents a connection point.
#[derive(Clone, Debug)]
pub struct IGESDrawConnectPoint {
    pub id: usize,
}

/// Handle-based Array1 of IGESDraw_ConnectPoint objects.
/// In OCCT, this was NCollection_HArray1<opencascade::handle<IGESDraw_ConnectPoint>>.
/// This Rust newtype wraps a Vec with 1-indexed access for faithful handle semantics.
pub struct IGESDrawHArray1OfConnectPoint {
    items: Vec<IGESDrawConnectPoint>,
    lower: usize,
}

impl IGESDrawHArray1OfConnectPoint {
    /// Creates an array with a given lower bound and size.
    pub fn new(lower: usize, size: usize) -> Self {
        IGESDrawHArray1OfConnectPoint {
            items: vec![IGESDrawConnectPoint { id: 0 }; size],
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
    pub fn value(&self, index: usize) -> Option<&IGESDrawConnectPoint> {
        if index >= self.lower && index <= self.upper() {
            self.items.get(index - self.lower)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index.
    pub fn value_mut(&mut self, index: usize) -> Option<&mut IGESDrawConnectPoint> {
        if index >= self.lower && index <= self.upper() {
            self.items.get_mut(index - self.lower)
        } else {
            None
        }
    }

    /// Sets the value at the given index.
    pub fn set_value(&mut self, index: usize, value: IGESDrawConnectPoint) -> bool {
        if index >= self.lower && index <= self.upper() {
            self.items[index - self.lower] = value;
            true
        } else {
            false
        }
    }

    /// Returns an iterator over the array.
    pub fn iter(&self) -> std::slice::Iter<IGESDrawConnectPoint> {
        self.items.iter()
    }
}

impl Default for IGESDrawHArray1OfConnectPoint {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array() {
        let arr = IGESDrawHArray1OfConnectPoint::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
    }

    #[test]
    fn test_value_access() {
        let mut arr = IGESDrawHArray1OfConnectPoint::new(1, 3);
        let elem = IGESDrawConnectPoint { id: 42 };
        arr.set_value(1, elem.clone());

        assert_eq!(arr.value(1).unwrap().id, 42);
    }

    #[test]
    fn test_value_mut() {
        let mut arr = IGESDrawHArray1OfConnectPoint::new(1, 3);
        arr.set_value(2, IGESDrawConnectPoint { id: 10 });

        if let Some(val) = arr.value_mut(2) {
            val.id = 99;
        }

        assert_eq!(arr.value(2).unwrap().id, 99);
    }

    #[test]
    fn test_iterator() {
        let mut arr = IGESDrawHArray1OfConnectPoint::new(1, 2);
        arr.set_value(1, IGESDrawConnectPoint { id: 1 });
        arr.set_value(2, IGESDrawConnectPoint { id: 2 });

        let ids: Vec<usize> = arr.iter().map(|e| e.id).collect();
        assert_eq!(ids, vec![1, 2]);
    }
}
