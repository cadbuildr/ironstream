// FILE: interface_data_state.rs
// occt: Interface_DataState

/// Represents the state of data in an entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterfaceDataState {
    /// Data is undefined
    Undefined = 0,
    /// Data has been loaded
    Loaded = 1,
    /// Data is in progress
    InProgress = 2,
    /// Data is complete
    Complete = 3,
}

impl InterfaceDataState {
    /// Returns true if the state represents complete data
    pub fn is_complete(&self) -> bool {
        *self == InterfaceDataState::Complete
    }

    /// Returns true if data has been loaded
    pub fn is_loaded(&self) -> bool {
        matches!(self, InterfaceDataState::Loaded | InterfaceDataState::Complete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_states() {
        let undefined = InterfaceDataState::Undefined;
        let loaded = InterfaceDataState::Loaded;
        let complete = InterfaceDataState::Complete;

        assert!(!undefined.is_complete());
        assert!(complete.is_complete());
    }

    #[test]
    fn test_is_loaded() {
        let undefined = InterfaceDataState::Undefined;
        let loaded = InterfaceDataState::Loaded;
        let complete = InterfaceDataState::Complete;

        assert!(!undefined.is_loaded());
        assert!(loaded.is_loaded());
        assert!(complete.is_loaded());
    }
}
