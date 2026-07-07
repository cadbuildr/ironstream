// FILE: transfer_transfer_output.rs
// occt: Transfer_TransferOutput

/// Output specification for a transfer operation.
/// Defines where and how transfer results should be stored.
#[derive(Clone, Debug)]
pub struct TransferTransferOutput {
    /// Output destination ID
    destination_id: u32,
    /// Output format (0=default, 1=compact, 2=verbose)
    format: u32,
    /// Whether to include metadata
    include_metadata: bool,
}

impl TransferTransferOutput {
    /// Creates a new transfer output specification.
    pub fn new(destination_id: u32) -> Self {
        Self {
            destination_id,
            format: 0,
            include_metadata: false,
        }
    }

    /// Returns the destination ID.
    pub fn destination_id(&self) -> u32 {
        self.destination_id
    }

    /// Returns the output format.
    pub fn format(&self) -> u32 {
        self.format
    }

    /// Sets the output format.
    pub fn set_format(&mut self, format: u32) {
        self.format = format;
    }

    /// Returns whether metadata is included.
    pub fn include_metadata(&self) -> bool {
        self.include_metadata
    }

    /// Sets whether to include metadata.
    pub fn set_include_metadata(&mut self, include: bool) {
        self.include_metadata = include;
    }
}

impl Default for TransferTransferOutput {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let output = TransferTransferOutput::new(456);
        assert_eq!(output.destination_id(), 456);
        assert_eq!(output.format(), 0);
        assert!(!output.include_metadata());
    }

    #[test]
    fn test_set_format() {
        let mut output = TransferTransferOutput::new(1);
        output.set_format(1);
        assert_eq!(output.format(), 1);

        output.set_format(2);
        assert_eq!(output.format(), 2);
    }

    #[test]
    fn test_set_include_metadata() {
        let mut output = TransferTransferOutput::new(1);
        assert!(!output.include_metadata());

        output.set_include_metadata(true);
        assert!(output.include_metadata());
    }

    #[test]
    fn test_default() {
        let output = TransferTransferOutput::default();
        assert_eq!(output.destination_id(), 0);
    }
}
