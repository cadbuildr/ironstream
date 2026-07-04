// FILE: iges_data_iges_type.rs
// occt: IGESData_IGESType

//! Type and Form information for IGES entities.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IGESType {
    type_num: i32,
    form_num: i32,
}

impl IGESType {
    pub fn new(type_num: i32, form_num: i32) -> Self {
        IGESType { type_num, form_num }
    }

    pub fn type_num(&self) -> i32 {
        self.type_num
    }

    pub fn form_num(&self) -> i32 {
        self.form_num
    }

    pub fn is_zero(&self) -> bool {
        self.type_num == 0 && self.form_num == 0
    }
}

impl Default for IGESType {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = IGESType::new(102, 1);
        assert_eq!(t.type_num(), 102);
        assert_eq!(t.form_num(), 1);
    }

    #[test]
    fn test_is_zero() {
        let t1 = IGESType::new(0, 0);
        assert!(t1.is_zero());

        let t2 = IGESType::new(1, 0);
        assert!(!t2.is_zero());
    }

    #[test]
    fn test_default() {
        let t = IGESType::default();
        assert_eq!(t.type_num(), 0);
        assert_eq!(t.form_num(), 0);
        assert!(t.is_zero());
    }

    #[test]
    fn test_equality() {
        let t1 = IGESType::new(100, 5);
        let t2 = IGESType::new(100, 5);
        let t3 = IGESType::new(100, 6);

        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }
}
