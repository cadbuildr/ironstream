// FILE: step_shape_csg_select.rs
// occt: StepShape_CsgSelect

//! Representation of STEP entity CsgSelect

use super::step_shape_csg_primitive::CsgPrimitive;

#[derive(Clone, Debug)]
pub enum CsgSelectContent {
    BooleanResult(String),
    CsgPrimitive(CsgPrimitive),
}

#[derive(Clone, Debug)]
pub struct CsgSelect {
    content: Option<CsgSelectContent>,
    type_of_content: i32,
}

impl CsgSelect {
    /// Returns a CsgSelect SelectType
    pub fn new() -> Self {
        CsgSelect {
            content: None,
            type_of_content: 0,
        }
    }

    /// Set TypeOfContent
    pub fn set_type_of_content(&mut self, type_of_content: i32) {
        self.type_of_content = type_of_content;
    }

    /// Returns TypeOfContent
    pub fn type_of_content(&self) -> i32 {
        self.type_of_content
    }

    /// Returns value as BooleanResult (None if another type)
    pub fn boolean_result(&self) -> Option<&str> {
        if let Some(CsgSelectContent::BooleanResult(br)) = &self.content {
            Some(br)
        } else {
            None
        }
    }

    /// Set BooleanResult
    pub fn set_boolean_result(&mut self, boolean_result: String) {
        self.content = Some(CsgSelectContent::BooleanResult(boolean_result));
        self.type_of_content = 1;
    }

    /// Returns value as CsgPrimitive (None if another type)
    pub fn csg_primitive(&self) -> Option<&CsgPrimitive> {
        if let Some(CsgSelectContent::CsgPrimitive(cp)) = &self.content {
            Some(cp)
        } else {
            None
        }
    }

    /// Set CsgPrimitive
    pub fn set_csg_primitive(&mut self, csg_primitive: CsgPrimitive) {
        self.content = Some(CsgSelectContent::CsgPrimitive(csg_primitive));
        self.type_of_content = 2;
    }
}

impl Default for CsgSelect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let select = CsgSelect::new();
        assert_eq!(select.type_of_content(), 0);
        assert!(select.boolean_result().is_none());
        assert!(select.csg_primitive().is_none());
    }

    #[test]
    fn test_set_type_of_content() {
        let mut select = CsgSelect::new();
        select.set_type_of_content(1);
        assert_eq!(select.type_of_content(), 1);
    }

    #[test]
    fn test_boolean_result() {
        let mut select = CsgSelect::new();
        select.set_boolean_result("bool1".to_string());
        assert_eq!(select.boolean_result(), Some("bool1"));
        assert_eq!(select.type_of_content(), 1);
    }

    #[test]
    fn test_csg_primitive() {
        let mut select = CsgSelect::new();
        let prim = CsgPrimitive::Sphere("sphere1".to_string());
        select.set_csg_primitive(prim);
        assert!(select.csg_primitive().is_some());
        assert_eq!(select.type_of_content(), 2);
    }
}
