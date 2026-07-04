// FILE: moni_tool_case_data.rs
// occt: MoniTool_CaseData

use std::collections::HashMap;

/// Records data attached to a case to be exploited
pub struct MoniToolCaseData {
    caseid: String,
    name: String,
    data: Vec<(String, DataValue)>,
}

#[derive(Clone, Debug)]
enum DataValue {
    Integer(i32),
    Real(f64),
    Text(String),
}

impl MoniToolCaseData {
    pub fn new(caseid: &str, name: &str) -> Self {
        MoniToolCaseData {
            caseid: caseid.to_string(),
            name: name.to_string(),
            data: Vec::new(),
        }
    }

    pub fn set_case_id(&mut self, caseid: &str) {
        self.caseid = caseid.to_string();
    }

    pub fn case_id(&self) -> &str {
        &self.caseid
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_integer(&mut self, val: i32) {
        self.data.push((String::new(), DataValue::Integer(val)));
    }

    pub fn add_real(&mut self, val: f64) {
        self.data.push((String::new(), DataValue::Real(val)));
    }

    pub fn add_text(&mut self, val: &str) {
        self.data.push((String::new(), DataValue::Text(val.to_string())));
    }

    pub fn nb_data(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for MoniToolCaseData {
    fn default() -> Self {
        Self::new("", "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let case_data = MoniToolCaseData::new("case1", "name1");
        assert_eq!(case_data.case_id(), "case1");
        assert_eq!(case_data.name(), "name1");
    }

    #[test]
    fn test_add_integer() {
        let mut case_data = MoniToolCaseData::new("case", "name");
        case_data.add_integer(42);
        assert_eq!(case_data.nb_data(), 1);
    }

    #[test]
    fn test_add_real() {
        let mut case_data = MoniToolCaseData::new("case", "name");
        case_data.add_real(3.14);
        assert_eq!(case_data.nb_data(), 1);
    }

    #[test]
    fn test_clear() {
        let mut case_data = MoniToolCaseData::new("case", "name");
        case_data.add_integer(1);
        case_data.clear();
        assert_eq!(case_data.nb_data(), 0);
    }
}
