// FILE: graphic3d_texture2_dplane.rs
// occt: Graphic3d_Texture2Dplane

/// Type of the texture projection plane for both S and T texture coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameOfTexturePlane {
    XY,
    YZ,
    ZX,
    Unknown,
}

/// This class allows the management of a 2D texture defined from a plane equation
/// Use the SetXXX() methods for positioning the texture as you want.
pub struct Texture2Dplane {
    plane_name: NameOfTexturePlane,
    // Plane S equation coefficients: A, B, C, D
    plane_s: [f32; 4],
    // Plane T equation coefficients: A, B, C, D
    plane_t: [f32; 4],
    // Texture S translation
    translate_s: f32,
    // Texture T translation
    translate_t: f32,
    // Texture S scale
    scale_s: f32,
    // Texture T scale
    scale_t: f32,
    // Rotation angle in degrees
    rotation: f32,
}

impl Texture2Dplane {
    /// Creates a texture from a predefined texture name set.
    pub fn new() -> Self {
        Texture2Dplane {
            plane_name: NameOfTexturePlane::XY,
            // Default S plane: <1.0, 0.0, 0.0, 0.0>
            plane_s: [1.0, 0.0, 0.0, 0.0],
            // Default T plane: <0.0, 1.0, 0.0, 0.0>
            plane_t: [0.0, 1.0, 0.0, 0.0],
            translate_s: 0.0,
            translate_t: 0.0,
            scale_s: 1.0,
            scale_t: 1.0,
            rotation: 0.0,
        }
    }

    /// Defines the texture projection plane for texture coordinate S.
    /// default is <1.0, 0.0, 0.0, 0.0>
    pub fn set_plane_s(&mut self, a: f32, b: f32, c: f32, d: f32) {
        self.plane_s = [a, b, c, d];
        self.plane_name = NameOfTexturePlane::Unknown;
    }

    /// Defines the texture projection plane for texture coordinate T.
    /// default is <0.0, 1.0, 0.0, 0.0>
    pub fn set_plane_t(&mut self, a: f32, b: f32, c: f32, d: f32) {
        self.plane_t = [a, b, c, d];
        self.plane_name = NameOfTexturePlane::Unknown;
    }

    /// Defines the texture projection plane for both S and T texture coordinate.
    /// default is NOTP_XY meaning:
    /// <1.0, 0.0, 0.0, 0.0> for S and
    /// <0.0, 1.0, 0.0, 0.0> for T
    pub fn set_plane(&mut self, plane: NameOfTexturePlane) {
        self.plane_name = plane;
        match plane {
            NameOfTexturePlane::XY => {
                self.plane_s = [1.0, 0.0, 0.0, 0.0];
                self.plane_t = [0.0, 1.0, 0.0, 0.0];
            }
            NameOfTexturePlane::YZ => {
                self.plane_s = [0.0, 1.0, 0.0, 0.0];
                self.plane_t = [0.0, 0.0, 1.0, 0.0];
            }
            NameOfTexturePlane::ZX => {
                self.plane_s = [0.0, 0.0, 1.0, 0.0];
                self.plane_t = [1.0, 0.0, 0.0, 0.0];
            }
            NameOfTexturePlane::Unknown => {
                // Keep existing plane equations
            }
        }
    }

    /// Defines the texture scale for the S texture coordinate.
    /// default to 1.0
    pub fn set_scale_s(&mut self, val: f32) {
        self.scale_s = val;
    }

    /// Defines the texture scale for the T texture coordinate.
    /// default to 1.0
    pub fn set_scale_t(&mut self, val: f32) {
        self.scale_t = val;
    }

    /// Defines the texture translation for the S texture coordinate.
    /// default to 0.0
    pub fn set_translate_s(&mut self, val: f32) {
        self.translate_s = val;
    }

    /// Defines the texture translation for the T texture coordinate.
    /// default to 0.0
    pub fn set_translate_t(&mut self, val: f32) {
        self.translate_t = val;
    }

    /// Sets the rotation angle of the whole texture in degrees.
    /// default is 0.0
    pub fn set_rotation(&mut self, val: f32) {
        self.rotation = val;
    }

    /// Returns the current texture plane name or Unknown when the plane is user defined.
    pub fn plane(&self) -> NameOfTexturePlane {
        self.plane_name
    }

    /// Returns the current texture plane S equation.
    pub fn plane_s(&self) -> (f32, f32, f32, f32) {
        (self.plane_s[0], self.plane_s[1], self.plane_s[2], self.plane_s[3])
    }

    /// Returns the current texture plane T equation.
    pub fn plane_t(&self) -> (f32, f32, f32, f32) {
        (self.plane_t[0], self.plane_t[1], self.plane_t[2], self.plane_t[3])
    }

    /// Returns the current texture S translation value.
    pub fn translate_s(&self) -> f32 {
        self.translate_s
    }

    /// Returns the current texture T translation value.
    pub fn translate_t(&self) -> f32 {
        self.translate_t
    }

    /// Returns the current texture S scale value.
    pub fn scale_s(&self) -> f32 {
        self.scale_s
    }

    /// Returns the current texture T scale value.
    pub fn scale_t(&self) -> f32 {
        self.scale_t
    }

    /// Returns the current texture rotation angle in degrees.
    pub fn rotation(&self) -> f32 {
        self.rotation
    }
}

impl Default for Texture2Dplane {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture2dplane_defaults() {
        let tex = Texture2Dplane::new();
        assert_eq!(tex.plane(), NameOfTexturePlane::XY);
        assert_eq!(tex.plane_s(), (1.0, 0.0, 0.0, 0.0));
        assert_eq!(tex.plane_t(), (0.0, 1.0, 0.0, 0.0));
        assert_eq!(tex.translate_s(), 0.0);
        assert_eq!(tex.translate_t(), 0.0);
        assert_eq!(tex.scale_s(), 1.0);
        assert_eq!(tex.scale_t(), 1.0);
        assert_eq!(tex.rotation(), 0.0);
    }

    #[test]
    fn test_set_plane_s() {
        let mut tex = Texture2Dplane::new();
        tex.set_plane_s(2.0, 3.0, 4.0, 5.0);
        assert_eq!(tex.plane_s(), (2.0, 3.0, 4.0, 5.0));
        // Setting a custom plane should mark it as unknown
        assert_eq!(tex.plane(), NameOfTexturePlane::Unknown);
    }

    #[test]
    fn test_set_plane_t() {
        let mut tex = Texture2Dplane::new();
        tex.set_plane_t(1.5, 2.5, 3.5, 4.5);
        assert_eq!(tex.plane_t(), (1.5, 2.5, 3.5, 4.5));
        assert_eq!(tex.plane(), NameOfTexturePlane::Unknown);
    }

    #[test]
    fn test_set_plane_xy() {
        let mut tex = Texture2Dplane::new();
        tex.set_plane_s(5.0, 5.0, 5.0, 5.0);
        tex.set_plane_t(5.0, 5.0, 5.0, 5.0);

        tex.set_plane(NameOfTexturePlane::XY);
        assert_eq!(tex.plane(), NameOfTexturePlane::XY);
        assert_eq!(tex.plane_s(), (1.0, 0.0, 0.0, 0.0));
        assert_eq!(tex.plane_t(), (0.0, 1.0, 0.0, 0.0));
    }

    #[test]
    fn test_set_plane_yz() {
        let mut tex = Texture2Dplane::new();
        tex.set_plane(NameOfTexturePlane::YZ);
        assert_eq!(tex.plane(), NameOfTexturePlane::YZ);
        assert_eq!(tex.plane_s(), (0.0, 1.0, 0.0, 0.0));
        assert_eq!(tex.plane_t(), (0.0, 0.0, 1.0, 0.0));
    }

    #[test]
    fn test_set_plane_zx() {
        let mut tex = Texture2Dplane::new();
        tex.set_plane(NameOfTexturePlane::ZX);
        assert_eq!(tex.plane(), NameOfTexturePlane::ZX);
        assert_eq!(tex.plane_s(), (0.0, 0.0, 1.0, 0.0));
        assert_eq!(tex.plane_t(), (1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_set_scales() {
        let mut tex = Texture2Dplane::new();
        tex.set_scale_s(2.5);
        tex.set_scale_t(3.5);
        assert_eq!(tex.scale_s(), 2.5);
        assert_eq!(tex.scale_t(), 3.5);
    }

    #[test]
    fn test_set_translations() {
        let mut tex = Texture2Dplane::new();
        tex.set_translate_s(1.5);
        tex.set_translate_t(2.5);
        assert_eq!(tex.translate_s(), 1.5);
        assert_eq!(tex.translate_t(), 2.5);
    }

    #[test]
    fn test_set_rotation() {
        let mut tex = Texture2Dplane::new();
        tex.set_rotation(45.0);
        assert_eq!(tex.rotation(), 45.0);
    }

    #[test]
    fn test_combined_transformations() {
        let mut tex = Texture2Dplane::new();
        tex.set_plane(NameOfTexturePlane::ZX);
        tex.set_scale_s(2.0);
        tex.set_scale_t(3.0);
        tex.set_translate_s(0.5);
        tex.set_translate_t(1.0);
        tex.set_rotation(90.0);

        assert_eq!(tex.plane(), NameOfTexturePlane::ZX);
        assert_eq!(tex.scale_s(), 2.0);
        assert_eq!(tex.scale_t(), 3.0);
        assert_eq!(tex.translate_s(), 0.5);
        assert_eq!(tex.translate_t(), 1.0);
        assert_eq!(tex.rotation(), 90.0);
    }
}
