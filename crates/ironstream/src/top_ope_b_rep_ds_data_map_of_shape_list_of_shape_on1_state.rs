// FILE: top_ope_b_rep_ds_data_map_of_shape_list_of_shape_on1_state.rs
// occt: TopOpeBRepDS_DataMapOfShapeListOfShapeOn1State
// occt-ref: TopOpeBRepDS_ShapeListOfShapeOn1State

use std::collections::HashMap;

/// ShapeState: State enumeration for shape containment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShapeState {
    IN = 0,
    OUT = 1,
    ON = 2,
}

impl ShapeState {
    pub fn from_int(value: i32) -> Self {
        match value {
            0 => ShapeState::IN,
            1 => ShapeState::OUT,
            2 => ShapeState::ON,
            _ => ShapeState::OUT,
        }
    }
}

/// ShapeSimple: Simplified shape.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeSimple {
    id: usize,
}

impl ShapeSimple {
    pub fn new(id: usize) -> Self {
        ShapeSimple { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// ListOfShape: Simple list of shapes.
#[derive(Clone, Debug)]
pub struct ListOfShape {
    shapes: Vec<ShapeSimple>,
}

impl ListOfShape {
    pub fn new() -> Self {
        ListOfShape {
            shapes: Vec::new(),
        }
    }

    pub fn append(&mut self, shape: ShapeSimple) {
        self.shapes.push(shape);
    }

    pub fn size(&self) -> usize {
        self.shapes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ShapeSimple> {
        self.shapes.iter()
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

impl Default for ListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// ShapeListOfShapeOn1State: Pair of shape and list with state.
#[derive(Clone, Debug)]
pub struct ShapeListOfShapeOn1State {
    shapes: ListOfShape,
    state: ShapeState,
}

impl ShapeListOfShapeOn1State {
    pub fn new(state: ShapeState) -> Self {
        ShapeListOfShapeOn1State {
            shapes: ListOfShape::new(),
            state,
        }
    }

    pub fn shapes(&self) -> &ListOfShape {
        &self.shapes
    }

    pub fn shapes_mut(&mut self) -> &mut ListOfShape {
        &mut self.shapes
    }

    pub fn state(&self) -> ShapeState {
        self.state
    }

    pub fn set_state(&mut self, state: ShapeState) {
        self.state = state;
    }

    pub fn append_shape(&mut self, shape: ShapeSimple) {
        self.shapes.append(shape);
    }
}

/// DataMapOfShapeListOfShapeOn1State: Maps Shape -> ShapeListOfShapeOn1State.
#[derive(Clone, Debug)]
pub struct DataMapOfShapeListOfShapeOn1State {
    data: HashMap<ShapeSimple, ShapeListOfShapeOn1State>,
}

impl DataMapOfShapeListOfShapeOn1State {
    pub fn new() -> Self {
        DataMapOfShapeListOfShapeOn1State {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: ShapeSimple, value: ShapeListOfShapeOn1State) -> bool {
        self.data.insert(shape, value).is_none()
    }

    pub fn contains(&self, shape: &ShapeSimple) -> bool {
        self.data.contains_key(shape)
    }

    pub fn find(&self, shape: &ShapeSimple) -> Option<&ShapeListOfShapeOn1State> {
        self.data.get(shape)
    }

    pub fn find_mut(&mut self, shape: &ShapeSimple) -> Option<&mut ShapeListOfShapeOn1State> {
        self.data.get_mut(shape)
    }

    pub fn remove(&mut self, shape: &ShapeSimple) -> bool {
        self.data.remove(shape).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ShapeSimple, &ShapeListOfShapeOn1State)> {
        self.data.iter()
    }
}

impl Default for DataMapOfShapeListOfShapeOn1State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_state() {
        assert_eq!(ShapeState::from_int(0), ShapeState::IN);
        assert_eq!(ShapeState::from_int(1), ShapeState::OUT);
        assert_eq!(ShapeState::from_int(2), ShapeState::ON);
    }

    #[test]
    fn test_shape_simple() {
        let shape = ShapeSimple::new(42);
        assert_eq!(shape.id(), 42);
    }

    #[test]
    fn test_list_of_shape() {
        let mut list = ListOfShape::new();
        list.append(ShapeSimple::new(1));
        list.append(ShapeSimple::new(2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_shape_list_of_shape_on1_state() {
        let mut entry = ShapeListOfShapeOn1State::new(ShapeState::IN);
        entry.append_shape(ShapeSimple::new(10));
        assert_eq!(entry.state(), ShapeState::IN);
        assert_eq!(entry.shapes().size(), 1);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfShapeListOfShapeOn1State::new();
        let shape = ShapeSimple::new(5);
        let entry = ShapeListOfShapeOn1State::new(ShapeState::OUT);

        assert!(map.bind(shape.clone(), entry));
        assert!(!map.bind(shape, ShapeListOfShapeOn1State::new(ShapeState::ON)));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfShapeListOfShapeOn1State::new();
        let shape = ShapeSimple::new(3);
        let entry = ShapeListOfShapeOn1State::new(ShapeState::ON);
        map.bind(shape.clone(), entry);

        let found = map.find(&shape).unwrap();
        assert_eq!(found.state(), ShapeState::ON);
    }

    #[test]
    fn test_data_map_remove() {
        let mut map = DataMapOfShapeListOfShapeOn1State::new();
        let shape = ShapeSimple::new(7);
        map.bind(shape.clone(), ShapeListOfShapeOn1State::new(ShapeState::IN));

        assert_eq!(map.size(), 1);
        assert!(map.remove(&shape));
        assert_eq!(map.size(), 0);
    }
}
