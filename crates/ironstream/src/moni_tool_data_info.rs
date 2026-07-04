// FILE: moni_tool_data_info.rs
// occt: MoniTool_DataInfo

/// Information about data
pub struct MoniToolDataInfo {
    typename: String,
    value: String,
}

impl MoniToolDataInfo {
    pub fn new(typename: &str, value: &str) -> Self {
        MoniToolDataInfo {
            typename: typename.to_string(),
            value: value.to_string(),
        }
    }

    pub fn typename(&self) -> &str {
        &self.typename
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Default for MoniToolDataInfo {
    fn default() -> Self {
        MoniToolDataInfo {
            typename: String::new(),
            value: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let info = MoniToolDataInfo::new("int", "42");
        assert_eq!(info.typename(), "int");
        assert_eq!(info.value(), "42");
    }
}
