// FILE: iges_geom_bounded_surface.rs
// occt: IGESGeom_Uboundedsurface

pub struct UboundedUsurface {
    entity_type: i32,
}

impl UboundedUsurface {
    pub fn new() -> Self {
        UboundedUsurface { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UboundedUsurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UboundedUsurface::new();
    }
}
