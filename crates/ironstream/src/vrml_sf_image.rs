// FILE: vrml_sf_image.rs
// occt: Vrml_SFImage
//
// Faithful port of OCCT Vrml_SFImage (DataExchange/TKDEVRML/Vrml/
// Vrml_SFImage.hxx/.cxx): a single-field image type for VRML.
// Contains width, height, components per pixel, and pixel data.

/// Port of Vrml_SFImage.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlSfImage {
    width: i32,
    height: i32,
    num_components: i32,
    pixels: Vec<u8>,
}

impl VrmlSfImage {
    /// Vrml_SFImage with default values (0x0, 1 component, empty pixels).
    pub fn new() -> Self {
        VrmlSfImage {
            width: 0,
            height: 0,
            num_components: 1,
            pixels: Vec::new(),
        }
    }

    /// Vrml_SFImage(aWidth, aHeight, aNumComponents).
    pub fn with_dimensions(a_width: i32, a_height: i32, a_num_components: i32) -> Self {
        VrmlSfImage {
            width: a_width,
            height: a_height,
            num_components: a_num_components,
            pixels: Vec::new(),
        }
    }

    pub fn set_width(&mut self, a_width: i32) {
        self.width = a_width;
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn set_height(&mut self, a_height: i32) {
        self.height = a_height;
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn set_components(&mut self, a_num_components: i32) {
        self.num_components = a_num_components;
    }

    pub fn components(&self) -> i32 {
        self.num_components
    }

    pub fn set_pixels(&mut self, a_pixels: Vec<u8>) {
        self.pixels = a_pixels;
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    /// Total number of pixel values.
    pub fn pixel_count(&self) -> usize {
        (self.width as usize) * (self.height as usize) * (self.num_components as usize)
    }
}

impl Default for VrmlSfImage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_image() {
        let img = VrmlSfImage::new();
        assert_eq!(img.width(), 0);
        assert_eq!(img.height(), 0);
        assert_eq!(img.components(), 1);
        assert_eq!(img.pixels().len(), 0);
    }

    #[test]
    fn with_dimensions() {
        let img = VrmlSfImage::with_dimensions(256, 128, 4);
        assert_eq!(img.width(), 256);
        assert_eq!(img.height(), 128);
        assert_eq!(img.components(), 4);
    }

    #[test]
    fn setters() {
        let mut img = VrmlSfImage::new();
        img.set_width(64);
        img.set_height(32);
        img.set_components(3);
        assert_eq!(img.width(), 64);
        assert_eq!(img.height(), 32);
        assert_eq!(img.components(), 3);
    }

    #[test]
    fn pixel_count() {
        let img = VrmlSfImage::with_dimensions(2, 3, 4);
        assert_eq!(img.pixel_count(), 2 * 3 * 4);
    }

    #[test]
    fn set_and_get_pixels() {
        let mut img = VrmlSfImage::with_dimensions(2, 2, 3);
        let pixels = vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128];
        img.set_pixels(pixels.clone());
        assert_eq!(img.pixels(), pixels.as_slice());
    }
}
