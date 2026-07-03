// FILE: aspect_xr_action_set.rs
// occt: Aspect_XRActionSet

use crate::aspect_xr_action::AspectXRAction;
use std::collections::BTreeMap;

/// XR action set.
///
/// Contains a collection of XR actions organized by action ID.
#[derive(Debug, Clone)]
pub struct AspectXRActionSet {
    /// Action set ID
    id: String,
    /// Action set handle
    raw_handle: u64,
    /// Map of actions indexed by ID
    actions: BTreeMap<String, AspectXRAction>,
}

impl AspectXRActionSet {
    /// Create a new action set.
    /// @param id action set identifier
    pub fn new(id: String) -> Self {
        AspectXRActionSet {
            id,
            raw_handle: 0,
            actions: BTreeMap::new(),
        }
    }

    /// Return action set ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return action set handle.
    pub fn raw_handle(&self) -> u64 {
        self.raw_handle
    }

    /// Set action set handle.
    pub fn set_raw_handle(&mut self, handle: u64) {
        self.raw_handle = handle;
    }

    /// Add action to the action set.
    /// @param action action to add
    pub fn add_action(&mut self, action: AspectXRAction) {
        self.actions.insert(action.id().to_string(), action);
    }

    /// Return map of actions (immutable reference).
    pub fn actions(&self) -> &BTreeMap<String, AspectXRAction> {
        &self.actions
    }

    /// Return mutable reference to actions map.
    pub fn actions_mut(&mut self) -> &mut BTreeMap<String, AspectXRAction> {
        &mut self.actions
    }

    /// Return action by ID, if it exists.
    pub fn get_action(&self, id: &str) -> Option<&AspectXRAction> {
        self.actions.get(id)
    }

    /// Return action by ID, mutable reference, if it exists.
    pub fn get_action_mut(&mut self, id: &str) -> Option<&mut AspectXRAction> {
        self.actions.get_mut(id)
    }

    /// Check if action with given ID exists.
    pub fn has_action(&self, id: &str) -> bool {
        self.actions.contains_key(id)
    }

    /// Remove action by ID.
    /// @return TRUE if action was removed
    pub fn remove_action(&mut self, id: &str) -> bool {
        self.actions.remove(id).is_some()
    }

    /// Get number of actions in the set.
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aspect_xr_action_type::AspectXRActionType;

    #[test]
    fn test_create_action_set() {
        let action_set = AspectXRActionSet::new("test_set".to_string());
        assert_eq!(action_set.id(), "test_set");
        assert_eq!(action_set.raw_handle(), 0);
        assert_eq!(action_set.action_count(), 0);
    }

    #[test]
    fn test_set_raw_handle() {
        let mut action_set = AspectXRActionSet::new("handle_test".to_string());
        action_set.set_raw_handle(99999);
        assert_eq!(action_set.raw_handle(), 99999);
    }

    #[test]
    fn test_add_action() {
        let mut action_set = AspectXRActionSet::new("actions_test".to_string());
        assert_eq!(action_set.action_count(), 0);

        let action =
            AspectXRAction::new("grip".to_string(), AspectXRActionType::InputDigital);
        action_set.add_action(action);

        assert_eq!(action_set.action_count(), 1);
        assert!(action_set.has_action("grip"));
    }

    #[test]
    fn test_get_action() {
        let mut action_set = AspectXRActionSet::new("get_test".to_string());
        let action =
            AspectXRAction::new("trigger".to_string(), AspectXRActionType::InputAnalog);
        action_set.add_action(action);

        let retrieved = action_set.get_action("trigger");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), "trigger");
    }

    #[test]
    fn test_remove_action() {
        let mut action_set = AspectXRActionSet::new("remove_test".to_string());
        let action =
            AspectXRAction::new("button".to_string(), AspectXRActionType::InputDigital);
        action_set.add_action(action);

        assert_eq!(action_set.action_count(), 1);
        let removed = action_set.remove_action("button");
        assert!(removed);
        assert_eq!(action_set.action_count(), 0);
    }

    #[test]
    fn test_multiple_actions() {
        let mut action_set = AspectXRActionSet::new("multi_test".to_string());

        let action1 =
            AspectXRAction::new("pose1".to_string(), AspectXRActionType::InputPose);
        let action2 =
            AspectXRAction::new("pose2".to_string(), AspectXRActionType::InputPose);
        let action3 = AspectXRAction::new(
            "haptic".to_string(),
            AspectXRActionType::OutputHaptic,
        );

        action_set.add_action(action1);
        action_set.add_action(action2);
        action_set.add_action(action3);

        assert_eq!(action_set.action_count(), 3);
        assert!(action_set.has_action("pose1"));
        assert!(action_set.has_action("pose2"));
        assert!(action_set.has_action("haptic"));
        assert!(!action_set.has_action("nonexistent"));
    }

    #[test]
    fn test_has_action() {
        let mut action_set = AspectXRActionSet::new("has_test".to_string());
        let action = AspectXRAction::new(
            "skeletal".to_string(),
            AspectXRActionType::InputSkeletal,
        );
        action_set.add_action(action);

        assert!(action_set.has_action("skeletal"));
        assert!(!action_set.has_action("missing"));
    }

    #[test]
    fn test_action_set_clone() {
        let mut action_set = AspectXRActionSet::new("clone_test".to_string());
        action_set.set_raw_handle(12345);

        let action = AspectXRAction::new("test".to_string(), AspectXRActionType::InputDigital);
        action_set.add_action(action);

        let cloned = action_set.clone();
        assert_eq!(cloned.id(), "clone_test");
        assert_eq!(cloned.raw_handle(), 12345);
        assert_eq!(cloned.action_count(), 1);
        assert!(cloned.has_action("test"));
    }

    #[test]
    fn test_actions_iterator() {
        let mut action_set = AspectXRActionSet::new("iter_test".to_string());

        for i in 0..3 {
            let action = AspectXRAction::new(
                format!("action_{}", i),
                AspectXRActionType::InputDigital,
            );
            action_set.add_action(action);
        }

        let actions = action_set.actions();
        let keys: Vec<_> = actions.keys().collect();
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn test_get_action_mut() {
        let mut action_set = AspectXRActionSet::new("mut_test".to_string());
        let mut action = AspectXRAction::new("test".to_string(), AspectXRActionType::InputDigital);
        action.set_raw_handle(111);
        action_set.add_action(action);

        if let Some(mut_action) = action_set.get_action_mut("test") {
            mut_action.set_raw_handle(222);
        }

        let retrieved = action_set.get_action("test").unwrap();
        assert_eq!(retrieved.raw_handle(), 222);
    }
}
