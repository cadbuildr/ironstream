// FILE: crates/ironstream/src/ais_shape.rs

// occt: AIS_Shape
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayMode {
    Wireframe,
    Shaded,
    ShadedWithEdges,
}

// occt: AIS_Shape
pub struct AisShape {
    pub shape: String,
    pub color: [f64; 3],
    pub transparency: f64,
    pub display_mode: DisplayMode,
    pub visible: bool,
}

impl AisShape {
    pub fn new(shape: &str) -> Self {
        AisShape {
            shape: shape.to_string(),
            color: [1.0, 1.0, 1.0],
            transparency: 0.0,
            display_mode: DisplayMode::Shaded,
            visible: true,
        }
    }

    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.color = [r, g, b];
    }

    pub fn set_transparency(&mut self, t: f64) {
        self.transparency = t;
    }

    pub fn set_display_mode(&mut self, m: DisplayMode) {
        self.display_mode = m;
    }

    pub fn shape(&self) -> &str {
        &self.shape
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

// occt: AIS_ColoredShape
pub struct ColoredShape {
    pub base: AisShape,
    /// Each entry pairs a face identifier encoded as RGB key with its assigned color.
    /// The first [f64;3] is reserved for face identification (placeholder); the second is the color.
    pub face_colors: Vec<([f64; 3], [f64; 3])>,
}

impl ColoredShape {
    pub fn new(shape: &str) -> Self {
        ColoredShape {
            base: AisShape::new(shape),
            face_colors: Vec::new(),
        }
    }

    /// Associate a custom color with a named face sub-shape.
    /// The face string is hashed into a deterministic f64 triple for storage.
    pub fn set_custom_color(&mut self, face: &str, color: [f64; 3]) {
        let key = face_key(face);
        // Replace existing entry for the same key, or push a new one.
        for entry in self.face_colors.iter_mut() {
            if entry.0 == key {
                entry.1 = color;
                return;
            }
        }
        self.face_colors.push((key, color));
    }
}

/// Derive a stable [f64;3] key from a face name string using a simple hash.
fn face_key(face: &str) -> [f64; 3] {
    let bytes = face.as_bytes();
    let mut h0: u64 = 0x9e3779b97f4a7c15;
    let mut h1: u64 = 0x6c62272e07bb0142;
    let mut h2: u64 = 0x94d049bb133111eb;
    for (i, &b) in bytes.iter().enumerate() {
        match i % 3 {
            0 => h0 = h0.wrapping_mul(0x517cc1b727220a95).wrapping_add(b as u64),
            1 => h1 = h1.wrapping_mul(0x6b86b273ff34fce9).wrapping_add(b as u64),
            _ => h2 = h2.wrapping_mul(0xbf58476d1ce4e5b9).wrapping_add(b as u64),
        }
    }
    [h0 as f64, h1 as f64, h2 as f64]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ais_shape_defaults() {
        let s = AisShape::new("box");
        assert_eq!(s.shape(), "box");
        assert_eq!(s.color, [1.0, 1.0, 1.0]);
        assert_eq!(s.transparency, 0.0);
        assert_eq!(s.display_mode, DisplayMode::Shaded);
        assert!(s.is_visible());
    }

    #[test]
    fn ais_shape_setters() {
        let mut s = AisShape::new("sphere");
        s.set_color(0.1, 0.2, 0.3);
        assert_eq!(s.color, [0.1, 0.2, 0.3]);
        s.set_transparency(0.5);
        assert_eq!(s.transparency, 0.5);
        s.set_display_mode(DisplayMode::Wireframe);
        assert_eq!(s.display_mode, DisplayMode::Wireframe);
        s.visible = false;
        assert!(!s.is_visible());
    }

    #[test]
    fn colored_shape_face_colors() {
        let mut cs = ColoredShape::new("body");
        cs.set_custom_color("face_1", [1.0, 0.0, 0.0]);
        cs.set_custom_color("face_2", [0.0, 1.0, 0.0]);
        assert_eq!(cs.face_colors.len(), 2);
        // Overwrite should not grow the vec.
        cs.set_custom_color("face_1", [0.0, 0.0, 1.0]);
        assert_eq!(cs.face_colors.len(), 2);
        let key1 = face_key("face_1");
        let found = cs.face_colors.iter().find(|e| e.0 == key1).unwrap();
        assert_eq!(found.1, [0.0, 0.0, 1.0]);
    }
}
