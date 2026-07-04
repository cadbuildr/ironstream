// FILE: cdm_meta_data.rs
// occt: CDM_MetaData

/// returns the folder in which the meta-data has to be created
  //! or has to be found.
  Standard_EXP
#[derive(Debug, Clone)]
pub struct CDM_MetaData {
    // TODO: Port fields from OCCT
}

impl CDM_MetaData {
    /// Creates a new instance
    pub fn new() -> Self {
        CDM_MetaData {
        }
    }
}

impl Default for CDM_MetaData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdm_meta_data_creation() {
        let obj = CDM_MetaData::new();
        let _default = CDM_MetaData::default();
        // TODO: Add more tests from OCCT gtest
    }
}
