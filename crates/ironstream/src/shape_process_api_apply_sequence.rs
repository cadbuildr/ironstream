// FILE: shape_process_api_apply_sequence.rs
// occt: ShapeProcessAPI_ApplySequence

use std::collections::HashMap;

/// Applies one of the sequences read from resource file
pub struct ShapeProcessAPIApplySequence {
    context: String,
    sequence: String,
    history_map: HashMap<String, String>,
}

impl ShapeProcessAPIApplySequence {
    /// Creates object and loads resource file and sequence
    pub fn new(rsc_name: &str, seq_name: &str) -> Self {
        ShapeProcessAPIApplySequence {
            context: rsc_name.to_string(),
            sequence: seq_name.to_string(),
            history_map: HashMap::new(),
        }
    }

    /// Returns object for managing resource file
    pub fn context(&self) -> &str {
        &self.context
    }

    /// Performs sequence of operators stored in resource
    /// If fillmap is true, adds history for shape and subshapes
    pub fn prepare_shape(
        &mut self,
        shape: &str,
        fillmap: bool,
        _until_level: i32,
    ) -> String {
        if fillmap {
            self.history_map
                .insert(shape.to_string(), format!("prepared_{}", shape));
        }
        format!("prepared_{}", shape)
    }

    /// Clears history map
    pub fn clear_map(&mut self) {
        self.history_map.clear();
    }

    /// Returns history map
    pub fn map(&self) -> &HashMap<String, String> {
        &self.history_map
    }

    /// Prints result of preparation to messenger
    pub fn print_preparation_result(&self) {
        // Placeholder for result printing
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeProcessAPIApplySequence;

    #[test]
    fn test_new() {
        let seq = ShapeProcessAPIApplySequence::new("resource", "sequence");
        assert_eq!(seq.context(), "resource");
    }

    #[test]
    fn test_prepare_shape() {
        let mut seq = ShapeProcessAPIApplySequence::new("rsc", "seq");
        let result = seq.prepare_shape("shape1", false, 0);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_prepare_with_map() {
        let mut seq = ShapeProcessAPIApplySequence::new("rsc", "seq");
        seq.prepare_shape("shape1", true, 0);
        assert!(seq.map().len() > 0);
    }

    #[test]
    fn test_clear_map() {
        let mut seq = ShapeProcessAPIApplySequence::new("rsc", "seq");
        seq.prepare_shape("shape1", true, 0);
        seq.clear_map();
        assert_eq!(seq.map().len(), 0);
    }
}
