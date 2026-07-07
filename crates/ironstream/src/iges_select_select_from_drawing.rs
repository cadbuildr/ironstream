// FILE: iges_select_select_from_drawing.rs
// occt: IGESSelect_SelectFromDrawing

pub struct IGESSelectSelectFromDrawing;

impl IGESSelectSelectFromDrawing {
    pub fn new() -> Self {
        IGESSelectSelectFromDrawing
    }
}

impl Default for IGESSelectSelectFromDrawing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectSelectFromDrawing::new();
    }
}
