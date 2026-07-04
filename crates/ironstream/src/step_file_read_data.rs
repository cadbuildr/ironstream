// FILE: step_file_read_data.rs
// occt: StepFile_ReadData

//! Data structures and tools for collecting and storing STEP file data.

use std::collections::VecDeque;

/// Parameter type from STEP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    Text,
    Ident,
    Integer,
    Real,
    Enum,
    List,
}

impl ParamType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ParamType::Text => "Text",
            ParamType::Ident => "Ident",
            ParamType::Integer => "Integer",
            ParamType::Real => "Real",
            ParamType::Enum => "Enum",
            ParamType::List => "List",
        }
    }
}

/// An argument in a STEP record
#[derive(Debug, Clone)]
pub struct Argument {
    value: String,
    param_type: ParamType,
}

impl Argument {
    pub fn new(value: String, param_type: ParamType) -> Self {
        Self { value, param_type }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn param_type(&self) -> ParamType {
        self.param_type
    }
}

/// A STEP record (entity description)
#[derive(Debug, Clone)]
pub struct Record {
    ident: String,
    record_type: String,
    arguments: Vec<Argument>,
}

impl Record {
    pub fn new(ident: String, record_type: String) -> Self {
        Self {
            ident,
            record_type,
            arguments: Vec::new(),
        }
    }

    pub fn ident(&self) -> &str {
        &self.ident
    }

    pub fn record_type(&self) -> &str {
        &self.record_type
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }

    pub fn add_argument(&mut self, arg: Argument) {
        self.arguments.push(arg);
    }

    pub fn num_arguments(&self) -> usize {
        self.arguments.len()
    }
}

/// ReadData tool for STEP file parsing
#[derive(Debug)]
pub struct StepFileReadData {
    records: Vec<Record>,
    current_record: Option<Record>,
    current_text: String,
    num_records: usize,
    num_head: usize,
    mode_print: i32,
    errors: VecDeque<String>,
}

impl StepFileReadData {
    /// Create a new ReadData instance
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            current_record: None,
            current_text: String::new(),
            num_records: 0,
            num_head: 0,
            mode_print: 0,
            errors: VecDeque::new(),
        }
    }

    /// Create new text from parsing
    pub fn create_new_text(&mut self, text: &str) {
        self.current_text = text.to_string();
    }

    /// Create a new record with current text as identifier
    pub fn record_ident(&mut self) {
        let ident = self.current_text.clone();
        self.current_record = Some(Record::new(ident, String::new()));
    }

    /// Set the type of the current record
    pub fn record_type(&mut self) {
        if let Some(ref mut record) = self.current_record {
            record.record_type = self.current_text.clone();
        }
    }

    /// Finish current record and add to records list
    pub fn record_new_entity(&mut self) {
        if let Some(record) = self.current_record.take() {
            self.records.push(record);
            self.num_records += 1;
        }
    }

    /// Create a new argument with current text
    pub fn create_new_arg(&mut self, param_type: ParamType) {
        let arg = Argument::new(self.current_text.clone(), param_type);
        if let Some(ref mut record) = self.current_record {
            record.add_argument(arg);
        }
    }

    /// Get description of current argument
    pub fn get_arg_description(&self) -> Option<(ParamType, String)> {
        if let Some(ref record) = self.current_record {
            if let Some(arg) = record.arguments.last() {
                return Some((arg.param_type, arg.value.clone()));
            }
        }
        None
    }

    /// Get description of current record
    pub fn get_record_description(&self) -> Option<(String, String, usize)> {
        if let Some(ref record) = self.current_record {
            return Some((
                record.ident.clone(),
                record.record_type.clone(),
                record.arguments.len(),
            ));
        }
        None
    }

    /// Get file counters
    pub fn get_file_nb(&self) -> (usize, usize, usize) {
        (self.num_head, self.num_records, 1) // 1 page for simplicity
    }

    /// Clear records and data
    pub fn clear_recorder(&mut self, mode: i32) {
        match mode {
            1 => {
                self.records.clear();
                self.num_records = 0;
            }
            2 => {
                self.current_text.clear();
            }
            3 => {
                self.records.clear();
                self.current_text.clear();
                self.errors.clear();
                self.num_records = 0;
            }
            _ => {}
        }
    }

    /// Move to next record
    pub fn next_record(&mut self) {
        // Placeholder for iteration
    }

    /// Add error message
    pub fn add_error(&mut self, error_message: &str) {
        self.errors.push_back(error_message.to_string());
    }

    /// Get last error message
    pub fn get_last_error(&self) -> Option<&str> {
        self.errors.back().map(|s| s.as_str())
    }

    /// Set print mode
    pub fn set_mode_print(&mut self, mode: i32) {
        self.mode_print = mode;
    }

    /// Get print mode
    pub fn get_mode_print(&self) -> i32 {
        self.mode_print
    }

    /// Get number of records
    pub fn get_nb_record(&self) -> usize {
        self.num_records
    }

    /// Get all records
    pub fn records(&self) -> &[Record] {
        &self.records
    }
}

impl Default for StepFileReadData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = StepFileReadData::new();
        assert_eq!(data.get_nb_record(), 0);
        assert_eq!(data.get_mode_print(), 0);
    }

    #[test]
    fn test_param_type() {
        assert_eq!(ParamType::Text.as_str(), "Text");
        assert_eq!(ParamType::Ident.as_str(), "Ident");
        assert_eq!(ParamType::Integer.as_str(), "Integer");
    }

    #[test]
    fn test_create_new_text() {
        let mut data = StepFileReadData::new();
        data.create_new_text("#123");
        assert_eq!(data.current_text, "#123");
    }

    #[test]
    fn test_record_ident() {
        let mut data = StepFileReadData::new();
        data.create_new_text("#1");
        data.record_ident();
        assert!(data.current_record.is_some());
    }

    #[test]
    fn test_record_type() {
        let mut data = StepFileReadData::new();
        data.create_new_text("#1");
        data.record_ident();
        data.create_new_text("FACE");
        data.record_type();
        let desc = data.get_record_description();
        assert!(desc.is_some());
        let (_, rec_type, _) = desc.unwrap();
        assert_eq!(rec_type, "FACE");
    }

    #[test]
    fn test_create_new_arg() {
        let mut data = StepFileReadData::new();
        data.create_new_text("#1");
        data.record_ident();
        data.create_new_text("value");
        data.create_new_arg(ParamType::Text);
        let desc = data.get_record_description();
        assert!(desc.is_some());
        let (_, _, num_args) = desc.unwrap();
        assert_eq!(num_args, 1);
    }

    #[test]
    fn test_record_new_entity() {
        let mut data = StepFileReadData::new();
        data.create_new_text("#1");
        data.record_ident();
        data.record_new_entity();
        assert_eq!(data.get_nb_record(), 1);
    }

    #[test]
    fn test_add_error() {
        let mut data = StepFileReadData::new();
        data.add_error("Test error");
        assert_eq!(data.get_last_error(), Some("Test error"));
    }

    #[test]
    fn test_set_mode_print() {
        let mut data = StepFileReadData::new();
        data.set_mode_print(2);
        assert_eq!(data.get_mode_print(), 2);
    }

    #[test]
    fn test_argument() {
        let arg = Argument::new("test_value".to_string(), ParamType::Ident);
        assert_eq!(arg.value(), "test_value");
        assert_eq!(arg.param_type(), ParamType::Ident);
    }

    #[test]
    fn test_record() {
        let mut rec = Record::new("#1".to_string(), "FACE".to_string());
        rec.add_argument(Argument::new("arg1".to_string(), ParamType::Text));
        assert_eq!(rec.ident(), "#1");
        assert_eq!(rec.record_type(), "FACE");
        assert_eq!(rec.num_arguments(), 1);
    }

    #[test]
    fn test_clear_recorder() {
        let mut data = StepFileReadData::new();
        data.create_new_text("#1");
        data.record_ident();
        data.record_new_entity();
        assert_eq!(data.get_nb_record(), 1);
        data.clear_recorder(1);
        assert_eq!(data.get_nb_record(), 0);
    }
}
