// FILE: iges_graph_line_font_def_pattern.rs
// occt-ref: IGESGraph_Ulinefontdefpattern

pub struct UlineUfontUdefUpattern {
    entity_type: i32,
}

impl UlineUfontUdefUpattern {
    pub fn new() -> Self {
        UlineUfontUdefUpattern { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UlineUfontUdefUpattern {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UlineUfontUdefUpattern::new();
    }
}
