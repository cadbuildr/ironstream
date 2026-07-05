// FILE: iges_geom_h_array1_of_curve_on_surface.rs
// occt: IGESGeom_HArray1OfCurveOnSurface

/// Represents a curve on surface.
#[derive(Clone, Debug)]
pub struct IGESGeomCurveOnSurface {
    pub id: usize,
}

/// Handle-based Array1 of IGESGeom_CurveOnSurface objects.
/// In OCCT, this was NCollection_HArray1<opencascade::handle<IGESGeom_CurveOnSurface>>.
/// This Rust newtype wraps a Vec with 1-indexed access for faithful handle semantics.
pub struct IGESGeomHArray1OfCurveOnSurface {
    items: Vec<IGESGeomCurveOnSurface>,
    lower: usize,
}

impl IGESGeomHArray1OfCurveOnSurface {
    /// Creates an array with a given lower bound and size.
    pub fn new(lower: usize, size: usize) -> Self {
        IGESGeomHArray1OfCurveOnSurface {
            items: vec![IGESGeomCurveOnSurface { id: 0 }; size],
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
    pub fn value(&self, index: usize) -> Option<&IGESGeomCurveOnSurface> {
        if index >= self.lower && index <= self.upper() {
            self.items.get(index - self.lower)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index.
    pub fn value_mut(&mut self, index: usize) -> Option<&mut IGESGeomCurveOnSurface> {
        if index >= self.lower && index <= self.upper() {
            self.items.get_mut(index - self.lower)
        } else {
            None
        }
    }

    /// Sets the value at the given index.
    pub fn set_value(&mut self, index: usize, value: IGESGeomCurveOnSurface) -> bool {
        if index >= self.lower && index <= self.upper() {
            self.items[index - self.lower] = value;
            true
        } else {
            false
        }
    }

    /// Returns an iterator over the array.
    pub fn iter(&self) -> std::slice::Iter<IGESGeomCurveOnSurface> {
        self.items.iter()
    }
}

impl Default for IGESGeomHArray1OfCurveOnSurface {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array() {
        let arr = IGESGeomHArray1OfCurveOnSurface::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
    }

    #[test]
    fn test_value_access() {
        let mut arr = IGESGeomHArray1OfCurveOnSurface::new(1, 3);
        let elem = IGESGeomCurveOnSurface { id: 42 };
        arr.set_value(1, elem.clone());

        assert_eq!(arr.value(1).unwrap().id, 42);
    }

    #[test]
    fn test_value_mut() {
        let mut arr = IGESGeomHArray1OfCurveOnSurface::new(1, 3);
        arr.set_value(2, IGESGeomCurveOnSurface { id: 10 });

        if let Some(val) = arr.value_mut(2) {
            val.id = 99;
        }

        assert_eq!(arr.value(2).unwrap().id, 99);
    }

    #[test]
    fn test_iterator() {
        let mut arr = IGESGeomHArray1OfCurveOnSurface::new(1, 2);
        arr.set_value(1, IGESGeomCurveOnSurface { id: 1 });
        arr.set_value(2, IGESGeomCurveOnSurface { id: 2 });

        let ids: Vec<usize> = arr.iter().map(|e| e.id).collect();
        assert_eq!(ids, vec![1, 2]);
    }
}
