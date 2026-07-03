// FILE: ais_textured_shape.rs
// occt: AIS_TexturedShape

#[derive(Clone, Debug)]
pub struct AisTexturedShape {
    pub object_id: u32,
    pub texture_file: String,
    pub is_visible: bool,
}

impl AisTexturedShape {
    pub fn new(object_id: u32) -> Self {
        Self { object_id, texture_file: String::new(), is_visible: true }
    }
    pub fn set_texture(&mut self, path: &str) { self.texture_file = path.to_string(); }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

impl Default for AisTexturedShape {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let ts = AisTexturedShape::new(1);
        assert_eq!(ts.object_id, 1);
        assert!(ts.is_visible);
    }
}
