// FILE: top_ope_b_rep_ds_data_map_of_shape_state.rs
// occt: TopOpeBRepDS_DataMapOfShapeState, TopOpeBRepDS_State

use std::collections::HashMap;

/// State: Enumeration for topological state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum State {
    IN = 0,
    OUT = 1,
    ON = 2,
    UNKNOWN = 3,
}

impl State {
    pub fn from_int(value: i32) -> Self {
        match value {
            0 => State::IN,
            1 => State::OUT,
            2 => State::ON,
            _ => State::UNKNOWN,
        }
    }

    pub fn as_int(&self) -> i32 {
        *self as i32
    }
}

/// ShapeKey: Simplified shape key.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeKey {
    id: usize,
}

impl ShapeKey {
    pub fn new(id: usize) -> Self {
        ShapeKey { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// DataMapOfShapeState: Maps Shape to State.
#[derive(Clone, Debug)]
pub struct DataMapOfShapeState {
    data: HashMap<ShapeKey, State>,
}

impl DataMapOfShapeState {
    pub fn new() -> Self {
        DataMapOfShapeState {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, state: State) -> bool {
        self.data.insert(shape, state).is_none()
    }

    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.data.contains_key(shape)
    }

    pub fn find(&self, shape: &ShapeKey) -> Option<State> {
        self.data.get(shape).copied()
    }

    pub fn remove(&mut self, shape: &ShapeKey) -> bool {
        self.data.remove(shape).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ShapeKey, &State)> {
        self.data.iter()
    }
}

impl Default for DataMapOfShapeState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_variants() {
        assert_eq!(State::IN.as_int(), 0);
        assert_eq!(State::OUT.as_int(), 1);
        assert_eq!(State::ON.as_int(), 2);
        assert_eq!(State::UNKNOWN.as_int(), 3);
    }

    #[test]
    fn test_state_from_int() {
        assert_eq!(State::from_int(0), State::IN);
        assert_eq!(State::from_int(1), State::OUT);
        assert_eq!(State::from_int(2), State::ON);
        assert_eq!(State::from_int(999), State::UNKNOWN);
    }

    #[test]
    fn test_shape_key() {
        let key = ShapeKey::new(42);
        assert_eq!(key.id(), 42);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfShapeState::new();
        let shape = ShapeKey::new(1);
        assert!(map.bind(shape.clone(), State::IN));
        assert!(!map.bind(shape, State::OUT));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfShapeState::new();
        let shape = ShapeKey::new(3);
        map.bind(shape.clone(), State::ON);

        let state = map.find(&shape).unwrap();
        assert_eq!(state, State::ON);
    }

    #[test]
    fn test_data_map_remove() {
        let mut map = DataMapOfShapeState::new();
        let shape = ShapeKey::new(7);
        map.bind(shape.clone(), State::OUT);

        assert_eq!(map.size(), 1);
        assert!(map.remove(&shape));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_data_map_clear() {
        let mut map = DataMapOfShapeState::new();
        map.bind(ShapeKey::new(1), State::IN);
        map.bind(ShapeKey::new(2), State::OUT);
        map.clear();
        assert_eq!(map.size(), 0);
    }
}
