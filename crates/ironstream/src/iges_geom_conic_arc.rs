// FILE: iges_geom_conic_arc.rs
// occt: IGESGeom_Uconicarc

pub struct UconicUarc {
    entity_type: i32,
}

impl UconicUarc {
    pub fn new() -> Self {
        UconicUarc { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UconicUarc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UconicUarc::new();
    }
}
