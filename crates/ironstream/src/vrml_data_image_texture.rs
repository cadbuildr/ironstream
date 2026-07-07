// FILE: vrml_data_image_texture.rs
// occt: VrmlData_ImageTexture

#[derive(Clone, Debug)]
pub struct VrmlDataImageTexture {
    url: String,
}

impl VrmlDataImageTexture {
    pub fn new(url: &str) -> Self {
        VrmlDataImageTexture {
            url: url.to_string(),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let t = VrmlDataImageTexture::new("texture.jpg");
        assert_eq!(t.url(), "texture.jpg");
    }
}
