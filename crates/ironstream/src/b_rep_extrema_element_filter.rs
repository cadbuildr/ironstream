// FILE: b_rep_extrema_element_filter.rs
// occt: BRepExtrema_ElementFilter

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FilterResult {
    NoCheck,
    Overlap,
    DoCheck,
}

/// Filtering tool for mesh element detection
pub trait ElementFilter {
    fn pre_check_elements(&self, idx1: i32, idx2: i32) -> FilterResult {
        let _ = (idx1, idx2);
        FilterResult::DoCheck
    }
}

pub struct DefaultElementFilter;

impl ElementFilter for DefaultElementFilter {
    fn pre_check_elements(&self, _idx1: i32, _idx2: i32) -> FilterResult {
        FilterResult::DoCheck
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_result() {
        assert_eq!(FilterResult::NoCheck, FilterResult::NoCheck);
        assert_eq!(FilterResult::Overlap, FilterResult::Overlap);
        assert_eq!(FilterResult::DoCheck, FilterResult::DoCheck);
    }

    #[test]
    fn test_default_filter() {
        let filter = DefaultElementFilter;
        assert_eq!(filter.pre_check_elements(0, 1), FilterResult::DoCheck);
    }
}
