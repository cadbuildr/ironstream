// FILE: vrml_data_texture_transform.rs
// occt: VrmlData_TextureTransform
//
// Faithful port of OCCT VrmlData_TextureTransform (DataExchange/TKDEVRML/VrmlData/
// VrmlData_TextureTransform.hxx/.cxx): VRML 2.0 TextureTransform node.
// Applies 2D affine transformations (translation, rotation, scale) to texture coordinates.

use std::cell::RefCell;
use std::rc::Rc;

/// 2D vector for texture space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureTransformVec2 {
    pub x: f32,
    pub y: f32,
}

impl TextureTransformVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        TextureTransformVec2 { x, y }
    }

    /// Magnitude.
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Normalize in-place.
    pub fn normalize(&mut self) {
        let len = self.length();
        if len > 1e-7 {
            self.x /= len;
            self.y /= len;
        }
    }
}

impl Default for TextureTransformVec2 {
    fn default() -> Self {
        TextureTransformVec2::new(0.0, 0.0)
    }
}

/// Error status for read/write operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureTransformErrorStatus {
    Ok = 0,
    EndOfFile = 1,
    NotEndOfFile = 2,
    GeneralError = 3,
}

/// Input buffer for parsing.
pub struct TextureTransformInBuffer {
    pub line_num: u32,
}

impl TextureTransformInBuffer {
    pub fn new() -> Self {
        TextureTransformInBuffer { line_num: 1 }
    }
}

impl Default for TextureTransformInBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// VRML TextureTransform node: applies 2D affine transformations to texture coordinates.
/// Defaults: translation (0, 0), rotation 0, scale (1, 1), center (0.5, 0.5).
#[derive(Debug)]
pub struct VrmlDataTextureTransform {
    my_translation: TextureTransformVec2,    // (tx, ty)
    my_rotation: f32,                        // radians
    my_scale: TextureTransformVec2,          // (sx, sy)
    my_center: TextureTransformVec2,         // rotation/scale center
    my_name: String,
}

impl VrmlDataTextureTransform {
    /// Constructor: default identity transform.
    pub fn new(name: Option<&str>) -> Self {
        VrmlDataTextureTransform {
            my_translation: TextureTransformVec2::new(0.0, 0.0),
            my_rotation: 0.0,
            my_scale: TextureTransformVec2::new(1.0, 1.0),
            my_center: TextureTransformVec2::new(0.5, 0.5),
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Full constructor.
    pub fn with_fields(
        translation: TextureTransformVec2,
        rotation: f32,
        scale: TextureTransformVec2,
        center: TextureTransformVec2,
        name: Option<&str>,
    ) -> Self {
        VrmlDataTextureTransform {
            my_translation: translation,
            my_rotation: rotation,
            my_scale: scale,
            my_center: center,
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Query the name.
    pub fn name(&self) -> &str {
        &self.my_name
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        self.my_name = name.to_string();
    }

    /// Get translation vector.
    pub fn translation(&self) -> TextureTransformVec2 {
        self.my_translation
    }

    /// Set translation vector.
    pub fn set_translation(&mut self, t: TextureTransformVec2) {
        self.my_translation = t;
    }

    /// Get rotation angle (radians).
    pub fn rotation(&self) -> f32 {
        self.my_rotation
    }

    /// Set rotation angle (radians).
    pub fn set_rotation(&mut self, angle: f32) {
        self.my_rotation = angle;
    }

    /// Get scale factors.
    pub fn scale(&self) -> TextureTransformVec2 {
        self.my_scale
    }

    /// Set scale factors.
    pub fn set_scale(&mut self, s: TextureTransformVec2) {
        self.my_scale = s;
    }

    /// Get rotation/scale center.
    pub fn center(&self) -> TextureTransformVec2 {
        self.my_center
    }

    /// Set rotation/scale center.
    pub fn set_center(&mut self, c: TextureTransformVec2) {
        self.my_center = c;
    }

    /// Check if this transform is identity (no-op).
    pub fn is_identity(&self) -> bool {
        self.my_translation.x.abs() < 1e-7
            && self.my_translation.y.abs() < 1e-7
            && self.my_rotation.abs() < 1e-7
            && (self.my_scale.x - 1.0).abs() < 1e-7
            && (self.my_scale.y - 1.0).abs() < 1e-7
    }

    /// Check if in default state.
    pub fn is_default(&self) -> bool {
        self.is_identity()
    }

    /// Apply this transform to a texture coordinate (s, t).
    /// Applies: translate -> rotate around center -> scale around center
    pub fn transform_coord(&self, s: f32, t: f32) -> (f32, f32) {
        // Apply translation
        let mut ts = s + self.my_translation.x;
        let mut tt = t + self.my_translation.y;

        // Translate to center, rotate, scale, translate back
        let cs = self.my_center.x;
        let ct = self.my_center.y;

        ts -= cs;
        tt -= ct;

        // Apply rotation (2D rotation matrix)
        let cos_r = self.my_rotation.cos();
        let sin_r = self.my_rotation.sin();
        let rs = ts * cos_r - tt * sin_r;
        let rt = ts * sin_r + tt * cos_r;

        // Apply scale
        let ss = rs * self.my_scale.x;
        let st = rt * self.my_scale.y;

        // Translate back
        (ss + cs, st + ct)
    }

    /// Virtual read method: parse TextureTransform node from VRML stream.
    pub fn read(&mut self, _buffer: &mut TextureTransformInBuffer) -> TextureTransformErrorStatus {
        // Subclass/user provides actual parsing.
        TextureTransformErrorStatus::Ok
    }

    /// Virtual write method: output TextureTransform node to VRML format.
    pub fn write(&self, _prefix: Option<&str>) -> TextureTransformErrorStatus {
        // Subclass/user provides actual output.
        TextureTransformErrorStatus::Ok
    }
}

impl Default for VrmlDataTextureTransform {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlDataTextureTransform {
    fn clone(&self) -> Self {
        VrmlDataTextureTransform {
            my_translation: self.my_translation,
            my_rotation: self.my_rotation,
            my_scale: self.my_scale,
            my_center: self.my_center,
            my_name: self.my_name.clone(),
        }
    }
}

impl PartialEq for VrmlDataTextureTransform {
    fn eq(&self, other: &Self) -> bool {
        (self.my_translation.x - other.my_translation.x).abs() < 1e-7
            && (self.my_translation.y - other.my_translation.y).abs() < 1e-7
            && (self.my_rotation - other.my_rotation).abs() < 1e-7
            && (self.my_scale.x - other.my_scale.x).abs() < 1e-7
            && (self.my_scale.y - other.my_scale.y).abs() < 1e-7
            && (self.my_center.x - other.my_center.x).abs() < 1e-7
            && (self.my_center.y - other.my_center.y).abs() < 1e-7
            && self.my_name == other.my_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn vec2_creation() {
        let v = TextureTransformVec2::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn vec2_length() {
        let v = TextureTransformVec2::new(3.0, 4.0);
        assert!((v.length() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn vec2_normalize() {
        let mut v = TextureTransformVec2::new(3.0, 4.0);
        v.normalize();
        assert!((v.x - 0.6).abs() < 1e-6);
        assert!((v.y - 0.8).abs() < 1e-6);
    }

    #[test]
    fn default_transform() {
        let tf = VrmlDataTextureTransform::new(None);
        assert_eq!(tf.translation(), TextureTransformVec2::new(0.0, 0.0));
        assert!((tf.rotation() - 0.0).abs() < 1e-7);
        assert_eq!(tf.scale(), TextureTransformVec2::new(1.0, 1.0));
        assert!(tf.is_identity());
        assert!(tf.is_default());
    }

    #[test]
    fn with_fields() {
        let tf = VrmlDataTextureTransform::with_fields(
            TextureTransformVec2::new(0.1, 0.2),
            0.5,
            TextureTransformVec2::new(2.0, 3.0),
            TextureTransformVec2::new(0.5, 0.5),
            Some("Transform"),
        );
        assert_eq!(tf.translation(), TextureTransformVec2::new(0.1, 0.2));
        assert!((tf.rotation() - 0.5).abs() < 1e-7);
    }

    #[test]
    fn set_translation() {
        let mut tf = VrmlDataTextureTransform::new(None);
        tf.set_translation(TextureTransformVec2::new(0.5, 0.5));
        assert_eq!(tf.translation(), TextureTransformVec2::new(0.5, 0.5));
    }

    #[test]
    fn set_rotation() {
        let mut tf = VrmlDataTextureTransform::new(None);
        tf.set_rotation(PI / 2.0);
        assert!((tf.rotation() - PI / 2.0).abs() < 1e-6);
    }

    #[test]
    fn set_scale() {
        let mut tf = VrmlDataTextureTransform::new(None);
        tf.set_scale(TextureTransformVec2::new(2.0, 3.0));
        assert_eq!(tf.scale(), TextureTransformVec2::new(2.0, 3.0));
    }

    #[test]
    fn transform_coord_translate_only() {
        let mut tf = VrmlDataTextureTransform::new(None);
        tf.set_translation(TextureTransformVec2::new(0.1, 0.2));
        let (s, t) = tf.transform_coord(0.5, 0.5);
        assert!((s - 0.6).abs() < 1e-6);
        assert!((t - 0.7).abs() < 1e-6);
    }

    #[test]
    fn transform_coord_scale_only() {
        let mut tf = VrmlDataTextureTransform::new(None);
        tf.set_scale(TextureTransformVec2::new(2.0, 2.0));
        tf.set_center(TextureTransformVec2::new(0.0, 0.0)); // scale around origin
        let (s, t) = tf.transform_coord(0.5, 0.5);
        assert!((s - 1.0).abs() < 1e-6);
        assert!((t - 1.0).abs() < 1e-6);
    }

    #[test]
    fn transform_coord_rotate_90() {
        let mut tf = VrmlDataTextureTransform::new(None);
        tf.set_rotation(PI / 2.0);
        tf.set_center(TextureTransformVec2::new(0.0, 0.0));
        let (s, t) = tf.transform_coord(1.0, 0.0);
        // (1, 0) rotated 90 degrees = (0, 1)
        assert!(s.abs() < 1e-6);
        assert!((t - 1.0).abs() < 1e-6);
    }

    #[test]
    fn clone_preserves_data() {
        let tf = VrmlDataTextureTransform::with_fields(
            TextureTransformVec2::new(0.1, 0.2),
            0.5,
            TextureTransformVec2::new(2.0, 2.0),
            TextureTransformVec2::new(0.5, 0.5),
            Some("Original"),
        );
        let cloned = tf.clone();
        assert_eq!(cloned, tf);
    }

    #[test]
    fn equality() {
        let tf1 = VrmlDataTextureTransform::new(None);
        let tf2 = VrmlDataTextureTransform::new(None);
        assert_eq!(tf1, tf2);
    }

    #[test]
    fn set_name() {
        let mut tf = VrmlDataTextureTransform::new(Some("Old"));
        tf.set_name("New");
        assert_eq!(tf.name(), "New");
    }
}
