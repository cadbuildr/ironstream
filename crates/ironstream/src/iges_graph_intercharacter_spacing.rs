// FILE: iges_graph_intercharacter_spacing.rs
// occt: IGESGraph_Uintercharacterspacing

pub struct UintercharacterUspacing {
    entity_type: i32,
}

impl UintercharacterUspacing {
    pub fn new() -> Self {
        UintercharacterUspacing { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UintercharacterUspacing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UintercharacterUspacing::new();
    }
}
