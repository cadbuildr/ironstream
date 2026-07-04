// FILE: osd_parallel.rs
// occt: OSD_Parallel

/// Parallel execution utilities.
pub struct Parallel;

impl Parallel {
    pub fn new() -> Self {
        Self
    }

    pub fn num_threads() -> usize {
        num_cpus::get()
    }
}

impl Default for Parallel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_threads() {
        let n = Parallel::num_threads();
        assert!(n > 0);
    }
}
