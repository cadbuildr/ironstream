// FILE: interface_array1_of_file_parameter.rs
// occt: Interface_Array1OfFileParameter

/// Deprecated alias for NCollection_Array1<Interface_FileParameter>.
/// Maintains backward compatibility. Use Vec directly in new code.
pub struct InterfaceArray1OfFileParameter {
    items: Vec<u32>, // Placeholder for Interface_FileParameter (opaque type)
    lower: usize,    // OCCT array lower bound
}

impl InterfaceArray1OfFileParameter {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        Self {
            items: vec![0; size],
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

    pub fn set_value(&mut self, index: usize, value: u32) {
        if index >= self.lower && index <= self.upper() {
            let idx = index - self.lower;
            if idx < self.items.len() {
                self.items[idx] = value;
            }
        }
    }

    pub fn value_at(&self, index: usize) -> Option<u32> {
        if index >= self.lower && index <= self.upper() {
            let idx = index - self.lower;
            if idx < self.items.len() {
                return Some(self.items[idx]);
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
        let arr = InterfaceArray1OfFileParameter::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_array_set_and_get() {
        let mut arr = InterfaceArray1OfFileParameter::new(1, 3);
        arr.set_value(1, 10);
        arr.set_value(2, 20);
        arr.set_value(3, 30);

        assert_eq!(arr.value_at(1), Some(10));
        assert_eq!(arr.value_at(2), Some(20));
        assert_eq!(arr.value_at(3), Some(30));
    }

    #[test]
    fn test_array_bounds() {
        let mut arr = InterfaceArray1OfFileParameter::new(0, 2);
        arr.set_value(0, 100);
        arr.set_value(1, 200);
        arr.set_value(2, 300);

        assert_eq!(arr.lower(), 0);
        assert_eq!(arr.upper(), 2);
        assert_eq!(arr.length(), 3);
    }

    #[test]
    fn test_array_out_of_bounds() {
        let arr = InterfaceArray1OfFileParameter::new(1, 3);
        assert_eq!(arr.value_at(0), None);
        assert_eq!(arr.value_at(4), None);
    }

    #[test]
    fn test_empty_array() {
        let arr = InterfaceArray1OfFileParameter::new(1, 0);
        assert!(arr.is_empty());
        assert_eq!(arr.length(), 0);
    }
}
