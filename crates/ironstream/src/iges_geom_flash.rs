// FILE: iges_geom_flash.rs
// occt-ref: IGESGeom_Uflash

pub struct Uflash {
    entity_type: i32,
}

impl Uflash {
    pub fn new() -> Self {
        Uflash { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for Uflash {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = Uflash::new();
    }
}
