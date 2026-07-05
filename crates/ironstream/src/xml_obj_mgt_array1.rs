// FILE: xml_obj_mgt_array1.rs
// occt: XmlObjMgt_Array1

/// XmlObjMgt_Array1 represents a unidimensional array of fixed size known at run time.
/// The range of the index is user defined.
pub struct XmlObjMgt_Array1 {
    data: Vec<String>,
    lower: i32,
    upper: i32,
}

impl XmlObjMgt_Array1 {
    /// Create an array with lower bound Low and upper bound Up.
    /// Panics if Up < Low.
    pub fn new(lower: i32, upper: i32) -> Self {
        if upper < lower {
            panic!("Array range error: upper < lower");
        }
        let size = (upper - lower + 1) as usize;
        XmlObjMgt_Array1 {
            data: vec![String::new(); size],
            lower,
            upper,
        }
    }

    /// Returns the number of elements.
    pub fn length(&self) -> i32 {
        self.upper - self.lower + 1
    }

    /// Returns the lower bound.
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// Returns the upper bound.
    pub fn upper(&self) -> i32 {
        self.upper
    }

    /// Set the Indexth element to Value.
    pub fn set_value(&mut self, index: i32, value: String) {
        if index < self.lower || index > self.upper {
            panic!("Array index out of bounds");
        }
        let pos = (index - self.lower) as usize;
        self.data[pos] = value;
    }

    /// Get the Indexth element.
    pub fn value(&self, index: i32) -> &str {
        if index < self.lower || index > self.upper {
            panic!("Array index out of bounds");
        }
        let pos = (index - self.lower) as usize;
        &self.data[pos]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let arr = XmlObjMgt_Array1::new(1, 10);
        assert_eq!(arr.length(), 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
    }

    #[test]
    fn test_set_and_get_value() {
        let mut arr = XmlObjMgt_Array1::new(0, 5);
        arr.set_value(3, "test".to_string());
        assert_eq!(arr.value(3), "test");
    }

    #[test]
    fn test_negative_indices() {
        let mut arr = XmlObjMgt_Array1::new(-5, 5);
        arr.set_value(-2, "neg".to_string());
        assert_eq!(arr.value(-2), "neg");
        assert_eq!(arr.length(), 11);
    }

    #[test]
    #[should_panic]
    fn test_invalid_range() {
        XmlObjMgt_Array1::new(10, 5);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_access() {
        let arr = XmlObjMgt_Array1::new(0, 5);
        let _ = arr.value(10);
    }
}
