// FILE: top_ope_b_rep_ds_reducer.rs
// occt: TopOpeBRepDS_Reducer

/// Tool for reducing data structure
#[derive(Debug, Clone)]
pub struct Reducer {
    /// DS reference
    ds_id: Option<usize>,
    /// Reduction map
    reductions: Vec<(usize, usize)>,
}

impl Reducer {
    /// Create new Reducer
    pub fn new() -> Self {
        Reducer {
            ds_id: None,
            reductions: Vec::new(),
        }
    }

    /// Create with data structure
    pub fn with_ds(ds_id: usize) -> Self {
        Reducer {
            ds_id: Some(ds_id),
            reductions: Vec::new(),
        }
    }

    /// Reduce (placeholder)
    pub fn reduce(&mut self) {
        // Placeholder for actual reduce logic
    }

    /// Add reduction mapping
    pub fn add_reduction(&mut self, old_id: usize, new_id: usize) {
        self.reductions.push((old_id, new_id));
    }

    /// Get reductions
    pub fn reductions(&self) -> &[(usize, usize)] {
        &self.reductions
    }
}

impl Default for Reducer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reducer_new() {
        let r = Reducer::new();
        assert!(r.ds_id.is_none());
    }

    #[test]
    fn test_reducer_reductions() {
        let mut r = Reducer::new();
        r.add_reduction(10, 5);
        r.add_reduction(20, 8);
        assert_eq!(r.reductions().len(), 2);
        assert_eq!(r.reductions()[0], (10, 5));
    }
}
