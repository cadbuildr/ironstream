// FILE: iges_geom_boundary.rs
// occt: IGESGeom_Boundary

pub struct Boundary {
    entity_type: i32,
}

impl Boundary {
    pub fn new() -> Self {
        Boundary { entity_type: 141 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for Boundary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let boundary = Boundary::new();
        assert_eq!(boundary.entity_type(), 141);
    }
}
