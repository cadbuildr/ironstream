// FILE: iges_select_select_faces.rs
// occt: IGESSelect_SelectFaces

pub struct IGESSelectSelectFaces;

impl IGESSelectSelectFaces {
    pub fn new() -> Self {
        IGESSelectSelectFaces
    }
}

impl Default for IGESSelectSelectFaces {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectSelectFaces::new();
    }
}
