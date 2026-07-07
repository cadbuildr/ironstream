// FILE: open_gl_point_sprite.rs
// occt: OpenGl_PointSprite

/// Point sprite texture for point-based rendering.
#[derive(Debug, Clone)]
pub struct OpenGlPointSprite;

impl OpenGlPointSprite {
    pub fn new() -> Self {
        OpenGlPointSprite
    }
}

impl Default for OpenGlPointSprite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_sprite() {
        let _sprite = OpenGlPointSprite::new();
    }
}
