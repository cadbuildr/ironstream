// FILE: message_lazy_progress_scope.rs
// occt: Message_LazyProgressScope

/// Lazy progress scope for deferred progress tracking.
pub struct MessageLazyProgressScope {
    name: String,
    total: i32,
    current: i32,
}

impl MessageLazyProgressScope {
    pub fn new(name: &str, total: i32) -> Self {
        Self {
            name: name.to_string(),
            total,
            current: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn total(&self) -> i32 {
        self.total
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn advance(&mut self) {
        if self.current < self.total {
            self.current += 1;
        }
    }

    pub fn progress(&self) -> f64 {
        if self.total > 0 {
            self.current as f64 / self.total as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let scope = MessageLazyProgressScope::new("task", 10);
        assert_eq!(scope.name(), "task");
        assert_eq!(scope.total(), 10);
        assert_eq!(scope.current(), 0);
    }

    #[test]
    fn test_advance() {
        let mut scope = MessageLazyProgressScope::new("task", 10);
        scope.advance();
        assert_eq!(scope.current(), 1);
    }

    #[test]
    fn test_progress() {
        let mut scope = MessageLazyProgressScope::new("task", 10);
        assert_eq!(scope.progress(), 0.0);
        scope.current = 5;
        assert_eq!(scope.progress(), 0.5);
    }
}
