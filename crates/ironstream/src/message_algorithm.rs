// FILE: message_algorithm.rs
// occt: Message_Algorithm

/// Base class for algorithms with message reporting capabilities.
pub struct MessageAlgorithm {
    name: String,
    is_running: bool,
}

impl MessageAlgorithm {
    /// Constructs an algorithm.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_running: false,
        }
    }

    /// Returns the algorithm name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts the algorithm.
    pub fn start(&mut self) {
        self.is_running = true;
    }

    /// Stops the algorithm.
    pub fn stop(&mut self) {
        self.is_running = false;
    }

    /// Returns whether the algorithm is running.
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let algo = MessageAlgorithm::new("TestAlgo");
        assert_eq!(algo.name(), "TestAlgo");
        assert!(!algo.is_running());
    }

    #[test]
    fn test_start_stop() {
        let mut algo = MessageAlgorithm::new("TestAlgo");
        algo.start();
        assert!(algo.is_running());
        algo.stop();
        assert!(!algo.is_running());
    }
}
