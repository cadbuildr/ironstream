// FILE: iges_select_select_drawing_from.rs
// occt: IGESSelect_SelectDrawingFrom

pub struct IGESSelectSelectDrawingFrom;

impl IGESSelectSelectDrawingFrom {
    pub fn new() -> Self {
        IGESSelectSelectDrawingFrom
    }
}

impl Default for IGESSelectSelectDrawingFrom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectSelectDrawingFrom::new();
    }
}
