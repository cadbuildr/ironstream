// FILE: step_file_read.rs
// occt: StepFile_Read

//! STEP file reading functions.

use std::io::{BufRead, BufReader, Read};

/// STEP data model placeholder
#[derive(Debug, Clone)]
pub struct StepModel {
    entities: Vec<String>,
}

impl StepModel {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: String) {
        self.entities.push(entity);
    }

    pub fn entities(&self) -> &[String] {
        &self.entities
    }

    pub fn num_entities(&self) -> usize {
        self.entities.len()
    }
}

impl Default for StepModel {
    fn default() -> Self {
        Self::new()
    }
}

/// STEP protocol placeholder
#[derive(Debug, Clone)]
pub struct StepProtocol {
    name: String,
}

impl StepProtocol {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Error codes for reading
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadStatus {
    Success = 0,
    StreamFail = -1,
    ParseError = 1,
}

impl ReadStatus {
    pub fn to_code(&self) -> i32 {
        match self {
            ReadStatus::Success => 0,
            ReadStatus::StreamFail => -1,
            ReadStatus::ParseError => 1,
        }
    }

    pub fn from_code(code: i32) -> Option<ReadStatus> {
        match code {
            0 => Some(ReadStatus::Success),
            -1 => Some(ReadStatus::StreamFail),
            1 => Some(ReadStatus::ParseError),
            _ => None,
        }
    }
}

/// Interrupt handler for errors
pub fn file_interrupt(error_message: &str, is_fail: bool) {
    if is_fail {
        eprintln!("FAIL: {}", error_message);
    } else {
        eprintln!("TRACE: {}", error_message);
    }
}

/// Read STEP file or stream
/// Returns 0 on success, -1 if stream fails, 1 in case of parsing error
pub fn file_read<R: Read>(
    name: &str,
    reader: R,
    model: &mut StepModel,
    _protocol: &StepProtocol,
) -> i32 {
    let buf_reader = BufReader::new(reader);
    let mut line_count = 0;
    let mut header_section = false;
    let mut data_section = false;

    for line_result in buf_reader.lines() {
        match line_result {
            Ok(line) => {
                let trimmed = line.trim();
                line_count += 1;

                // Parse STEP file structure
                if trimmed == "HEADER;" {
                    header_section = true;
                } else if trimmed == "ENDSEC;" {
                    if header_section {
                        header_section = false;
                        data_section = true;
                    } else if data_section {
                        data_section = false;
                    }
                } else if trimmed == "DATA;" {
                    data_section = true;
                } else if data_section && !trimmed.is_empty() && !trimmed.starts_with('#') {
                    // Add entity to model (simplified parsing)
                    model.add_entity(trimmed.to_string());
                }
            }
            Err(_) => {
                file_interrupt(&format!("Stream read error in file: {}", name), true);
                return ReadStatus::StreamFail.to_code();
            }
        }
    }

    if line_count == 0 {
        file_interrupt(&format!("Empty file: {}", name), true);
        return ReadStatus::ParseError.to_code();
    }

    ReadStatus::Success.to_code()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_step_model() {
        let mut model = StepModel::new();
        model.add_entity("entity1".to_string());
        model.add_entity("entity2".to_string());
        assert_eq!(model.num_entities(), 2);
    }

    #[test]
    fn test_step_protocol() {
        let proto = StepProtocol::new("AP214".to_string());
        assert_eq!(proto.name(), "AP214");
    }

    #[test]
    fn test_read_status() {
        assert_eq!(ReadStatus::Success.to_code(), 0);
        assert_eq!(ReadStatus::StreamFail.to_code(), -1);
        assert_eq!(ReadStatus::ParseError.to_code(), 1);
    }

    #[test]
    fn test_read_status_from_code() {
        assert_eq!(ReadStatus::from_code(0), Some(ReadStatus::Success));
        assert_eq!(ReadStatus::from_code(-1), Some(ReadStatus::StreamFail));
        assert_eq!(ReadStatus::from_code(1), Some(ReadStatus::ParseError));
    }

    #[test]
    fn test_file_read_valid() {
        let mut model = StepModel::new();
        let protocol = StepProtocol::new("AP214".to_string());
        let content = b"HEADER;\nENDSEC;\nDATA;\n#1 = ENTITY1;\nENDSEC;";
        let cursor = Cursor::new(content);
        let status = file_read("test.stp", cursor, &mut model, &protocol);
        assert_eq!(status, 0);
    }

    #[test]
    fn test_file_read_empty() {
        let mut model = StepModel::new();
        let protocol = StepProtocol::new("AP214".to_string());
        let content = b"";
        let cursor = Cursor::new(content);
        let status = file_read("test.stp", cursor, &mut model, &protocol);
        assert_eq!(status, 1); // ParseError
    }

    #[test]
    fn test_file_interrupt() {
        file_interrupt("Test error message", true);
        file_interrupt("Test info message", false);
        // Just verify no panic
        assert!(true);
    }
}
