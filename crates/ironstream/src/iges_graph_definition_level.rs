// FILE: iges_graph_definition_level.rs
// occt: IGESGraph_Udefinitionlevel

pub struct UdefinitionUlevel {
    entity_type: i32,
}

impl UdefinitionUlevel {
    pub fn new() -> Self {
        UdefinitionUlevel { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UdefinitionUlevel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UdefinitionUlevel::new();
    }
}
