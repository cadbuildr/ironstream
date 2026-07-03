// FILE: local_analysis_o.rs
// occt: LocalAnalysis

/// Local curve and surface analysis utilities
pub struct LocalAnalysis;

impl LocalAnalysis {
    /// Create new analysis
    pub fn new() -> Self {
        LocalAnalysis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _analysis = LocalAnalysis::new();
    }
}
