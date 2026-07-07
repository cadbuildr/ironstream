// FILE: transfer_result_from_model.rs
// occt: Transfer_ResultFromModel

/// Represents a transfer result derived from a model entity.
/// Tracks the origin model and the resulting entity.
#[derive(Clone, Debug)]
pub struct TransferResultFromModel {
    /// Model entity ID
    model_entity_id: u32,
    /// Result entity ID
    result_entity_id: u32,
    /// Whether the transfer succeeded
    success: bool,
}

impl TransferResultFromModel {
    /// Creates a new transfer result from model.
    pub fn new(model_entity_id: u32, result_entity_id: u32) -> Self {
        Self {
            model_entity_id,
            result_entity_id,
            success: true,
        }
    }

    /// Returns the model entity ID.
    pub fn model_entity_id(&self) -> u32 {
        self.model_entity_id
    }

    /// Returns the result entity ID.
    pub fn result_entity_id(&self) -> u32 {
        self.result_entity_id
    }

    /// Returns whether the transfer succeeded.
    pub fn success(&self) -> bool {
        self.success
    }

    /// Sets the success status.
    pub fn set_success(&mut self, success: bool) {
        self.success = success;
    }
}

impl Default for TransferResultFromModel {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = TransferResultFromModel::new(100, 200);
        assert_eq!(result.model_entity_id(), 100);
        assert_eq!(result.result_entity_id(), 200);
        assert!(result.success());
    }

    #[test]
    fn test_set_success() {
        let mut result = TransferResultFromModel::new(1, 2);
        assert!(result.success());

        result.set_success(false);
        assert!(!result.success());

        result.set_success(true);
        assert!(result.success());
    }

    #[test]
    fn test_default() {
        let result = TransferResultFromModel::default();
        assert_eq!(result.model_entity_id(), 0);
        assert_eq!(result.result_entity_id(), 0);
    }
}
