// FILE: top_ope_b_rep_build_list_of_shape_list_of_shape.rs
// occt: TopOpeBRepBuild_ListOfShapeListOfShape
// occt-ref: TopOpeBRepBuild_ShapeListOfShape

/// SimpleShape: Simplified shape representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleShape {
    id: usize,
}

impl SimpleShape {
    pub fn new(id: usize) -> Self {
        SimpleShape { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// ListOfShapeSimple: A list of simple shapes.
#[derive(Clone, Debug)]
pub struct ListOfShapeSimple {
    shapes: Vec<SimpleShape>,
}

impl ListOfShapeSimple {
    pub fn new() -> Self {
        ListOfShapeSimple {
            shapes: Vec::new(),
        }
    }

    pub fn append(&mut self, shape: SimpleShape) {
        self.shapes.push(shape);
    }

    pub fn size(&self) -> usize {
        self.shapes.len()
    }

    pub fn length(&self) -> usize {
        self.shapes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &SimpleShape> {
        self.shapes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SimpleShape> {
        self.shapes.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&SimpleShape> {
        self.shapes.get(index)
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

impl Default for ListOfShapeSimple {
    fn default() -> Self {
        Self::new()
    }
}

/// ShapeListOfShape: Pair of a shape with list of shapes.
#[derive(Clone, Debug)]
pub struct ShapeListOfShape {
    shape: SimpleShape,
    shapes: ListOfShapeSimple,
}

impl ShapeListOfShape {
    pub fn new(shape: SimpleShape) -> Self {
        ShapeListOfShape {
            shape,
            shapes: ListOfShapeSimple::new(),
        }
    }

    pub fn shape(&self) -> &SimpleShape {
        &self.shape
    }

    pub fn set_shape(&mut self, shape: SimpleShape) {
        self.shape = shape;
    }

    pub fn shapes(&self) -> &ListOfShapeSimple {
        &self.shapes
    }

    pub fn shapes_mut(&mut self) -> &mut ListOfShapeSimple {
        &mut self.shapes
    }

    pub fn append_shape(&mut self, shape: SimpleShape) {
        self.shapes.append(shape);
    }
}

/// ListOfShapeListOfShape: OCCT list of ShapeListOfShape pairs.
#[derive(Clone, Debug)]
pub struct ListOfShapeListOfShape {
    entries: Vec<ShapeListOfShape>,
}

impl ListOfShapeListOfShape {
    pub fn new() -> Self {
        ListOfShapeListOfShape {
            entries: Vec::new(),
        }
    }

    pub fn append(&mut self, entry: ShapeListOfShape) {
        self.entries.push(entry);
    }

    pub fn prepend(&mut self, entry: ShapeListOfShape) {
        self.entries.insert(0, entry);
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn length(&self) -> usize {
        self.entries.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ShapeListOfShape> {
        self.entries.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ShapeListOfShape> {
        self.entries.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&ShapeListOfShape> {
        self.entries.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut ShapeListOfShape> {
        self.entries.get_mut(index)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn remove(&mut self, index: usize) -> Option<ShapeListOfShape> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }
}

impl Default for ListOfShapeListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// ListIterator: Iterator for ListOfShapeListOfShape.
pub struct ListIterator {
    entries: Vec<ShapeListOfShape>,
    index: usize,
}

impl ListIterator {
    pub fn new(list: &ListOfShapeListOfShape) -> Self {
        ListIterator {
            entries: list.entries.clone(),
            index: 0,
        }
    }

    pub fn is_more(&self) -> bool {
        self.index < self.entries.len()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn current(&self) -> Option<&ShapeListOfShape> {
        self.entries.get(self.index)
    }

    pub fn value(&self) -> Option<&ShapeListOfShape> {
        self.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_shape() {
        let shape = SimpleShape::new(5);
        assert_eq!(shape.id(), 5);
    }

    #[test]
    fn test_list_of_shape_simple() {
        let mut list = ListOfShapeSimple::new();
        list.append(SimpleShape::new(1));
        list.append(SimpleShape::new(2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_shape_list_of_shape() {
        let shape = SimpleShape::new(10);
        let mut entry = ShapeListOfShape::new(shape.clone());
        entry.append_shape(SimpleShape::new(20));
        entry.append_shape(SimpleShape::new(30));

        assert_eq!(entry.shape().id(), 10);
        assert_eq!(entry.shapes().size(), 2);
    }

    #[test]
    fn test_list_of_shape_list_of_shape_append() {
        let mut list = ListOfShapeListOfShape::new();
        let entry1 = ShapeListOfShape::new(SimpleShape::new(1));
        let entry2 = ShapeListOfShape::new(SimpleShape::new(2));

        list.append(entry1);
        list.append(entry2);
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_list_of_shape_list_of_shape_prepend() {
        let mut list = ListOfShapeListOfShape::new();
        list.append(ShapeListOfShape::new(SimpleShape::new(2)));
        list.prepend(ShapeListOfShape::new(SimpleShape::new(1)));

        assert_eq!(list.get(0).unwrap().shape().id(), 1);
        assert_eq!(list.get(1).unwrap().shape().id(), 2);
    }

    #[test]
    fn test_list_of_shape_list_of_shape_clear() {
        let mut list = ListOfShapeListOfShape::new();
        list.append(ShapeListOfShape::new(SimpleShape::new(1)));
        list.append(ShapeListOfShape::new(SimpleShape::new(2)));
        list.clear();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_list_of_shape_list_of_shape_remove() {
        let mut list = ListOfShapeListOfShape::new();
        list.append(ShapeListOfShape::new(SimpleShape::new(1)));
        list.append(ShapeListOfShape::new(SimpleShape::new(2)));
        list.append(ShapeListOfShape::new(SimpleShape::new(3)));

        let removed = list.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().shape().id(), 2);
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_list_iterator() {
        let mut list = ListOfShapeListOfShape::new();
        list.append(ShapeListOfShape::new(SimpleShape::new(1)));
        list.append(ShapeListOfShape::new(SimpleShape::new(2)));

        let mut iter = ListIterator::new(&list);
        assert!(iter.is_more());
        assert_eq!(iter.current().unwrap().shape().id(), 1);
        iter.next();
        assert!(iter.is_more());
        assert_eq!(iter.current().unwrap().shape().id(), 2);
        iter.next();
        assert!(!iter.is_more());
    }
}
