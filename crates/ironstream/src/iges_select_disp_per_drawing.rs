// FILE: iges_select_disp_per_drawing.rs
// occt: IGESSelect_DispPerDrawing

pub struct IGESSelectDispPerDrawing;

impl IGESSelectDispPerDrawing {
    pub fn new() -> Self {
        IGESSelectDispPerDrawing
    }
}

impl Default for IGESSelectDispPerDrawing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectDispPerDrawing::new();
    }
}
