// FILE: iges_geom_array1_of_transformation_matrix.rs
// occt: IGESGeom_Array1OfTransformationMatrix

/// Represents a transformation matrix.
#[derive(Clone, Debug)]
pub struct IGESGeomTransformationMatrix {
    pub id: usize,
}

/// Array1 of IGESGeom_TransformationMatrix objects.
/// In OCCT, this was NCollection_Array1<opencascade::handle<IGESGeom_TransformationMatrix>>.
/// This Rust newtype wraps a Vec with 1-indexed access for faithful behavior.
pub struct IGESGeomArray1OfTransformationMatrix {
    items: Vec<IGESGeomTransformationMatrix>,
    lower: usize,
}

impl IGESGeomArray1OfTransformationMatrix {
    /// Creates an array with a given lower bound and size.
    pub fn new(lower: usize, size: usize) -> Self {
        IGESGeomArray1OfTransformationMatrix {
            items: vec![IGESGeomTransformationMatrix { id: 0 }; size],
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
    pub fn value(&self, index: usize) -> Option<&IGESGeomTransformationMatrix> {
        if index >= self.lower && index <= self.upper() {
            self.items.get(index - self.lower)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index.
    pub fn value_mut(&mut self, index: usize) -> Option<&mut IGESGeomTransformationMatrix> {
        if index >= self.lower && index <= self.upper() {
            self.items.get_mut(index - self.lower)
        } else {
            None
        }
    }

    /// Sets the value at the given index.
    pub fn set_value(&mut self, index: usize, value: IGESGeomTransformationMatrix) -> bool {
        if index >= self.lower && index <= self.upper() {
            self.items[index - self.lower] = value;
            true
        } else {
            false
        }
    }

    /// Returns an iterator over the array.
    pub fn iter(&self) -> std::slice::Iter<IGESGeomTransformationMatrix> {
        self.items.iter()
    }
}

impl Default for IGESGeomArray1OfTransformationMatrix {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array() {
        let arr = IGESGeomArray1OfTransformationMatrix::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
    }

    #[test]
    fn test_value_access() {
        let mut arr = IGESGeomArray1OfTransformationMatrix::new(1, 3);
        let elem = IGESGeomTransformationMatrix { id: 42 };
        arr.set_value(1, elem.clone());

        assert_eq!(arr.value(1).unwrap().id, 42);
    }

    #[test]
    fn test_value_mut() {
        let mut arr = IGESGeomArray1OfTransformationMatrix::new(1, 3);
        arr.set_value(2, IGESGeomTransformationMatrix { id: 10 });

        if let Some(val) = arr.value_mut(2) {
            val.id = 99;
        }

        assert_eq!(arr.value(2).unwrap().id, 99);
    }

    #[test]
    fn test_iterator() {
        let mut arr = IGESGeomArray1OfTransformationMatrix::new(1, 2);
        arr.set_value(1, IGESGeomTransformationMatrix { id: 1 });
        arr.set_value(2, IGESGeomTransformationMatrix { id: 2 });

        let ids: Vec<usize> = arr.iter().map(|e| e.id).collect();
        assert_eq!(ids, vec![1, 2]);
    }
}
