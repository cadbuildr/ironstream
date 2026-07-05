// FILE: viewer_test_event_manager.rs
// occt: ViewerTest_EventManager

#[derive(Clone, Debug)]
pub struct ViewerTestEventManager {
    event_count: u32,
}

impl ViewerTestEventManager {
    pub fn new() -> Self {
        ViewerTestEventManager { event_count: 0 }
    }

    pub fn handle_event(&mut self) {
        self.event_count += 1;
    }

    pub fn event_count(&self) -> u32 {
        self.event_count
    }

    pub fn clear(&mut self) {
        self.event_count = 0;
    }
}

impl Default for ViewerTestEventManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let em = ViewerTestEventManager::new();
        assert_eq!(em.event_count(), 0);
    }

    #[test]
    fn test_handle_event() {
        let mut em = ViewerTestEventManager::new();
        em.handle_event();
        assert_eq!(em.event_count(), 1);
        em.handle_event();
        assert_eq!(em.event_count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut em = ViewerTestEventManager::new();
        em.handle_event();
        em.clear();
        assert_eq!(em.event_count(), 0);
    }
}
