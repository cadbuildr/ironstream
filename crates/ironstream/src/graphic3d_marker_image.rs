// FILE: graphic3d_marker_image.rs
// occt: Graphic3d_MarkerImage

/// Stores bitmaps and images for markers rendering.
/// Converts between bitmap textures and PixMap format.
pub struct Graphic3dMarkerImage {
    image_id: String,
    image_alpha_id: String,
    width: i32,
    height: i32,
}

impl Graphic3dMarkerImage {
    /// Creates a new marker image from width and height.
    pub fn new(width: i32, height: i32) -> Self {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);

        Graphic3dMarkerImage {
            image_id: format!("marker_image_{}", id),
            image_alpha_id: format!("marker_alpha_{}", id),
            width,
            height,
        }
    }

    /// Returns the unique image ID.
    pub fn get_image_id(&self) -> &str {
        &self.image_id
    }

    /// Returns the unique alpha image ID.
    pub fn get_image_alpha_id(&self) -> &str {
        &self.image_alpha_id
    }

    /// Returns texture dimensions.
    pub fn get_texture_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_image_creation() {
        let img = Graphic3dMarkerImage::new(64, 64);
        let (w, h) = img.get_texture_size();
        assert_eq!(w, 64);
        assert_eq!(h, 64);
        assert!(!img.get_image_id().is_empty());
        assert!(!img.get_image_alpha_id().is_empty());
    }

    #[test]
    fn test_marker_image_unique_ids() {
        let img1 = Graphic3dMarkerImage::new(32, 32);
        let img2 = Graphic3dMarkerImage::new(32, 32);
        assert_ne!(img1.get_image_id(), img2.get_image_id());
    }
}
