// FILE: graphic3d_texture1_d.rs
// occt: Graphic3d_Texture1D

/// Abstract base class for managing 1D textures.
///
/// This class represents a 1D texture that can be applied along an edge or
/// segment of a surface. Subclasses provide concrete implementations for
/// manual texturing and segment-based texturing.
#[derive(Debug, Clone)]
pub struct Graphic3dTexture1D {
    /// The name of the predefined texture, if applicable
    name: Graphic3dNameOfTexture1D,
    /// File name if the texture is loaded from a file
    filename: Option<String>,
}

/// Enumeration of predefined 1D texture names
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Graphic3dNameOfTexture1D {
    /// Unknown or not a predefined texture
    Unknown,
    /// Wood texture
    Wood,
    /// Marble texture
    Marble,
    /// Metal texture
    Metal,
}

impl Graphic3dTexture1D {
    /// Creates a new 1D texture from a file name.
    pub fn from_file(filename: impl Into<String>) -> Self {
        Self {
            name: Graphic3dNameOfTexture1D::Unknown,
            filename: Some(filename.into()),
        }
    }

    /// Creates a new 1D texture from a predefined texture name.
    pub fn from_name(name: Graphic3dNameOfTexture1D) -> Self {
        Self {
            name,
            filename: None,
        }
    }

    /// Returns the name of the texture or Unknown if loaded from file.
    pub fn name(&self) -> Graphic3dNameOfTexture1D {
        self.name
    }

    /// Returns the number of predefined 1D textures.
    pub fn number_of_textures() -> usize {
        3 // Wood, Marble, Metal
    }

    /// Returns the name of the predefined texture at the given rank (1-indexed).
    pub fn texture_name(rank: usize) -> Option<String> {
        match rank {
            1 => Some("Wood".to_string()),
            2 => Some("Marble".to_string()),
            3 => Some("Metal".to_string()),
            _ => None,
        }
    }

    /// Returns the file name if this texture was loaded from a file.
    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_file() {
        let texture = Graphic3dTexture1D::from_file("texture.jpg");
        assert_eq!(texture.name(), Graphic3dNameOfTexture1D::Unknown);
        assert_eq!(texture.filename(), Some("texture.jpg"));
    }

    #[test]
    fn test_create_from_name() {
        let texture = Graphic3dTexture1D::from_name(Graphic3dNameOfTexture1D::Wood);
        assert_eq!(texture.name(), Graphic3dNameOfTexture1D::Wood);
        assert_eq!(texture.filename(), None);
    }

    #[test]
    fn test_number_of_textures() {
        assert_eq!(Graphic3dTexture1D::number_of_textures(), 3);
    }

    #[test]
    fn test_texture_name_valid_ranks() {
        assert_eq!(
            Graphic3dTexture1D::texture_name(1),
            Some("Wood".to_string())
        );
        assert_eq!(
            Graphic3dTexture1D::texture_name(2),
            Some("Marble".to_string())
        );
        assert_eq!(
            Graphic3dTexture1D::texture_name(3),
            Some("Metal".to_string())
        );
    }

    #[test]
    fn test_texture_name_invalid_rank() {
        assert_eq!(Graphic3dTexture1D::texture_name(0), None);
        assert_eq!(Graphic3dTexture1D::texture_name(4), None);
        assert_eq!(Graphic3dTexture1D::texture_name(100), None);
    }

    #[test]
    fn test_clone_texture() {
        let texture = Graphic3dTexture1D::from_file("test.jpg");
        let cloned = texture.clone();
        assert_eq!(cloned.name(), texture.name());
        assert_eq!(cloned.filename(), texture.filename());
    }
}
