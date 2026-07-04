// FILE: xs_control_transfer_writer.rs
// occt: XSControl_TransferWriter

/// Writer for transferring and writing entities using the control framework.
/// Manages the transfer of entities to output format.
#[derive(Clone, Debug)]
pub struct XSControlTransferWriter {
    /// Writer identifier
    writer_id: u32,
    /// Output mode
    mode: u32,
}

impl XSControlTransferWriter {
    /// Creates a new transfer writer.
    pub fn new() -> Self {
        Self {
            writer_id: 0,
            mode: 0,
        }
    }

    /// Returns the writer ID.
    pub fn id(&self) -> u32 {
        self.writer_id
    }

    /// Returns the output mode.
    pub fn mode(&self) -> u32 {
        self.mode
    }

    /// Sets the output mode.
    pub fn set_mode(&mut self, mode: u32) {
        self.mode = mode;
    }
}

impl Default for XSControlTransferWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let writer = XSControlTransferWriter::new();
        assert_eq!(writer.id(), 0);
        assert_eq!(writer.mode(), 0);
    }

    #[test]
    fn test_set_mode() {
        let mut writer = XSControlTransferWriter::new();
        writer.set_mode(2);
        assert_eq!(writer.mode(), 2);
    }
}
