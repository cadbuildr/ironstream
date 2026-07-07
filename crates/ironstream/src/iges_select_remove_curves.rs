// FILE: iges_select_remove_curves.rs
// occt: IGESSelect_RemoveCurves

pub struct IGESSelectRemoveCurves;

impl IGESSelectRemoveCurves {
    pub fn new() -> Self {
        IGESSelectRemoveCurves
    }
}

impl Default for IGESSelectRemoveCurves {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectRemoveCurves::new();
    }
}
