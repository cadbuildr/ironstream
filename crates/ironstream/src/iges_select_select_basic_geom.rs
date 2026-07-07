// FILE: iges_select_select_basic_geom.rs
// occt: IGESSelect_SelectBasicGeom

pub struct IGESSelectSelectBasicGeom;

impl IGESSelectSelectBasicGeom {
    pub fn new() -> Self {
        IGESSelectSelectBasicGeom
    }
}

impl Default for IGESSelectSelectBasicGeom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectSelectBasicGeom::new();
    }
}
