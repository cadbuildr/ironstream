// FILE: iges_data_undefined_entity.rs
// occt: IGESData_UndefinedEntity

//! Undefined IGES entity - used for entities without specific class definitions.

#[derive(Clone, Debug)]
pub struct UndefinedEntity {
    type_num: i32,
    form_num: i32,
    parameters: Vec<String>,
}

impl UndefinedEntity {
    pub fn new(type_num: i32, form_num: i32) -> Self {
        UndefinedEntity {
            type_num,
            form_num,
            parameters: Vec::new(),
        }
    }

    pub fn type_num(&self) -> i32 {
        self.type_num
    }

    pub fn form_num(&self) -> i32 {
        self.form_num
    }

    pub fn add_parameter(&mut self, param: &str) {
        self.parameters.push(param.to_string());
    }

    pub fn parameters(&self) -> &[String] {
        &self.parameters
    }

    pub fn param_count(&self) -> usize {
        self.parameters.len()
    }
}

impl Default for UndefinedEntity {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = UndefinedEntity::new(999, 5);
        assert_eq!(entity.type_num(), 999);
        assert_eq!(entity.form_num(), 5);
        assert_eq!(entity.param_count(), 0);
    }

    #[test]
    fn test_add_parameter() {
        let mut entity = UndefinedEntity::new(100, 0);
        entity.add_parameter("param1");
        entity.add_parameter("param2");
        assert_eq!(entity.param_count(), 2);
        assert_eq!(entity.parameters(), &["param1".to_string(), "param2".to_string()]);
    }

    #[test]
    fn test_default() {
        let entity = UndefinedEntity::default();
        assert_eq!(entity.type_num(), 0);
        assert_eq!(entity.form_num(), 0);
    }
}
