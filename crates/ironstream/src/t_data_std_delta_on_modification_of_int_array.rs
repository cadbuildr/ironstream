// FILE: t_data_std_delta_on_modification_of_int_array.rs
// occt: TDataStd_DeltaOnModificationOfIntArray

/// Delta for integer array modification.
pub struct TDataStdDeltaOnModificationOfIntArray;

impl TDataStdDeltaOnModificationOfIntArray {
    /// Creates a new integer array modification delta.
    pub fn new() -> Self {
        TDataStdDeltaOnModificationOfIntArray
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_int_array() {
        let _delta = TDataStdDeltaOnModificationOfIntArray::new();
    }
}
