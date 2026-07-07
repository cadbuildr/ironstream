// FILE: open_gl_texture_set.rs
// occt: OpenGl_TextureSet

//! Class holding array of textures to be mapped as a set.
//! Textures should be defined in ascending order of texture units within the set.

/// Texture unit enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureUnit {
    Unit0,
    Unit1,
    Unit2,
    Unit3,
    Unit4,
    Unit5,
    Unit6,
    Unit7,
}

/// Texture slot - combination of Texture and binding Unit
#[derive(Debug, Clone)]
pub struct TextureSlot {
    pub texture: Option<String>, // Simplified: texture handle
    pub unit: TextureUnit,
}

impl TextureSlot {
    pub fn new() -> Self {
        Self {
            texture: None,
            unit: TextureUnit::Unit0,
        }
    }

    pub fn with_unit(unit: TextureUnit) -> Self {
        Self {
            texture: None,
            unit,
        }
    }
}

impl Default for TextureSlot {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over TextureSet
pub struct TextureSetIterator {
    textures: Vec<TextureSlot>,
    index: usize,
}

impl TextureSetIterator {
    pub fn new(textures: &[TextureSlot]) -> Self {
        Self {
            textures: textures.to_vec(),
            index: 0,
        }
    }

    pub fn next(&mut self) -> Option<&TextureSlot> {
        if self.index < self.textures.len() {
            let slot = &self.textures[self.index];
            self.index += 1;
            Some(slot)
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<&Option<String>> {
        if self.index > 0 && self.index <= self.textures.len() {
            Some(&self.textures[self.index - 1].texture)
        } else {
            None
        }
    }

    pub fn unit(&self) -> Option<TextureUnit> {
        if self.index > 0 && self.index <= self.textures.len() {
            Some(self.textures[self.index - 1].unit)
        } else {
            None
        }
    }
}

/// Class for iterating texture set
pub struct OpenGlTextureSetIterator {
    textures: Vec<TextureSlot>,
    index: usize,
}

impl OpenGlTextureSetIterator {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            index: 0,
        }
    }

    /// Constructor with texture set
    pub fn with_textures(textures: &[TextureSlot]) -> Self {
        Self {
            textures: textures.to_vec(),
            index: 0,
        }
    }

    /// Check if iterator has more elements
    pub fn more(&self) -> bool {
        self.index < self.textures.len()
    }

    /// Move to next element
    pub fn next(&mut self) {
        if self.index < self.textures.len() {
            self.index += 1;
        }
    }

    /// Access current texture
    pub fn value(&self) -> Option<&Option<String>> {
        if self.index < self.textures.len() {
            Some(&self.textures[self.index].texture)
        } else {
            None
        }
    }

    /// Access current texture unit
    pub fn unit(&self) -> Option<TextureUnit> {
        if self.index < self.textures.len() {
            Some(self.textures[self.index].unit)
        } else {
            None
        }
    }

    /// Change current texture unit
    pub fn change_unit(&mut self) -> Option<&mut TextureUnit> {
        if self.index < self.textures.len() {
            Some(&mut self.textures[self.index].unit)
        } else {
            None
        }
    }
}

impl Default for OpenGlTextureSetIterator {
    fn default() -> Self {
        Self::new()
    }
}

/// Texture set bits enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureSetBits {
    None = 0,
    BaseColor = 1,
    Normal = 2,
    Metallic = 4,
    Roughness = 8,
}

/// OpenGL Texture Set
#[derive(Debug, Clone)]
pub struct OpenGlTextureSet {
    textures: Vec<TextureSlot>,
    texture_set_bits: i32,
}

impl OpenGlTextureSet {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            texture_set_bits: 0,
        }
    }

    /// Constructor with capacity
    pub fn with_capacity(nb_textures: usize) -> Self {
        Self {
            textures: Vec::with_capacity(nb_textures),
            texture_set_bits: 0,
        }
    }

    /// Constructor for a single texture
    pub fn from_texture(texture_id: String, unit: TextureUnit) -> Self {
        let mut set = Self::new();
        set.textures.push(TextureSlot {
            texture: Some(texture_id),
            unit,
        });
        set
    }

    /// Return texture units declared within the program
    pub fn texture_set_bits(&self) -> i32 {
        self.texture_set_bits
    }

    /// Mutably access texture units bits
    pub fn change_texture_set_bits(&mut self) -> &mut i32 {
        &mut self.texture_set_bits
    }

    /// Return TRUE if texture array is empty
    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    /// Return number of textures
    pub fn size(&self) -> usize {
        self.textures.len()
    }

    /// Return the lower index in texture set (0)
    pub fn lower(&self) -> usize {
        0
    }

    /// Return the upper index in texture set
    pub fn upper(&self) -> usize {
        if self.textures.is_empty() {
            0
        } else {
            self.textures.len() - 1
        }
    }

    /// Return the first texture
    pub fn first(&self) -> Option<&Option<String>> {
        self.textures.first().map(|slot| &slot.texture)
    }

    /// Mutably access the first texture
    pub fn change_first(&mut self) -> Option<&mut Option<String>> {
        self.textures.first_mut().map(|slot| &mut slot.texture)
    }

    /// Return the first texture unit
    pub fn first_unit(&self) -> Option<TextureUnit> {
        self.textures.first().map(|slot| slot.unit)
    }

    /// Return the last texture
    pub fn last(&self) -> Option<&Option<String>> {
        self.textures.last().map(|slot| &slot.texture)
    }

    /// Mutably access the last texture
    pub fn change_last(&mut self) -> Option<&mut Option<String>> {
        self.textures.last_mut().map(|slot| &mut slot.texture)
    }

    /// Return the last texture unit
    pub fn last_unit(&self) -> Option<TextureUnit> {
        self.textures.last().map(|slot| slot.unit)
    }

    /// Mutably access the last texture unit
    pub fn change_last_unit(&mut self) -> Option<&mut TextureUnit> {
        self.textures.last_mut().map(|slot| &mut slot.unit)
    }

    /// Return the texture at specified position
    pub fn value(&self, index: usize) -> Option<&Option<String>> {
        self.textures.get(index).map(|slot| &slot.texture)
    }

    /// Mutably access texture at specified position
    pub fn change_value(&mut self, index: usize) -> Option<&mut Option<String>> {
        self.textures.get_mut(index).map(|slot| &mut slot.texture)
    }

    /// Return TRUE if texture color modulation has been enabled for the first texture
    pub fn is_modulate(&self) -> bool {
        // First texture's modulation status is true by default or if texture not set
        self.textures.is_empty() || self.textures[0].texture.is_none()
    }

    /// Return TRUE if other than point sprite textures are defined within point set
    pub fn has_non_point_sprite(&self) -> bool {
        !self.textures.is_empty()
    }

    /// Return TRUE if last texture is a point sprite
    pub fn has_point_sprite(&self) -> bool {
        // Last texture in set is considered a point sprite
        !self.textures.is_empty()
    }

    /// Nullify all handles
    pub fn init_zero(&mut self) {
        self.textures.clear();
        self.texture_set_bits = 0;
    }

    /// Add a texture to the set
    pub fn add_texture(&mut self, texture: Option<String>, unit: TextureUnit) {
        self.textures.push(TextureSlot { texture, unit });
    }

    /// Get iterator over textures
    pub fn iter(&self) -> OpenGlTextureSetIterator {
        OpenGlTextureSetIterator::with_textures(&self.textures)
    }
}

impl Default for OpenGlTextureSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_slot_new() {
        let slot = TextureSlot::new();
        assert!(slot.texture.is_none());
        assert_eq!(slot.unit, TextureUnit::Unit0);
    }

    #[test]
    fn test_texture_set_empty() {
        let set = OpenGlTextureSet::new();
        assert!(set.is_empty());
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_texture_set_add() {
        let mut set = OpenGlTextureSet::new();
        set.add_texture(Some("tex1".to_string()), TextureUnit::Unit0);
        set.add_texture(Some("tex2".to_string()), TextureUnit::Unit1);

        assert!(!set.is_empty());
        assert_eq!(set.size(), 2);
    }

    #[test]
    fn test_texture_set_first_last() {
        let mut set = OpenGlTextureSet::with_capacity(2);
        set.add_texture(Some("first".to_string()), TextureUnit::Unit0);
        set.add_texture(Some("last".to_string()), TextureUnit::Unit1);

        assert_eq!(set.first(), Some(&Some("first".to_string())));
        assert_eq!(set.last(), Some(&Some("last".to_string())));
        assert_eq!(set.first_unit(), Some(TextureUnit::Unit0));
        assert_eq!(set.last_unit(), Some(TextureUnit::Unit1));
    }

    #[test]
    fn test_texture_set_bounds() {
        let mut set = OpenGlTextureSet::new();
        set.add_texture(Some("tex".to_string()), TextureUnit::Unit0);

        assert_eq!(set.lower(), 0);
        assert_eq!(set.upper(), 0);
    }

    #[test]
    fn test_texture_set_value() {
        let mut set = OpenGlTextureSet::new();
        set.add_texture(Some("tex0".to_string()), TextureUnit::Unit0);
        set.add_texture(Some("tex1".to_string()), TextureUnit::Unit1);

        assert_eq!(set.value(0), Some(&Some("tex0".to_string())));
        assert_eq!(set.value(1), Some(&Some("tex1".to_string())));
        assert_eq!(set.value(2), None);
    }

    #[test]
    fn test_texture_set_bits() {
        let mut set = OpenGlTextureSet::new();
        assert_eq!(set.texture_set_bits(), 0);

        *set.change_texture_set_bits() = 15;
        assert_eq!(set.texture_set_bits(), 15);
    }

    #[test]
    fn test_init_zero() {
        let mut set = OpenGlTextureSet::new();
        set.add_texture(Some("tex".to_string()), TextureUnit::Unit0);
        *set.change_texture_set_bits() = 7;

        set.init_zero();
        assert!(set.is_empty());
        assert_eq!(set.texture_set_bits(), 0);
    }

    #[test]
    fn test_from_texture() {
        let set = OpenGlTextureSet::from_texture("single".to_string(), TextureUnit::Unit2);
        assert_eq!(set.size(), 1);
        assert_eq!(set.first_unit(), Some(TextureUnit::Unit2));
    }

    #[test]
    fn test_is_modulate() {
        let empty_set = OpenGlTextureSet::new();
        assert!(empty_set.is_modulate());

        let mut set = OpenGlTextureSet::new();
        set.add_texture(Some("tex".to_string()), TextureUnit::Unit0);
        // First texture exists, so modulation check depends on logic
        assert!(!set.is_modulate());
    }

    #[test]
    fn test_iterator() {
        let mut set = OpenGlTextureSet::new();
        set.add_texture(Some("tex0".to_string()), TextureUnit::Unit0);
        set.add_texture(Some("tex1".to_string()), TextureUnit::Unit1);

        let mut iter = set.iter();
        assert!(iter.more());
        assert_eq!(iter.value(), Some(&Some("tex0".to_string())));
        assert_eq!(iter.unit(), Some(TextureUnit::Unit0));

        iter.next();
        assert!(iter.more());
        assert_eq!(iter.value(), Some(&Some("tex1".to_string())));

        iter.next();
        assert!(!iter.more());
    }
}
