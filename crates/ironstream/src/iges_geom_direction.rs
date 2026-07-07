// FILE: iges_geom_direction.rs
// occt: IGESGeom_Udirection

pub struct Udirection {
    entity_type: i32,
}

impl Udirection {
    pub fn new() -> Self {
        Udirection { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for Udirection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = Udirection::new();
    }
}
