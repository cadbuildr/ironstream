// FILE: iges_graph_line_font_predefined.rs
// occt: IGESGraph_Ulinefontpredefined

pub struct UlineUfontUpredefined {
    entity_type: i32,
}

impl UlineUfontUpredefined {
    pub fn new() -> Self {
        UlineUfontUpredefined { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UlineUfontUpredefined {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UlineUfontUpredefined::new();
    }
}
