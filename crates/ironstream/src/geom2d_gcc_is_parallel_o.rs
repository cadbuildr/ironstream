// FILE: geom2d_gcc_is_parallel_o.rs
// occt: Geom2dGcc_IsParallel

/// Test for parallel geometries.
pub struct IsParallel {
    result: bool,
}

impl IsParallel {
    pub fn new() -> Self {
        IsParallel { result: false }
    }

    pub fn is_parallel(&self) -> bool {
        self.result
    }

    pub fn set_parallel(&mut self, value: bool) {
        self.result = value;
    }
}

impl Default for IsParallel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let test = IsParallel::new();
        assert!(!test.is_parallel());
    }

    #[test]
    fn test_set_parallel() {
        let mut test = IsParallel::new();
        test.set_parallel(true);
        assert!(test.is_parallel());
    }
}
