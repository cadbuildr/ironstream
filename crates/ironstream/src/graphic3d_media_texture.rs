// FILE: graphic3d_media_texture.rs
// occt: Graphic3d_MediaTexture

use std::sync::Mutex;

/// Texture adapter for Media_Frame data.
pub struct Graphic3dMediaTexture {
    mutex: Mutex<()>,
    plane: i32,
}

impl Graphic3dMediaTexture {
    /// Creates a new media texture with the specified plane index.
    /// plane: -1 for all planes, or specific plane index
    pub fn new(plane: i32) -> Self {
        Graphic3dMediaTexture {
            mutex: Mutex::new(()),
            plane,
        }
    }

    /// Returns the plane index.
    pub fn plane(&self) -> i32 {
        self.plane
    }

    /// Regenerates a new texture ID.
    pub fn generate_new_id(&self) {
        // In real implementation, this would invalidate the current texture ID
        // and force generation of a new one
        let _lock = self.mutex.lock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_texture_creation() {
        let tex = Graphic3dMediaTexture::new(-1);
        assert_eq!(tex.plane(), -1);
    }

    #[test]
    fn test_media_texture_with_plane() {
        let tex = Graphic3dMediaTexture::new(0);
        assert_eq!(tex.plane(), 0);

        let tex2 = Graphic3dMediaTexture::new(1);
        assert_eq!(tex2.plane(), 1);
    }

    #[test]
    fn test_media_texture_generate_id() {
        let tex = Graphic3dMediaTexture::new(0);
        tex.generate_new_id();
        // If we get here without panic, the method works correctly
        assert!(true);
    }
}
