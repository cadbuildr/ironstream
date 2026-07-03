// FILE: top_ope_b_rep_tool_box_sort.rs
// occt: TopOpeBRepTool_BoxSort

/// BoxSort stub implementation.
pub struct BoxSort;

impl BoxSort {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        BoxSort
    }
}

impl Default for BoxSort {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BoxSort::new();
    }

    #[test]
    fn test_default() {
        let _ = BoxSort::default();
    }
}
