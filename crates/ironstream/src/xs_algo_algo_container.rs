// FILE: xs_algo_algo_container.rs
// occt: XSAlgo_AlgoContainer

/// Container for shape processing algorithms.
/// Holds references to various algorithmic processors used in the exchange framework.
#[derive(Clone, Debug)]
pub struct XSAlgoAlgoContainer {
    /// Container identifier
    container_id: u32,
    /// Processing mode
    mode: u32,
}

impl XSAlgoAlgoContainer {
    /// Creates a new algorithm container.
    pub fn new() -> Self {
        Self {
            container_id: 1,
            mode: 0,
        }
    }

    /// Returns the container ID.
    pub fn id(&self) -> u32 {
        self.container_id
    }

    /// Returns the processing mode.
    pub fn mode(&self) -> u32 {
        self.mode
    }

    /// Sets the processing mode.
    pub fn set_mode(&mut self, mode: u32) {
        self.mode = mode;
    }
}

impl Default for XSAlgoAlgoContainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let container = XSAlgoAlgoContainer::new();
        assert_eq!(container.id(), 1);
        assert_eq!(container.mode(), 0);
    }

    #[test]
    fn test_set_mode() {
        let mut container = XSAlgoAlgoContainer::new();
        container.set_mode(2);
        assert_eq!(container.mode(), 2);
    }
}
