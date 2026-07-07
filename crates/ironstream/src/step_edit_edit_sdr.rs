// FILE: step_edit_edit_sdr.rs
// occt: STEPEdit_EditSDR

/// Shape Definition Representation editor
pub struct STEPEdit_EditSDR;

impl STEPEdit_EditSDR {
    pub fn new() -> Self {
        STEPEdit_EditSDR
    }
}

impl Default for STEPEdit_EditSDR {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _sdr = STEPEdit_EditSDR::new();
    }
}
