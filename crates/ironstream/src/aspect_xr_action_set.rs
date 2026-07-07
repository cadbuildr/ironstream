// FILE: aspect_xr_action_set.rs
// occt: Aspect_XRActionSet

use std::collections::HashMap;
use std::sync::Arc;

/// XR action definition.
pub struct AspectXRAction {
    id: String,
    raw_handle: u64,
}

impl AspectXRAction {
    /// Create a new XR action.
    pub fn new(id: impl Into<String>) -> Self {
        AspectXRAction {
            id: id.into(),
            raw_handle: 0,
        }
    }

    /// Return the action id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the action handle.
    pub fn raw_handle(&self) -> u64 {
        self.raw_handle
    }

    /// Set the action handle.
    pub fn set_raw_handle(&mut self, handle: u64) {
        self.raw_handle = handle;
    }
}

/// Type alias for a reference-counted XR action.
pub type AspectXRActionHandle = Arc<AspectXRAction>;

/// XR action set.
pub struct AspectXRActionSet {
    id: String,
    raw_handle: u64,
    actions: HashMap<String, AspectXRActionHandle>,
}

impl AspectXRActionSet {
    /// Create a new XR action set with the given id.
    pub fn new(id: impl Into<String>) -> Self {
        AspectXRActionSet {
            id: id.into(),
            raw_handle: 0,
            actions: HashMap::new(),
        }
    }

    /// Return the action set id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the action set handle.
    pub fn raw_handle(&self) -> u64 {
        self.raw_handle
    }

    /// Set the action set handle.
    pub fn set_raw_handle(&mut self, handle: u64) {
        self.raw_handle = handle;
    }

    /// Add an action to this action set.
    pub fn add_action(&mut self, action: AspectXRActionHandle) {
        self.actions.insert(action.id().to_string(), action);
    }

    /// Return a reference to the map of actions.
    pub fn actions(&self) -> &HashMap<String, AspectXRActionHandle> {
        &self.actions
    }

    /// Return a mutable reference to the map of actions.
    pub fn actions_mut(&mut self) -> &mut HashMap<String, AspectXRActionHandle> {
        &mut self.actions
    }

    /// Get an action by id.
    pub fn get_action(&self, id: &str) -> Option<&AspectXRActionHandle> {
        self.actions.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xr_action_set_creation() {
        let action_set = AspectXRActionSet::new("test_set");
        assert_eq!(action_set.id(), "test_set");
        assert_eq!(action_set.raw_handle(), 0);
        assert!(action_set.actions().is_empty());
    }

    #[test]
    fn test_xr_action_set_handle() {
        let mut action_set = AspectXRActionSet::new("pose_set");
        action_set.set_raw_handle(9999);
        assert_eq!(action_set.raw_handle(), 9999);
    }

    #[test]
    fn test_add_action_to_set() {
        let mut action_set = AspectXRActionSet::new("input_set");
        let action = AspectXRActionHandle::new(AspectXRAction::new("press_button"));

        action_set.add_action(action.clone());
        assert_eq!(action_set.actions().len(), 1);

        let retrieved = action_set.get_action("press_button");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), "press_button");
    }

    #[test]
    fn test_multiple_actions_in_set() {
        let mut action_set = AspectXRActionSet::new("multi_action_set");

        let action1 = AspectXRActionHandle::new(AspectXRAction::new("action1"));
        let action2 = AspectXRActionHandle::new(AspectXRAction::new("action2"));

        action_set.add_action(action1);
        action_set.add_action(action2);

        assert_eq!(action_set.actions().len(), 2);
        assert!(action_set.get_action("action1").is_some());
        assert!(action_set.get_action("action2").is_some());
    }
}
