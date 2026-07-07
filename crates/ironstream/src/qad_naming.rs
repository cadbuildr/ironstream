// FILE: qad_naming.rs
// occt: QADNaming

//! QA test package for naming operations.

/// Naming context
#[derive(Debug, Clone)]
pub struct NamingContext {
    id: u32,
}

impl NamingContext {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Naming test handler
#[derive(Debug)]
pub struct QADNamingManager {
    contexts: Vec<NamingContext>,
}

impl QADNamingManager {
    pub fn new() -> Self {
        Self {
            contexts: Vec::new(),
        }
    }

    pub fn create_context(&mut self, id: u32) -> &NamingContext {
        self.contexts.push(NamingContext::new(id));
        self.contexts.last().unwrap()
    }

    pub fn num_contexts(&self) -> usize {
        self.contexts.len()
    }

    pub fn get_context(&self, id: u32) -> Option<&NamingContext> {
        self.contexts.iter().find(|c| c.id == id)
    }
}

impl Default for QADNamingManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_context() {
        let mut manager = QADNamingManager::new();
        manager.create_context(1);
        assert_eq!(manager.num_contexts(), 1);
    }

    #[test]
    fn test_get_context() {
        let mut manager = QADNamingManager::new();
        manager.create_context(42);
        assert!(manager.get_context(42).is_some());
        assert!(manager.get_context(99).is_none());
    }
}
