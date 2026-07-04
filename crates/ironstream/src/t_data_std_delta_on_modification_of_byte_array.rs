// FILE: t_data_std_delta_on_modification_of_byte_array.rs
// occt: TDataStd_DeltaOnModificationOfByteArray

/// Delta for byte array modification.
pub struct TDataStdDeltaOnModificationOfByteArray;

impl TDataStdDeltaOnModificationOfByteArray {
    /// Creates a new byte array modification delta.
    pub fn new() -> Self {
        TDataStdDeltaOnModificationOfByteArray
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_byte_array() {
        let _delta = TDataStdDeltaOnModificationOfByteArray::new();
    }
}
