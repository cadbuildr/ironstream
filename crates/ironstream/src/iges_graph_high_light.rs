// FILE: iges_graph_high_light.rs
// occt: IGESGraph_Uhighlight

pub struct UhighUlight {
    entity_type: i32,
}

impl UhighUlight {
    pub fn new() -> Self {
        UhighUlight { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UhighUlight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UhighUlight::new();
    }
}
