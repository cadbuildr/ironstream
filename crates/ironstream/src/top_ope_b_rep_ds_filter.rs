// FILE: top_ope_b_rep_ds_filter.rs
// occt: TopOpeBRepDS_Filter

/// Tool for filtering interferences
#[derive(Debug, Clone)]
pub struct Filter {
    /// DS reference
    ds_id: Option<usize>,
    /// Filtered indices
    filtered: Vec<usize>,
}

impl Filter {
    /// Create new Filter
    pub fn new() -> Self {
        Filter {
            ds_id: None,
            filtered: Vec::new(),
        }
    }

    /// Create with data structure
    pub fn with_ds(ds_id: usize) -> Self {
        Filter {
            ds_id: Some(ds_id),
            filtered: Vec::new(),
        }
    }

    /// Filter process (placeholder)
    pub fn filter(&mut self) {
        // Placeholder for actual filtering logic
    }

    /// Get filtered list
    pub fn filtered(&self) -> &[usize] {
        &self.filtered
    }

    /// Add to filtered
    pub fn add_filtered(&mut self, i: usize) {
        self.filtered.push(i);
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_new() {
        let f = Filter::new();
        assert!(f.ds_id.is_none());
        assert_eq!(f.filtered().len(), 0);
    }

    #[test]
    fn test_filter_with_ds() {
        let f = Filter::with_ds(42);
        assert_eq!(f.ds_id, Some(42));
    }

    #[test]
    fn test_filter_add() {
        let mut f = Filter::new();
        f.add_filtered(1);
        f.add_filtered(2);
        assert_eq!(f.filtered().len(), 2);
    }
}
