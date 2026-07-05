// FILE: top_tools_array1_of_shape.rs
// occt: TopTools_Array1OfShape

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

/// Array1OfShape: 1-based array of shapes.
#[derive(Clone, Debug)]
pub struct Array1OfShape {
    data: Vec<Shape>,
    lower: usize,
}

impl Array1OfShape {
    pub fn new(size: usize) -> Self {
        Array1OfShape {
            data: (0..size).map(|i| Shape::new(i)).collect(),
            lower: 1,
        }
    }

    pub fn new_from_bounds(lower: usize, upper: usize) -> Self {
        if lower == 0 {
            panic!("OCCT arrays use 1-based indexing");
        }
        let size = upper - lower + 1;
        Array1OfShape {
            data: (0..size).map(|i| Shape::new(i)).collect(),
            lower,
        }
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> usize {
        self.lower + self.data.len() - 1
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn value(&self, index_1based: usize) -> Option<&Shape> {
        if index_1based < self.lower {
            None
        } else {
            self.data.get(index_1based - self.lower)
        }
    }

    pub fn set_value(&mut self, index_1based: usize, value: Shape) {
        let idx = index_1based - self.lower;
        if idx >= self.data.len() {
            panic!("Index out of bounds");
        }
        self.data[idx] = value;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Shape> {
        self.data.iter()
    }
}

impl Default for Array1OfShape {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_new() {
        let shape = Shape::new(42);
        assert_eq!(shape.id(), 42);
    }

    #[test]
    fn test_array1_new() {
        let arr = Array1OfShape::new(5);
        assert_eq!(arr.length(), 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
    }

    #[test]
    fn test_array1_value() {
        let arr = Array1OfShape::new(3);
        assert!(arr.value(0).is_none());
        assert!(arr.value(1).is_some());
    }
}
