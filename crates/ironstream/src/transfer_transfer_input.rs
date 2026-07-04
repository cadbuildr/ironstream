// FILE: transfer_transfer_input.rs
// occt: Transfer_TransferInput

/// Input specification for a transfer operation.
/// Defines which entities and parameters should be used for a transfer.
#[derive(Clone, Debug)]
pub struct TransferTransferInput {
    /// Input entity ID
    entity_id: u32,
    /// Input mode (0=normal, 1=recursive, 2=bypass)
    mode: u32,
    /// Additional flags
    flags: u32,
}

impl TransferTransferInput {
    /// Creates a new transfer input.
    pub fn new(entity_id: u32) -> Self {
        Self {
            entity_id,
            mode: 0,
            flags: 0,
        }
    }

    /// Returns the input entity ID.
    pub fn entity_id(&self) -> u32 {
        self.entity_id
    }

    /// Returns the input mode.
    pub fn mode(&self) -> u32 {
        self.mode
    }

    /// Sets the input mode.
    pub fn set_mode(&mut self, mode: u32) {
        self.mode = mode;
    }

    /// Returns the flags.
    pub fn flags(&self) -> u32 {
        self.flags
    }

    /// Sets the flags.
    pub fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }
}

impl Default for TransferTransferInput {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = TransferTransferInput::new(123);
        assert_eq!(input.entity_id(), 123);
        assert_eq!(input.mode(), 0);
        assert_eq!(input.flags(), 0);
    }

    #[test]
    fn test_set_mode() {
        let mut input = TransferTransferInput::new(1);
        input.set_mode(1);
        assert_eq!(input.mode(), 1);

        input.set_mode(2);
        assert_eq!(input.mode(), 2);
    }

    #[test]
    fn test_set_flags() {
        let mut input = TransferTransferInput::new(1);
        input.set_flags(0xFF);
        assert_eq!(input.flags(), 0xFF);
    }

    #[test]
    fn test_default() {
        let input = TransferTransferInput::default();
        assert_eq!(input.entity_id(), 0);
    }
}
