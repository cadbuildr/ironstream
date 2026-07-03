// FILE: b_rep_preview_api_make_box.rs
// occt: BRepPreviewAPI_MakeBox

/// Builds a valid box or preview depending on point values.
#[derive(Debug, Clone)]
pub struct BRepPreviewApiMakeBox;

impl BRepPreviewApiMakeBox {
    pub fn new() -> Self {
        BRepPreviewApiMakeBox
    }
}

impl Default for BRepPreviewApiMakeBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_box() {
        let _mb = BRepPreviewApiMakeBox::new();
    }
}
