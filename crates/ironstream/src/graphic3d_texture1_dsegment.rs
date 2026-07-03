// FILE: graphic3d_texture1_dsegment.rs
// occt: Graphic3d_Texture1Dsegment

/// A 1D texture applicable along a segment of a surface.
///
/// This class provides a 1D texture that is stretched across facets using
/// explicit segment bounds. The segment defines the 3D interval along which
/// the texture is applied, allowing precise control over texture stretching.
#[derive(Debug, Clone)]
pub struct Graphic3dTexture1Dsegment {
    /// File name if loaded from file
    filename: Option<String>,
    /// Predefined texture name
    name: u32,
    /// Segment bounds: start point (x1, y1, z1)
    x1: f32,
    y1: f32,
    z1: f32,
    /// Segment bounds: end point (x2, y2, z2)
    x2: f32,
    y2: f32,
    z2: f32,
}

impl Graphic3dTexture1Dsegment {
    /// Creates a texture from a file name with default segment bounds.
    pub fn from_file(filename: impl Into<String>) -> Self {
        Self {
            filename: Some(filename.into()),
            name: 0,
            x1: 0.0,
            y1: 0.0,
            z1: 0.0,
            x2: 0.0,
            y2: 0.0,
            z2: 1.0,
        }
    }

    /// Creates a texture from a predefined texture name with default segment bounds.
    pub fn from_name(name: u32) -> Self {
        Self {
            filename: None,
            name,
            x1: 0.0,
            y1: 0.0,
            z1: 0.0,
            x2: 0.0,
            y2: 0.0,
            z2: 1.0,
        }
    }

    /// Sets the texture application bounds as a segment in 3D space.
    /// Defines how the texture is stretched across facets.
    pub fn set_segment(&mut self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) {
        self.x1 = x1;
        self.y1 = y1;
        self.z1 = z1;
        self.x2 = x2;
        self.y2 = y2;
        self.z2 = z2;
    }

    /// Returns the current segment bounds.
    pub fn segment(&self) -> (f32, f32, f32, f32, f32, f32) {
        (self.x1, self.y1, self.z1, self.x2, self.y2, self.z2)
    }

    /// Returns the filename if this texture was created from a file.
    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }

    /// Returns the predefined texture name index.
    pub fn name(&self) -> u32 {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_file() {
        let texture = Graphic3dTexture1Dsegment::from_file("segment_texture.jpg");
        assert_eq!(texture.filename(), Some("segment_texture.jpg"));
        let (x1, y1, z1, x2, y2, z2) = texture.segment();
        assert_eq!(x1, 0.0);
        assert_eq!(y1, 0.0);
        assert_eq!(z1, 0.0);
        assert_eq!(x2, 0.0);
        assert_eq!(y2, 0.0);
        assert_eq!(z2, 1.0);
    }

    #[test]
    fn test_create_from_name() {
        let texture = Graphic3dTexture1Dsegment::from_name(3);
        assert_eq!(texture.name(), 3);
        assert_eq!(texture.filename(), None);
    }

    #[test]
    fn test_set_segment_default() {
        let texture = Graphic3dTexture1Dsegment::from_file("test.png");
        let (x1, y1, z1, x2, y2, z2) = texture.segment();
        assert_eq!((x1, y1, z1), (0.0, 0.0, 0.0));
        assert_eq!((x2, y2, z2), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_set_segment_custom() {
        let mut texture = Graphic3dTexture1Dsegment::from_file("test.jpg");
        texture.set_segment(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);

        let (x1, y1, z1, x2, y2, z2) = texture.segment();
        assert_eq!(x1, 1.0);
        assert_eq!(y1, 2.0);
        assert_eq!(z1, 3.0);
        assert_eq!(x2, 4.0);
        assert_eq!(y2, 5.0);
        assert_eq!(z2, 6.0);
    }

    #[test]
    fn test_set_segment_negative_values() {
        let mut texture = Graphic3dTexture1Dsegment::from_name(1);
        texture.set_segment(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0);

        let (x1, y1, z1, x2, y2, z2) = texture.segment();
        assert_eq!(x1, -1.0);
        assert_eq!(y1, -2.0);
        assert_eq!(z1, -3.0);
        assert_eq!(x2, -4.0);
        assert_eq!(y2, -5.0);
        assert_eq!(z2, -6.0);
    }

    #[test]
    fn test_clone_texture() {
        let mut texture = Graphic3dTexture1Dsegment::from_file("clone_test.jpg");
        texture.set_segment(1.5, 2.5, 3.5, 4.5, 5.5, 6.5);

        let cloned = texture.clone();
        assert_eq!(cloned.filename(), texture.filename());
        assert_eq!(cloned.segment(), texture.segment());
    }

    #[test]
    fn test_set_segment_multiple_times() {
        let mut texture = Graphic3dTexture1Dsegment::from_name(2);

        texture.set_segment(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let (x1, y1, z1, x2, y2, z2) = texture.segment();
        assert_eq!((x1, y1, z1, x2, y2, z2), (0.0, 0.0, 0.0, 1.0, 1.0, 1.0));

        texture.set_segment(10.0, 20.0, 30.0, 40.0, 50.0, 60.0);
        let (x1, y1, z1, x2, y2, z2) = texture.segment();
        assert_eq!((x1, y1, z1, x2, y2, z2), (10.0, 20.0, 30.0, 40.0, 50.0, 60.0));
    }
}
