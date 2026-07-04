// FILE: iges_graph_general_module.rs
// occt: IGESGraph_Ugeneralmodule

pub struct UgeneralUmodule {
    entity_type: i32,
}

impl UgeneralUmodule {
    pub fn new() -> Self {
        UgeneralUmodule { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UgeneralUmodule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UgeneralUmodule::new();
    }
}
