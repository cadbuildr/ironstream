// FILE: interface_array1_of_h_ascii_string.rs
// occt: Interface_Array1OfHAsciiString

/// Deprecated alias for NCollection_Array1<opencascade::handle<TCollection_HAsciiString>>.
/// Maintains backward compatibility. Use Vec directly in new code.
pub struct InterfaceArray1OfHAsciiString {
    items: Vec<String>, // Placeholder for TCollection_HAsciiString
    lower: usize,       // OCCT array lower bound
}

impl InterfaceArray1OfHAsciiString {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        Self {
            items: vec![String::new(); size],
            lower,
        }
    }

    pub fn length(&self) -> usize {
        self.items.len()
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> usize {
        if self.items.is_empty() {
            self.lower - 1
        } else {
            self.lower + self.items.len() - 1
        }
    }

    pub fn set_value(&mut self, index: usize, value: String) {
        if index >= self.lower && index <= self.upper() {
            let idx = index - self.lower;
            if idx < self.items.len() {
                self.items[idx] = value;
            }
        }
    }

    pub fn value_at(&self, index: usize) -> Option<String> {
        if index >= self.lower && index <= self.upper() {
            let idx = index - self.lower;
            if idx < self.items.len() {
                return Some(self.items[idx].clone());
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_construction() {
        let arr = InterfaceArray1OfHAsciiString::new(1, 3);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 3);
        assert_eq!(arr.length(), 3);
    }

    #[test]
    fn test_array_set_and_get() {
        let mut arr = InterfaceArray1OfHAsciiString::new(1, 3);
        arr.set_value(1, "hello".to_string());
        arr.set_value(2, "world".to_string());
        arr.set_value(3, "test".to_string());

        assert_eq!(arr.value_at(1), Some("hello".to_string()));
        assert_eq!(arr.value_at(2), Some("world".to_string()));
        assert_eq!(arr.value_at(3), Some("test".to_string()));
    }

    #[test]
    fn test_array_bounds() {
        let mut arr = InterfaceArray1OfHAsciiString::new(0, 2);
        arr.set_value(0, "a".to_string());
        arr.set_value(1, "b".to_string());
        arr.set_value(2, "c".to_string());

        assert_eq!(arr.lower(), 0);
        assert_eq!(arr.upper(), 2);
        assert_eq!(arr.length(), 3);
    }

    #[test]
    fn test_array_out_of_bounds() {
        let arr = InterfaceArray1OfHAsciiString::new(1, 3);
        assert_eq!(arr.value_at(0), None);
        assert_eq!(arr.value_at(4), None);
    }

    #[test]
    fn test_empty_array() {
        let arr = InterfaceArray1OfHAsciiString::new(1, 0);
        assert!(arr.is_empty());
        assert_eq!(arr.length(), 0);
    }
}
