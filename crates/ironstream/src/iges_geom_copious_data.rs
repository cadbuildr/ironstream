// FILE: iges_geom_copious_data.rs
// occt: IGESGeom_Ucopiousdata

pub struct UcopiousUdata {
    entity_type: i32,
}

impl UcopiousUdata {
    pub fn new() -> Self {
        UcopiousUdata { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UcopiousUdata {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UcopiousUdata::new();
    }
}
