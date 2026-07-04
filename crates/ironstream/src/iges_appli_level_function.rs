// FILE: iges_appli_level_function.rs
// occt: IGESAppli_LevelFunction

/// Defines level functions in PCB design.
#[derive(Clone, Debug)]
pub struct IgesAppliLevelFunction {
    level_id: i32,
    function_name: String,
}

impl IgesAppliLevelFunction {
    pub fn new() -> Self {
        Self {
            level_id: 0,
            function_name: String::new(),
        }
    }

    pub fn init(&mut self, id: i32, name: String) {
        self.level_id = id;
        self.function_name = name;
    }

    pub fn level_id(&self) -> i32 {
        self.level_id
    }

    pub fn function_name(&self) -> &str {
        &self.function_name
    }
}

impl Default for IgesAppliLevelFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut func = IgesAppliLevelFunction::new();
        func.init(1, "top_copper".to_string());

        assert_eq!(func.level_id(), 1);
        assert_eq!(func.function_name(), "top_copper");
    }
}
