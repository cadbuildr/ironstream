// FILE: ldom_character_data.rs
// occt: LDOM_CharacterData

/// Represents character data in a DOM node.
#[derive(Clone, Default)]
pub struct LDOMCharacterData {
    data: String,
    myLength: i32,
}

impl LDOMCharacterData {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMCharacterData {
            data: String::new(),
            myLength: -1,
        }
    }

    /// Constructor with initial data
    pub fn with_data(data: &str) -> Self {
        let len = data.len() as i32;
        LDOMCharacterData {
            data: data.to_string(),
            myLength: len,
        }
    }

    /// Copy constructor
    pub fn from_other(other: &LDOMCharacterData) -> Self {
        LDOMCharacterData {
            data: other.data.clone(),
            myLength: other.myLength,
        }
    }

    /// Nullify the character data
    pub fn set_null(&mut self) {
        self.data.clear();
        self.myLength = -1;
    }

    /// Get the data
    pub fn get_data(&self) -> &str {
        &self.data
    }

    /// Set the data
    pub fn set_data(&mut self, data: &str) {
        self.data = data.to_string();
        self.myLength = data.len() as i32;
    }

    /// Get the length of the string
    pub fn get_length(&self) -> i32 {
        if self.myLength < 0 {
            self.data.len() as i32
        } else {
            self.myLength
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_character_data() {
        let cd = LDOMCharacterData::new();
        assert_eq!(cd.get_data(), "");
    }

    #[test]
    fn test_with_data() {
        let cd = LDOMCharacterData::with_data("hello world");
        assert_eq!(cd.get_data(), "hello world");
        assert_eq!(cd.get_length(), 11);
    }

    #[test]
    fn test_copy_constructor() {
        let cd1 = LDOMCharacterData::with_data("test");
        let cd2 = LDOMCharacterData::from_other(&cd1);
        assert_eq!(cd2.get_data(), "test");
        assert_eq!(cd2.get_length(), 4);
    }

    #[test]
    fn test_set_data() {
        let mut cd = LDOMCharacterData::new();
        cd.set_data("new data");
        assert_eq!(cd.get_data(), "new data");
        assert_eq!(cd.get_length(), 8);
    }

    #[test]
    fn test_get_length() {
        let cd = LDOMCharacterData::with_data("abc");
        assert_eq!(cd.get_length(), 3);
    }

    #[test]
    fn test_nullify() {
        let mut cd = LDOMCharacterData::with_data("data");
        cd.set_null();
        assert_eq!(cd.get_data(), "");
        assert_eq!(cd.myLength, -1);
    }
}
