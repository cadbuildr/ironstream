// FILE: top_tools_array2_of_shape.rs
// occt: TopTools_Array2OfShape

/// Shape: Shape identifier.
#[derive(Clone, Debug)]
pub struct Shape {
    id: usize,
}

impl Shape {
    pub fn new(id: usize) -> Self {
        Shape { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Array2OfShape: 2D 1-based array of shapes (row-major).
#[derive(Clone, Debug)]
pub struct Array2OfShape {
    data: Vec<Shape>,
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
}

impl Array2OfShape {
    pub fn new(row_lower: usize, row_upper: usize, col_lower: usize, col_upper: usize) -> Self {
        if row_lower == 0 || col_lower == 0 {
            panic!("OCCT arrays use 1-based indexing");
        }
        let rows = row_upper - row_lower + 1;
        let cols = col_upper - col_lower + 1;
        let size = rows * cols;
        Array2OfShape {
            data: (0..size).map(|i| Shape::new(i)).collect(),
            row_lower,
            row_upper,
            col_lower,
            col_upper,
        }
    }

    pub fn row_lower(&self) -> usize {
        self.row_lower
    }

    pub fn row_upper(&self) -> usize {
        self.row_upper
    }

    pub fn col_lower(&self) -> usize {
        self.col_lower
    }

    pub fn col_upper(&self) -> usize {
        self.col_upper
    }

    pub fn num_rows(&self) -> usize {
        self.row_upper - self.row_lower + 1
    }

    pub fn num_cols(&self) -> usize {
        self.col_upper - self.col_lower + 1
    }

    pub fn value(&self, row_1based: usize, col_1based: usize) -> Option<&Shape> {
        if row_1based < self.row_lower || row_1based > self.row_upper {
            return None;
        }
        if col_1based < self.col_lower || col_1based > self.col_upper {
            return None;
        }
        let r = row_1based - self.row_lower;
        let c = col_1based - self.col_lower;
        let idx = r * self.num_cols() + c;
        self.data.get(idx)
    }

    pub fn set_value(&mut self, row_1based: usize, col_1based: usize, value: Shape) {
        if row_1based < self.row_lower || row_1based > self.row_upper {
            panic!("Row index out of bounds");
        }
        if col_1based < self.col_lower || col_1based > self.col_upper {
            panic!("Column index out of bounds");
        }
        let r = row_1based - self.row_lower;
        let c = col_1based - self.col_lower;
        let idx = r * self.num_cols() + c;
        self.data[idx] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array2_new() {
        let arr = Array2OfShape::new(1, 3, 1, 4);
        assert_eq!(arr.num_rows(), 3);
        assert_eq!(arr.num_cols(), 4);
    }

    #[test]
    fn test_array2_value() {
        let arr = Array2OfShape::new(1, 2, 1, 2);
        assert!(arr.value(1, 1).is_some());
        assert!(arr.value(2, 2).is_some());
        assert!(arr.value(0, 1).is_none());
    }

    #[test]
    fn test_array2_set_value() {
        let mut arr = Array2OfShape::new(1, 2, 1, 2);
        arr.set_value(1, 1, Shape::new(99));
        let val = arr.value(1, 1).unwrap();
        assert_eq!(val.id(), 99);
    }
}
