// FILE: open_gl_aspects_sprite.rs
// occt: OpenGl_AspectsSprite

/// OpenGl_AspectsSprite manages sprite aspects.
pub struct OpenGlAspectsSprite {
    sprite_id: u32,
    size: f32,
}

impl OpenGlAspectsSprite {
    pub fn new(sprite_id: u32, size: f32) -> Self {
        OpenGlAspectsSprite { sprite_id, size }
    }

    pub fn sprite_id(&self) -> u32 {
        self.sprite_id
    }

    pub fn size(&self) -> f32 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_creation() {
        let sprite = OpenGlAspectsSprite::new(1, 10.0);
        assert_eq!(sprite.sprite_id(), 1);
        assert_eq!(sprite.size(), 10.0);
    }
}
