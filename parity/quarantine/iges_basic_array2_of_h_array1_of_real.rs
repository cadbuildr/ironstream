// FILE: iges_basic_array2_of_h_array1_of_real.rs
// occt: IGESBasic_Array2OfHArray1OfReal

/// Represents a 1D array of real numbers.
#[derive(Clone, Debug)]
pub struct HArray1OfReal {
    pub id: usize,
}

/// 2D Array of HArray1OfReal objects.
/// In OCCT, this was NCollection_Array2<opencascade::handle<TColStd_HArray1OfReal>>.
/// This Rust newtype wraps a 2D Vec for faithful behavior.
pub struct IGESBasicArray2OfHArray1OfReal {
    items: Vec<Vec<HArray1OfReal>>,
    row_lower: usize,
    col_lower: usize,
}

impl IGESBasicArray2OfHArray1OfReal {
    /// Creates a 2D array with given bounds and sizes.
    pub fn new(row_lower: usize, row_size: usize, col_lower: usize, col_size: usize) -> Self {
        let mut items = Vec::new();
        for _ in 0..row_size {
            items.push(vec![HArray1OfReal { id: 0 }; col_size]);
        }
        IGESBasicArray2OfHArray1OfReal {
            items,
            row_lower,
            col_lower,
        }
    }

    /// Returns the lower row bound.
    pub fn row_lower(&self) -> usize {
        self.row_lower
    }

    /// Returns the upper row bound.
    pub fn row_upper(&self) -> usize {
        self.row_lower + self.items.len() - 1
    }

    /// Returns the lower column bound.
    pub fn col_lower(&self) -> usize {
        self.col_lower
    }

    /// Returns the upper column bound.
    pub fn col_upper(&self) -> usize {
        if self.items.is_empty() {
            self.col_lower - 1
        } else {
            self.col_lower + self.items[0].len() - 1
        }
    }

    /// Returns a reference to the element at the given indices.
    pub fn value(&self, row: usize, col: usize) -> Option<&HArray1OfReal> {
        if row >= self.row_lower && row <= self.row_upper() && col >= self.col_lower
            && col <= self.col_upper()
        {
            self.items.get(row - self.row_lower)?.get(col - self.col_lower)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given indices.
    pub fn value_mut(&mut self, row: usize, col: usize) -> Option<&mut HArray1OfReal> {
        if row >= self.row_lower && row <= self.row_upper() && col >= self.col_lower
            && col <= self.col_upper()
        {
            self.items
                .get_mut(row - self.row_lower)?
                .get_mut(col - self.col_lower)
        } else {
            None
        }
    }

    /// Sets the value at the given indices.
    pub fn set_value(&mut self, row: usize, col: usize, value: HArray1OfReal) -> bool {
        if row >= self.row_lower && row <= self.row_upper() && col >= self.col_lower
            && col <= self.col_upper()
        {
            self.items[row - self.row_lower][col - self.col_lower] = value;
            true
        } else {
            false
        }
    }
}

impl Default for IGESBasicArray2OfHArray1OfReal {
    fn default() -> Self {
        Self::new(1, 0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array() {
        let arr = IGESBasicArray2OfHArray1OfReal::new(1, 3, 1, 4);
        assert_eq!(arr.row_lower(), 1);
        assert_eq!(arr.row_upper(), 3);
        assert_eq!(arr.col_lower(), 1);
        assert_eq!(arr.col_upper(), 4);
    }

    #[test]
    fn test_value_access() {
        let mut arr = IGESBasicArray2OfHArray1OfReal::new(1, 2, 1, 3);
        let elem = HArray1OfReal { id: 42 };
        arr.set_value(1, 1, elem.clone());

        assert_eq!(arr.value(1, 1).unwrap().id, 42);
        assert_eq!(arr.value(0, 1), None);
        assert_eq!(arr.value(1, 4), None);
    }

    #[test]
    fn test_value_mut() {
        let mut arr = IGESBasicArray2OfHArray1OfReal::new(1, 2, 1, 3);
        arr.set_value(2, 2, HArray1OfReal { id: 10 });

        if let Some(val) = arr.value_mut(2, 2) {
            val.id = 99;
        }

        assert_eq!(arr.value(2, 2).unwrap().id, 99);
    }

    #[test]
    fn test_set_value() {
        let mut arr = IGESBasicArray2OfHArray1OfReal::new(1, 2, 1, 3);
        let elem = HArray1OfReal { id: 50 };

        let result = arr.set_value(1, 2, elem);
        assert!(result);

        let result_oob = arr.set_value(5, 2, HArray1OfReal { id: 100 });
        assert!(!result_oob);
    }
}
