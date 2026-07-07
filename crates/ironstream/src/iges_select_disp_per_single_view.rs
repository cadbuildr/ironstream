// FILE: iges_select_disp_per_single_view.rs
// occt: IGESSelect_DispPerSingleView

pub struct IGESSelectDispPerSingleView;

impl IGESSelectDispPerSingleView {
    pub fn new() -> Self {
        IGESSelectDispPerSingleView
    }
}

impl Default for IGESSelectDispPerSingleView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectDispPerSingleView::new();
    }
}
