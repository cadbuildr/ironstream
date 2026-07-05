// FILE: top_ope_b_rep_ds_indexed_data_map_of_shape_with_state.rs
// occt: TopOpeBRepDS_IndexedDataMapOfShapeWithState, TopOpeBRepDS_ShapeWithState

/// State: Topological state enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    IN,
    OUT,
    ON,
    UNKNOWN,
}

/// ShapeKey: Simple shape identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
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

/// ShapeWithState: Pair of shape and state.
#[derive(Clone, Debug)]
pub struct ShapeWithState {
    state: State,
}

impl ShapeWithState {
    pub fn new(state: State) -> Self {
        ShapeWithState { state }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
}

/// IndexedDataMapOfShapeWithState: Indexed map (1-based) from Shape to ShapeWithState.
#[derive(Clone, Debug)]
pub struct IndexedDataMapOfShapeWithState {
    entries: Vec<(ShapeKey, ShapeWithState)>,
}

impl IndexedDataMapOfShapeWithState {
    pub fn new() -> Self {
        IndexedDataMapOfShapeWithState {
            entries: Vec::new(),
        }
    }

    /// Adds or updates an entry, returns 1-based index.
    pub fn add(&mut self, shape: ShapeKey, state: ShapeWithState) -> usize {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &shape) {
            self.entries[pos] = (shape, state);
            pos + 1
        } else {
            self.entries.push((shape, state));
            self.entries.len()
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, state: ShapeWithState) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &shape) {
            entry.1 = state;
            false
        } else {
            self.entries.push((shape, state));
            true
        }
    }

    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.entries.iter().any(|(k, _)| k == shape)
    }

    pub fn find(&self, shape: &ShapeKey) -> Option<&ShapeWithState> {
        self.entries.iter().find(|(k, _)| k == shape).map(|(_, v)| v)
    }

    pub fn find_mut(&mut self, shape: &ShapeKey) -> Option<&mut ShapeWithState> {
        self.entries
            .iter_mut()
            .find(|(k, _)| k == shape)
            .map(|(_, v)| v)
    }

    pub fn value_at(&self, index_1based: usize) -> Option<&ShapeWithState> {
        if index_1based == 0 {
            None
        } else {
            self.entries.get(index_1based - 1).map(|(_, v)| v)
        }
    }

    pub fn key_at(&self, index_1based: usize) -> Option<&ShapeKey> {
        if index_1based == 0 {
            None
        } else {
            self.entries.get(index_1based - 1).map(|(k, _)| k)
        }
    }

    pub fn remove(&mut self, shape: &ShapeKey) -> bool {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == shape) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn lower(&self) -> usize {
        1
    }

    pub fn upper(&self) -> usize {
        self.entries.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ShapeKey, &ShapeWithState)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }
}

impl Default for IndexedDataMapOfShapeWithState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_with_state() {
        let mut sws = ShapeWithState::new(State::IN);
        assert_eq!(sws.state(), State::IN);
        sws.set_state(State::OUT);
        assert_eq!(sws.state(), State::OUT);
    }

    #[test]
    fn test_indexed_map_add() {
        let mut map = IndexedDataMapOfShapeWithState::new();
        let shape1 = ShapeKey::new(1);
        let state1 = ShapeWithState::new(State::IN);
        let idx1 = map.add(shape1, state1);
        assert_eq!(idx1, 1);

        let shape2 = ShapeKey::new(2);
        let state2 = ShapeWithState::new(State::OUT);
        let idx2 = map.add(shape2, state2);
        assert_eq!(idx2, 2);
    }

    #[test]
    fn test_indexed_map_bind() {
        let mut map = IndexedDataMapOfShapeWithState::new();
        let shape = ShapeKey::new(5);
        let state = ShapeWithState::new(State::ON);

        assert!(map.bind(shape.clone(), state));
        assert!(!map.bind(shape, ShapeWithState::new(State::IN)));
    }

    #[test]
    fn test_indexed_map_contains() {
        let mut map = IndexedDataMapOfShapeWithState::new();
        let shape = ShapeKey::new(5);
        assert!(!map.contains(&shape));

        map.bind(shape.clone(), ShapeWithState::new(State::IN));
        assert!(map.contains(&shape));
    }

    #[test]
    fn test_indexed_map_value_at() {
        let mut map = IndexedDataMapOfShapeWithState::new();
        let shape1 = ShapeKey::new(1);
        let shape2 = ShapeKey::new(2);
        map.add(shape1, ShapeWithState::new(State::IN));
        map.add(shape2, ShapeWithState::new(State::OUT));

        assert!(map.value_at(0).is_none());
        assert_eq!(map.value_at(1).unwrap().state(), State::IN);
        assert_eq!(map.value_at(2).unwrap().state(), State::OUT);
    }

    #[test]
    fn test_indexed_map_remove() {
        let mut map = IndexedDataMapOfShapeWithState::new();
        let shape = ShapeKey::new(7);
        map.bind(shape.clone(), ShapeWithState::new(State::ON));

        assert_eq!(map.size(), 1);
        assert!(map.remove(&shape));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_indexed_map_bounds() {
        let mut map = IndexedDataMapOfShapeWithState::new();
        map.add(ShapeKey::new(1), ShapeWithState::new(State::IN));
        map.add(ShapeKey::new(2), ShapeWithState::new(State::OUT));

        assert_eq!(map.lower(), 1);
        assert_eq!(map.upper(), 2);
    }
}
