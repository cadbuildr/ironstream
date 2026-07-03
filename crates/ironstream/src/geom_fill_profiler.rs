// FILE: geom_fill_profiler.rs
// occt: GeomFill_Profiler

/// Profile management for sweep surfaces
pub struct Profiler;

impl Profiler {
    pub fn new() -> Self {
        Profiler
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _profiler = Profiler::new();
    }

    #[test]
    fn test_default() {
        let _profiler = Profiler::default();
    }
}
