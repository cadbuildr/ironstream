// FILE: iges_data_dir_part.rs
// occt: IGESData_DirPart

//! Literal/numeric description of an entity's directory section, taken from file.
//! Stores 17 integer values and reserved/label/subscript strings.

/// IGESData_DirPart represents the directory section of an IGES entity as read from file
#[derive(Clone, Debug)]
pub struct DirPart {
    vals: [i32; 17],
    res1: String,
    res2: String,
    label: String,
    subscript: String,
}

impl DirPart {
    /// Creates an empty DirPart, ready to be filled by init
    pub fn new() -> Self {
        DirPart {
            vals: [0; 17],
            res1: String::new(),
            res2: String::new(),
            label: String::new(),
            subscript: String::new(),
        }
    }

    /// Fills DirPart with consistent data read from file
    pub fn init(
        &mut self,
        i1: i32,
        i2: i32,
        i3: i32,
        i4: i32,
        i5: i32,
        i6: i32,
        i7: i32,
        i8: i32,
        i9: i32,
        i10: i32,
        i11: i32,
        i12: i32,
        i13: i32,
        i14: i32,
        i15: i32,
        i16: i32,
        i17: i32,
        res1: &str,
        res2: &str,
        label: &str,
        subscript: &str,
    ) {
        self.vals = [i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13, i14, i15, i16, i17];
        self.res1 = res1.to_string();
        self.res2 = res2.to_string();
        self.label = label.to_string();
        self.subscript = subscript.to_string();
    }

    /// Returns values recorded in DirPart
    pub fn values(
        &self,
    ) -> (
        [i32; 17],
        String,
        String,
        String,
        String,
    ) {
        (
            self.vals,
            self.res1.clone(),
            self.res2.clone(),
            self.label.clone(),
            self.subscript.clone(),
        )
    }

    /// Returns individual value at given index
    pub fn value_at(&self, index: usize) -> Option<i32> {
        if index < 17 {
            Some(self.vals[index])
        } else {
            None
        }
    }

    /// Sets individual value at given index
    pub fn set_value_at(&mut self, index: usize, val: i32) -> bool {
        if index < 17 {
            self.vals[index] = val;
            true
        } else {
            false
        }
    }

    /// Returns type and form info (indices 0 and 1)
    pub fn type_and_form(&self) -> (i32, i32) {
        (self.vals[0], self.vals[1])
    }

    /// Returns reserved string 1
    pub fn res1(&self) -> &str {
        &self.res1
    }

    /// Returns reserved string 2
    pub fn res2(&self) -> &str {
        &self.res2
    }

    /// Returns label string
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns subscript string
    pub fn subscript(&self) -> &str {
        &self.subscript
    }
}

impl Default for DirPart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dp = DirPart::new();
        assert_eq!(dp.vals, [0; 17]);
        assert_eq!(dp.res1(), "");
        assert_eq!(dp.label(), "");
    }

    #[test]
    fn test_init() {
        let mut dp = DirPart::new();
        dp.init(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
            "RES1", "RES2", "LAB", "SUB",
        );

        assert_eq!(dp.vals[0], 1);
        assert_eq!(dp.vals[16], 17);
        assert_eq!(dp.res1(), "RES1");
        assert_eq!(dp.res2(), "RES2");
        assert_eq!(dp.label(), "LAB");
        assert_eq!(dp.subscript(), "SUB");
    }

    #[test]
    fn test_type_and_form() {
        let mut dp = DirPart::new();
        dp.vals[0] = 128;
        dp.vals[1] = 0;
        let (t, f) = dp.type_and_form();
        assert_eq!(t, 128);
        assert_eq!(f, 0);
    }

    #[test]
    fn test_value_at() {
        let mut dp = DirPart::new();
        dp.vals[5] = 42;
        assert_eq!(dp.value_at(5), Some(42));
        assert_eq!(dp.value_at(17), None);
        assert_eq!(dp.value_at(100), None);
    }

    #[test]
    fn test_set_value_at() {
        let mut dp = DirPart::new();
        assert!(dp.set_value_at(3, 55));
        assert_eq!(dp.value_at(3), Some(55));

        assert!(!dp.set_value_at(20, 99));
    }

    #[test]
    fn test_values() {
        let mut dp = DirPart::new();
        dp.init(
            10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170,
            "A", "B", "C", "D",
        );

        let (vals, r1, r2, lab, sub) = dp.values();
        assert_eq!(vals[0], 10);
        assert_eq!(vals[16], 170);
        assert_eq!(r1, "A");
        assert_eq!(r2, "B");
        assert_eq!(lab, "C");
        assert_eq!(sub, "D");
    }

    #[test]
    fn test_default() {
        let dp = DirPart::default();
        assert_eq!(dp.vals, [0; 17]);
    }
}
