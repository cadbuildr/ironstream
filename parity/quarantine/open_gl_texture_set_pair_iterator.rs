// FILE: open_gl_texture_set_pair_iterator.rs
// occt: OpenGl_TextureSetPairIterator

//! Class for iterating pair of texture sets through each defined texture slot.
//! Note that iterator considers texture slots being in ascending order within OpenGl_TextureSet.

/// Simplified texture unit enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextureUnit {
    Unit0 = 0,
    Unit1 = 1,
    Unit2 = 2,
    Unit3 = 3,
    Unit4 = 4,
    Unit5 = 5,
    Unit6 = 6,
    Unit7 = 7,
}

impl TextureUnit {
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(TextureUnit::Unit0),
            1 => Some(TextureUnit::Unit1),
            2 => Some(TextureUnit::Unit2),
            3 => Some(TextureUnit::Unit3),
            4 => Some(TextureUnit::Unit4),
            5 => Some(TextureUnit::Unit5),
            6 => Some(TextureUnit::Unit6),
            7 => Some(TextureUnit::Unit7),
            _ => None,
        }
    }

    pub fn to_int(&self) -> i32 {
        *self as i32
    }
}

/// Simplified OpenGL Texture placeholder
#[derive(Debug, Clone)]
pub struct OpenGlTexture {
    id: u32,
}

impl OpenGlTexture {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Iterator over a single texture set
struct TextureSetIterator {
    textures: Vec<(Option<OpenGlTexture>, TextureUnit)>,
    index: usize,
}

impl TextureSetIterator {
    fn new(textures: Vec<(Option<OpenGlTexture>, TextureUnit)>) -> Self {
        Self { textures, index: 0 }
    }

    fn more(&self) -> bool {
        self.index < self.textures.len()
    }

    fn unit(&self) -> Option<TextureUnit> {
        if self.more() {
            Some(self.textures[self.index].1)
        } else {
            None
        }
    }

    fn value(&self) -> Option<&Option<OpenGlTexture>> {
        if self.more() {
            Some(&self.textures[self.index].0)
        } else {
            None
        }
    }

    fn next(&mut self) {
        if self.more() {
            self.index += 1;
        }
    }
}

/// Class for iterating pair of texture sets
pub struct OpenGlTextureSetPairIterator {
    iter1: TextureSetIterator,
    iter2: TextureSetIterator,
    texture1: Option<OpenGlTexture>,
    texture2: Option<OpenGlTexture>,
    unit_lower: i32,
    unit_upper: i32,
    unit_current: i32,
}

impl OpenGlTextureSetPairIterator {
    /// Constructor with two texture sets
    pub fn new(
        textures1: Vec<(Option<OpenGlTexture>, TextureUnit)>,
        textures2: Vec<(Option<OpenGlTexture>, TextureUnit)>,
    ) -> Self {
        let mut unit_lower = i32::MAX;
        let mut unit_upper = i32::MIN;

        if !textures1.is_empty() {
            unit_lower = unit_lower.min(textures1[0].1.to_int());
            unit_upper = unit_upper.max(textures1[textures1.len() - 1].1.to_int());
        }

        if !textures2.is_empty() {
            unit_lower = unit_lower.min(textures2[0].1.to_int());
            unit_upper = unit_upper.max(textures2[textures2.len() - 1].1.to_int());
        }

        let unit_current = unit_lower;

        let mut iter1 = TextureSetIterator::new(textures1);
        let mut iter2 = TextureSetIterator::new(textures2);

        let texture1 = if iter1.more() && iter1.unit() == Some(TextureUnit::from_int(unit_current).unwrap_or(TextureUnit::Unit0)) {
            iter1.value().cloned().flatten()
        } else {
            None
        };

        let texture2 = if iter2.more() && iter2.unit() == Some(TextureUnit::from_int(unit_current).unwrap_or(TextureUnit::Unit0)) {
            iter2.value().cloned().flatten()
        } else {
            None
        };

        Self {
            iter1,
            iter2,
            texture1,
            texture2,
            unit_lower,
            unit_upper,
            unit_current,
        }
    }

    /// Return TRUE if there are more texture units to pass through
    pub fn more(&self) -> bool {
        self.unit_current <= self.unit_upper
    }

    /// Return current texture unit
    pub fn unit(&self) -> TextureUnit {
        TextureUnit::from_int(self.unit_current).unwrap_or(TextureUnit::Unit0)
    }

    /// Access texture from first texture set
    pub fn texture1(&self) -> Option<&OpenGlTexture> {
        self.texture1.as_ref()
    }

    /// Access texture from second texture set
    pub fn texture2(&self) -> Option<&OpenGlTexture> {
        self.texture2.as_ref()
    }

    /// Move iterator position to the next pair
    pub fn next(&mut self) {
        self.unit_current += 1;
        self.texture1 = None;
        self.texture2 = None;

        // Search for next texture in iter1
        while self.iter1.more() {
            if let Some(unit) = self.iter1.unit() {
                if unit.to_int() >= self.unit_current {
                    if unit.to_int() == self.unit_current {
                        self.texture1 = self.iter1.value().cloned().flatten();
                    }
                    break;
                }
            }
            self.iter1.next();
        }

        // Search for next texture in iter2
        while self.iter2.more() {
            if let Some(unit) = self.iter2.unit() {
                if unit.to_int() >= self.unit_current {
                    if unit.to_int() == self.unit_current {
                        self.texture2 = self.iter2.value().cloned().flatten();
                    }
                    break;
                }
            }
            self.iter2.next();
        }
    }

    /// Check if either texture exists at current position
    pub fn has_texture(&self) -> bool {
        self.texture1.is_some() || self.texture2.is_some()
    }

    /// Count remaining units
    pub fn remaining_units(&self) -> i32 {
        if self.more() {
            self.unit_upper - self.unit_current + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_iterator() {
        let iter = OpenGlTextureSetPairIterator::new(Vec::new(), Vec::new());
        assert!(!iter.more());
        assert_eq!(iter.texture1(), None);
        assert_eq!(iter.texture2(), None);
    }

    #[test]
    fn test_single_set_first_set() {
        let textures1 = vec![(Some(OpenGlTexture::new(1)), TextureUnit::Unit0)];
        let iter = OpenGlTextureSetPairIterator::new(textures1, Vec::new());

        assert!(iter.more());
        assert_eq!(iter.unit(), TextureUnit::Unit0);
        assert!(iter.texture1().is_some());
        assert_eq!(iter.texture2(), None);
    }

    #[test]
    fn test_single_set_second_set() {
        let textures2 = vec![(Some(OpenGlTexture::new(2)), TextureUnit::Unit1)];
        let iter = OpenGlTextureSetPairIterator::new(Vec::new(), textures2);

        assert!(iter.more());
        assert_eq!(iter.unit(), TextureUnit::Unit1);
        assert_eq!(iter.texture1(), None);
        assert!(iter.texture2().is_some());
    }

    #[test]
    fn test_both_sets_same_unit() {
        let textures1 = vec![(Some(OpenGlTexture::new(1)), TextureUnit::Unit2)];
        let textures2 = vec![(Some(OpenGlTexture::new(2)), TextureUnit::Unit2)];
        let iter = OpenGlTextureSetPairIterator::new(textures1, textures2);

        assert!(iter.more());
        assert_eq!(iter.unit(), TextureUnit::Unit2);
        assert!(iter.texture1().is_some());
        assert!(iter.texture2().is_some());
    }

    #[test]
    fn test_both_sets_different_units() {
        let textures1 = vec![
            (Some(OpenGlTexture::new(1)), TextureUnit::Unit0),
            (Some(OpenGlTexture::new(2)), TextureUnit::Unit2),
        ];
        let textures2 = vec![
            (Some(OpenGlTexture::new(3)), TextureUnit::Unit1),
            (Some(OpenGlTexture::new(4)), TextureUnit::Unit3),
        ];
        let iter = OpenGlTextureSetPairIterator::new(textures1, textures2);

        assert!(iter.more());
        assert_eq!(iter.unit(), TextureUnit::Unit0);
        assert!(iter.has_texture());
    }

    #[test]
    fn test_next_iteration() {
        let textures1 = vec![
            (Some(OpenGlTexture::new(1)), TextureUnit::Unit0),
            (Some(OpenGlTexture::new(2)), TextureUnit::Unit2),
        ];
        let textures2 = vec![(Some(OpenGlTexture::new(3)), TextureUnit::Unit1)];

        let mut iter = OpenGlTextureSetPairIterator::new(textures1, textures2);

        assert_eq!(iter.unit(), TextureUnit::Unit0);
        iter.next();

        assert_eq!(iter.unit(), TextureUnit::Unit1);
        iter.next();

        assert_eq!(iter.unit(), TextureUnit::Unit2);
        iter.next();

        // Should not be more units after Unit3
        assert!(!iter.more());
    }

    #[test]
    fn test_remaining_units() {
        let textures1 = vec![
            (Some(OpenGlTexture::new(1)), TextureUnit::Unit0),
            (Some(OpenGlTexture::new(2)), TextureUnit::Unit2),
        ];
        let iter = OpenGlTextureSetPairIterator::new(textures1, Vec::new());

        assert!(iter.more());
        assert_eq!(iter.remaining_units(), 3); // Units 0, 1, 2
    }

    #[test]
    fn test_texture_unit_conversion() {
        assert_eq!(TextureUnit::Unit0.to_int(), 0);
        assert_eq!(TextureUnit::Unit7.to_int(), 7);
        assert_eq!(TextureUnit::from_int(3), Some(TextureUnit::Unit3));
        assert_eq!(TextureUnit::from_int(10), None);
    }
}
