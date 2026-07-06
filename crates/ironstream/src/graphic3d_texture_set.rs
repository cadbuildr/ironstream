// FILE: graphic3d_texture_set.rs
// occt: Graphic3d_TextureSet

/// A simple texture map placeholder for testing purposes.
/// In a real implementation, this would reference the full TextureMap type.
#[derive(Debug, Clone)]
pub struct TextureMapRef(pub String);

/// Class holding array of textures to be mapped as a set.
/// Textures should be defined in ascending order of texture units within the set.
pub struct TextureSet {
    textures: Vec<TextureMapRef>,
    lower_index: i32,
}

/// Iterator for TextureSet.
pub struct TextureSetIterator {
    textures: Vec<TextureMapRef>,
    current_index: usize,
}

impl TextureSetIterator {
    /// Creates an iterator over the texture set.
    pub fn new(set: &TextureSet) -> Self {
        TextureSetIterator {
            textures: set.textures.clone(),
            current_index: 0,
        }
    }

    /// Checks if there are more textures to iterate.
    pub fn has_next(&self) -> bool {
        self.current_index < self.textures.len()
    }

    /// Returns the next texture if available.
    pub fn next(&mut self) -> Option<&TextureMapRef> {
        if self.current_index < self.textures.len() {
            let tex = &self.textures[self.current_index];
            self.current_index += 1;
            Some(tex)
        } else {
            None
        }
    }
}

impl TextureSet {
    /// Empty constructor.
    pub fn new() -> Self {
        TextureSet {
            textures: Vec::new(),
            lower_index: 0,
        }
    }

    /// Constructor with specified number of texture slots.
    pub fn with_capacity(num_textures: usize) -> Self {
        TextureSet {
            textures: Vec::with_capacity(num_textures),
            lower_index: 0,
        }
    }

    /// Constructor for a single texture.
    pub fn from_single(texture: TextureMapRef) -> Self {
        TextureSet {
            textures: vec![texture],
            lower_index: 0,
        }
    }

    /// Return TRUE if texture array is empty.
    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    /// Return number of textures.
    pub fn size(&self) -> usize {
        self.textures.len()
    }

    /// Return the lower index in texture set.
    pub fn lower(&self) -> i32 {
        self.lower_index
    }

    /// Return the upper index in texture set.
    pub fn upper(&self) -> i32 {
        if self.textures.is_empty() {
            self.lower_index - 1
        } else {
            self.lower_index + self.textures.len() as i32 - 1
        }
    }

    /// Return the first texture.
    pub fn first(&self) -> Option<&TextureMapRef> {
        self.textures.first()
    }

    /// Set the first texture.
    pub fn set_first(&mut self, texture: TextureMapRef) {
        if self.textures.is_empty() {
            self.textures.push(texture);
        } else {
            self.textures[0] = texture;
        }
    }

    /// Return the texture at specified position within [0, size()) range.
    pub fn value(&self, index: usize) -> Option<&TextureMapRef> {
        self.textures.get(index)
    }

    /// Set the texture at specified position within [0, size()) range.
    pub fn set_value(&mut self, index: usize, texture: TextureMapRef) {
        if index < self.textures.len() {
            self.textures[index] = texture;
        }
    }

    /// Add a texture to the end of the set.
    pub fn push(&mut self, texture: TextureMapRef) {
        self.textures.push(texture);
    }

    /// Remove all textures.
    pub fn clear(&mut self) {
        self.textures.clear();
    }

    /// Create an iterator over the texture set.
    pub fn iter(&self) -> std::slice::Iter<'_, TextureMapRef> {
        self.textures.iter()
    }
}

impl Default for TextureSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_constructor() {
        let set = TextureSet::new();
        assert!(set.is_empty());
        assert_eq!(set.size(), 0);
        assert_eq!(set.lower(), 0);
        assert_eq!(set.upper(), -1);
    }

    #[test]
    fn test_with_capacity() {
        let set = TextureSet::with_capacity(3);
        assert!(set.is_empty());
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_from_single() {
        let tex = TextureMapRef("texture.jpg".to_string());
        let set = TextureSet::from_single(tex.clone());
        assert!(!set.is_empty());
        assert_eq!(set.size(), 1);
        assert_eq!(set.lower(), 0);
        assert_eq!(set.upper(), 0);
        assert_eq!(set.first().unwrap().0, "texture.jpg");
    }

    #[test]
    fn test_set_first() {
        let mut set = TextureSet::new();
        let tex = TextureMapRef("first.jpg".to_string());
        set.set_first(tex.clone());
        assert_eq!(set.size(), 1);
        assert_eq!(set.first().unwrap().0, "first.jpg");
    }

    #[test]
    fn test_value_and_set_value() {
        let mut set = TextureSet::with_capacity(3);
        set.push(TextureMapRef("tex0.jpg".to_string()));
        set.push(TextureMapRef("tex1.jpg".to_string()));
        set.push(TextureMapRef("tex2.jpg".to_string()));

        assert_eq!(set.value(0).unwrap().0, "tex0.jpg");
        assert_eq!(set.value(1).unwrap().0, "tex1.jpg");
        assert_eq!(set.value(2).unwrap().0, "tex2.jpg");
        assert!(set.value(3).is_none());

        set.set_value(1, TextureMapRef("new_tex1.jpg".to_string()));
        assert_eq!(set.value(1).unwrap().0, "new_tex1.jpg");
    }

    #[test]
    fn test_push_and_size() {
        let mut set = TextureSet::new();
        assert_eq!(set.size(), 0);
        set.push(TextureMapRef("a.jpg".to_string()));
        assert_eq!(set.size(), 1);
        set.push(TextureMapRef("b.jpg".to_string()));
        assert_eq!(set.size(), 2);
    }

    #[test]
    fn test_clear() {
        let mut set = TextureSet::new();
        set.push(TextureMapRef("a.jpg".to_string()));
        set.push(TextureMapRef("b.jpg".to_string()));
        assert_eq!(set.size(), 2);
        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    fn test_iterator_next() {
        let mut set = TextureSet::new();
        set.push(TextureMapRef("tex0.jpg".to_string()));
        set.push(TextureMapRef("tex1.jpg".to_string()));
        set.push(TextureMapRef("tex2.jpg".to_string()));

        let mut iter = TextureSetIterator::new(&set);
        assert!(iter.has_next());
        assert_eq!(iter.next().unwrap().0, "tex0.jpg");
        assert!(iter.has_next());
        assert_eq!(iter.next().unwrap().0, "tex1.jpg");
        assert!(iter.has_next());
        assert_eq!(iter.next().unwrap().0, "tex2.jpg");
        assert!(!iter.has_next());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_lower_upper() {
        let mut set = TextureSet::new();
        assert_eq!(set.lower(), 0);
        assert_eq!(set.upper(), -1);

        set.push(TextureMapRef("a.jpg".to_string()));
        assert_eq!(set.lower(), 0);
        assert_eq!(set.upper(), 0);

        set.push(TextureMapRef("b.jpg".to_string()));
        assert_eq!(set.lower(), 0);
        assert_eq!(set.upper(), 1);
    }

    #[test]
    fn test_iter_trait() {
        let mut set = TextureSet::new();
        set.push(TextureMapRef("a.jpg".to_string()));
        set.push(TextureMapRef("b.jpg".to_string()));

        let textures: Vec<_> = set.iter().map(|t| &t.0).collect();
        assert_eq!(textures.len(), 2);
        assert_eq!(textures[0], "a.jpg");
        assert_eq!(textures[1], "b.jpg");
    }
}
