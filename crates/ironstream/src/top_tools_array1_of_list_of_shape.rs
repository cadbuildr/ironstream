// FILE: top_tools_array1_of_list_of_shape.rs
// occt: TopTools_Array1OfListOfShape

/// Shape: Simple shape identifier.
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

/// ListOfShape: List of shapes.
#[derive(Clone, Debug)]
pub struct ListOfShape {
    shapes: Vec<Shape>,
}

impl ListOfShape {
    pub fn new() -> Self {
        ListOfShape {
            shapes: Vec::new(),
        }
    }

    pub fn append(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    pub fn size(&self) -> usize {
        self.shapes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Shape> {
        self.shapes.iter()
    }
}

impl Default for ListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Array1OfListOfShape: 1-based array of lists.
#[derive(Clone, Debug)]
pub struct Array1OfListOfShape {
    data: Vec<ListOfShape>,
    lower: usize,
}

impl Array1OfListOfShape {
    pub fn new(size: usize) -> Self {
        Array1OfListOfShape {
            data: (0..size).map(|_| ListOfShape::new()).collect(),
            lower: 1,
        }
    }

    pub fn new_from_bounds(lower: usize, upper: usize) -> Self {
        if lower == 0 {
            panic!("OCCT arrays use 1-based indexing");
        }
        let size = upper - lower + 1;
        Array1OfListOfShape {
            data: (0..size).map(|_| ListOfShape::new()).collect(),
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

    pub fn value(&self, index_1based: usize) -> Option<&ListOfShape> {
        if index_1based < self.lower {
            None
        } else {
            self.data.get(index_1based - self.lower)
        }
    }

    pub fn value_mut(&mut self, index_1based: usize) -> Option<&mut ListOfShape> {
        if index_1based < self.lower {
            None
        } else {
            self.data.get_mut(index_1based - self.lower)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &ListOfShape> {
        self.data.iter()
    }
}

impl Default for Array1OfListOfShape {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array1_new() {
        let arr = Array1OfListOfShape::new(5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_array1_value() {
        let arr = Array1OfListOfShape::new(3);
        assert!(arr.value(0).is_none());
        assert!(arr.value(1).is_some());
    }

    #[test]
    fn test_array1_value_mut() {
        let mut arr = Array1OfListOfShape::new(2);
        if let Some(list) = arr.value_mut(1) {
            list.append(Shape::new(42));
        }
        let list = arr.value(1).unwrap();
        assert_eq!(list.size(), 1);
    }
}
