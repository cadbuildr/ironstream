// FILE: graphic3d_texture1_dmanual.rs
// occt: Graphic3d_Texture1Dmanual

/// Manual 1D texture that requires explicit texture coordinates on facets.
///
/// This class provides a 1D texture that must have texture coordinates
/// explicitly specified by the user on each facet where it is applied.
/// Unlike automatic texturing, the user has full control over how the
/// texture is mapped to the geometry.
#[derive(Debug, Clone)]
pub struct Graphic3dTexture1Dmanual {
    /// Internal texture representation (simplified for this port)
    filename: Option<String>,
    name: u32, // Simplified representation of predefined name
}

impl Graphic3dTexture1Dmanual {
    /// Creates a manual 1D texture from a file name.
    pub fn from_file(filename: impl Into<String>) -> Self {
        Self {
            filename: Some(filename.into()),
            name: 0,
        }
    }

    /// Creates a manual 1D texture from a predefined texture name.
    /// The name parameter represents indices to predefined textures.
    pub fn from_name(name: u32) -> Self {
        Self {
            filename: None,
            name,
        }
    }

    /// Returns the file name if this texture was created from a file.
    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }

    /// Returns the predefined name index.
    pub fn name(&self) -> u32 {
        self.name
    }

    /// Checks if this is a file-based texture.
    pub fn is_file_based(&self) -> bool {
        self.filename.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_file() {
        let texture = Graphic3dTexture1Dmanual::from_file("manual_texture.jpg");
        assert!(texture.is_file_based());
        assert_eq!(texture.filename(), Some("manual_texture.jpg"));
    }

    #[test]
    fn test_create_from_predefined_name() {
        let texture = Graphic3dTexture1Dmanual::from_name(2);
        assert!(!texture.is_file_based());
        assert_eq!(texture.name(), 2);
        assert_eq!(texture.filename(), None);
    }

    #[test]
    fn test_clone_file_based_texture() {
        let texture = Graphic3dTexture1Dmanual::from_file("test.bmp");
        let cloned = texture.clone();
        assert_eq!(cloned.filename(), texture.filename());
        assert_eq!(cloned.name(), texture.name());
    }

    #[test]
    fn test_clone_name_based_texture() {
        let texture = Graphic3dTexture1Dmanual::from_name(5);
        let cloned = texture.clone();
        assert_eq!(cloned.name(), 5);
        assert!(!cloned.is_file_based());
    }

    #[test]
    fn test_file_based_flag() {
        let file_texture = Graphic3dTexture1Dmanual::from_file("test.png");
        let name_texture = Graphic3dTexture1Dmanual::from_name(1);

        assert!(file_texture.is_file_based());
        assert!(!name_texture.is_file_based());
    }
}
