// FILE: standard_condition.rs
// occt: Standard_Condition

/// Condition variable for thread synchronization
#[derive(Debug)]
pub struct StandardCondition {
    signaled: std::sync::Arc<std::sync::Mutex<bool>>,
}

impl StandardCondition {
    /// Creates a new condition variable
    pub fn new(initial: bool) -> Self {
        StandardCondition {
            signaled: std::sync::Arc::new(std::sync::Mutex::new(initial)),
        }
    }

    /// Sets the condition to signaled state
    pub fn set(&self) {
        if let Ok(mut guard) = self.signaled.lock() {
            *guard = true;
        }
    }

    /// Resets the condition
    pub fn reset(&self) {
        if let Ok(mut guard) = self.signaled.lock() {
            *guard = false;
        }
    }

    /// Checks if condition is signaled
    pub fn is_signaled(&self) -> bool {
        self.signaled.lock().map(|g| *g).unwrap_or(false)
    }

    /// Waits for condition to be signaled
    pub fn wait(&self) -> bool {
        let mut retries = 0;
        while !self.is_signaled() && retries < 1000 {
            std::thread::yield_now();
            retries += 1;
        }
        self.is_signaled()
    }
}

impl Clone for StandardCondition {
    fn clone(&self) -> Self {
        StandardCondition {
            signaled: std::sync::Arc::clone(&self.signaled),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_condition_new() {
        let cond = StandardCondition::new(false);
        assert!(!cond.is_signaled());
    }

    #[test]
    fn test_standard_condition_set() {
        let cond = StandardCondition::new(false);
        cond.set();
        assert!(cond.is_signaled());
    }

    #[test]
    fn test_standard_condition_reset() {
        let cond = StandardCondition::new(true);
        cond.reset();
        assert!(!cond.is_signaled());
    }

    #[test]
    fn test_standard_condition_wait() {
        let cond = StandardCondition::new(true);
        assert!(cond.wait());
    }
}
