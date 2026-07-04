// FILE: t_data_std_delta_on_modification_of_ext_string_array.rs
// occt: TDataStd_DeltaOnModificationOfExtStringArray

/// Delta for extended string array modification.
pub struct TDataStdDeltaOnModificationOfExtStringArray;

impl TDataStdDeltaOnModificationOfExtStringArray {
    /// Creates a new extended string array modification delta.
    pub fn new() -> Self {
        TDataStdDeltaOnModificationOfExtStringArray
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_ext_string_array() {
        let _delta = TDataStdDeltaOnModificationOfExtStringArray::new();
    }
}
