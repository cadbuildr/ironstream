// FILE: step_tidy_duplicate_cleaner.rs
// occt: StepTidy_DuplicateCleaner

use std::sync::Arc;
use std::collections::HashSet;

/// Placeholder for XSControl_WorkSession
pub struct WorkSession {
    entities: HashSet<usize>,
}

impl WorkSession {
    pub fn new() -> Self {
        WorkSession {
            entities: HashSet::new(),
        }
    }

    pub fn add_entity(&mut self, id: usize) {
        self.entities.insert(id);
    }

    pub fn remove_entity(&mut self, id: usize) -> bool {
        self.entities.remove(&id)
    }

    pub fn entities(&self) -> &HashSet<usize> {
        &self.entities
    }

    pub fn count(&self) -> usize {
        self.entities.len()
    }
}

impl Default for WorkSession {
    fn default() -> Self {
        Self::new()
    }
}

/// A class to merge STEP entities and remove duplicates.
/// This class is used to merge equal STEP entities in the work session
/// and remove duplicates. Equal or duplicate entities are those that
/// have equal names and very close numerical values.
pub struct DuplicateCleaner {
    work_session: Arc<WorkSession>,
}

impl DuplicateCleaner {
    /// Create a new DuplicateCleaner with a work session
    pub fn new(work_session: Arc<WorkSession>) -> Self {
        DuplicateCleaner { work_session }
    }

    /// Perform the merging of entities.
    /// All entities in the model stored in the work session that are considered equal to
    /// each other will be merged, and duplicates will be removed.
    pub fn perform(&mut self) {
        // This is a simplified implementation
        // In real use, this would identify and merge duplicate entities
    }

    /// Get the work session
    pub fn work_session(&self) -> &Arc<WorkSession> {
        &self.work_session
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_session_creation() {
        let ws = WorkSession::new();
        assert_eq!(ws.count(), 0);
    }

    #[test]
    fn test_add_entity() {
        let mut ws = WorkSession::new();
        ws.add_entity(1);
        ws.add_entity(2);

        assert_eq!(ws.count(), 2);
    }

    #[test]
    fn test_remove_entity() {
        let mut ws = WorkSession::new();
        ws.add_entity(1);
        ws.add_entity(2);

        let removed = ws.remove_entity(1);
        assert!(removed);
        assert_eq!(ws.count(), 1);

        let removed_again = ws.remove_entity(1);
        assert!(!removed_again);
    }

    #[test]
    fn test_duplicate_cleaner_creation() {
        let ws = Arc::new(WorkSession::new());
        let cleaner = DuplicateCleaner::new(ws.clone());

        assert_eq!(cleaner.work_session().count(), 0);
    }

    #[test]
    fn test_perform_with_empty_session() {
        let ws = Arc::new(WorkSession::new());
        let mut cleaner = DuplicateCleaner::new(ws.clone());

        // Perform should not crash on empty session
        cleaner.perform();
        assert_eq!(cleaner.work_session().count(), 0);
    }

    #[test]
    fn test_perform_with_entities() {
        let mut ws = WorkSession::new();
        ws.add_entity(1);
        ws.add_entity(2);

        let ws_arc = Arc::new(ws);
        let mut cleaner = DuplicateCleaner::new(ws_arc.clone());

        cleaner.perform();
        // After perform, entities should still be there
        // (simplified implementation doesn't actually merge)
        assert!(cleaner.work_session().count() > 0);
    }
}
