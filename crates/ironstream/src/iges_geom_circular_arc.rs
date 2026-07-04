// FILE: iges_geom_circular_arc.rs
// occt: IGESGeom_Ucirculararc

pub struct UcircularUarc {
    entity_type: i32,
}

impl UcircularUarc {
    pub fn new() -> Self {
        UcircularUarc { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UcircularUarc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UcircularUarc::new();
    }
}
