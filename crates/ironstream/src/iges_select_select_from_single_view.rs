// FILE: iges_select_select_from_single_view.rs
// occt: IGESSelect_SelectFromSingleView

pub struct IGESSelectSelectFromSingleView;

impl IGESSelectSelectFromSingleView {
    pub fn new() -> Self {
        IGESSelectSelectFromSingleView
    }
}

impl Default for IGESSelectSelectFromSingleView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectSelectFromSingleView::new();
    }
}
