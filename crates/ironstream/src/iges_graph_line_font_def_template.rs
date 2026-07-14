// FILE: iges_graph_line_font_def_template.rs
// occt-ref: IGESGraph_Ulinefontdeftemplate

pub struct UlineUfontUdefUtemplate {
    entity_type: i32,
}

impl UlineUfontUdefUtemplate {
    pub fn new() -> Self {
        UlineUfontUdefUtemplate { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UlineUfontUdefUtemplate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UlineUfontUdefUtemplate::new();
    }
}
