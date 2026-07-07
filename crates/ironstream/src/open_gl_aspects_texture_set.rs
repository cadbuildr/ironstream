// FILE: open_gl_aspects_texture_set.rs
// occt: OpenGl_AspectsTextureSet

/// OpenGl_AspectsTextureSet manages texture aspects.
pub struct OpenGlAspectsTextureSet {
    texture_ids: Vec<u32>,
}

impl OpenGlAspectsTextureSet {
    pub fn new() -> Self {
        OpenGlAspectsTextureSet {
            texture_ids: Vec::new(),
        }
    }

    pub fn add_texture(&mut self, id: u32) {
        self.texture_ids.push(id);
    }

    pub fn texture_count(&self) -> usize {
        self.texture_ids.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_set_creation() {
        let set = OpenGlAspectsTextureSet::new();
        assert_eq!(set.texture_count(), 0);
    }

    #[test]
    fn test_add_texture() {
        let mut set = OpenGlAspectsTextureSet::new();
        set.add_texture(1);
        assert_eq!(set.texture_count(), 1);
    }
}
