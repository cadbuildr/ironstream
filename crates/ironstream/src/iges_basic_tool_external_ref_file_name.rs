// FILE: iges_basic_tool_external_ref_file_name.rs
// occt: IGESBasic_ToolExternalRefFileName

/// Tool to work on an ExternalRefFileName.
pub struct IgesBasicToolExternalRefFileName;

impl IgesBasicToolExternalRefFileName {
    pub fn new() -> Self {
        Self
    }

    pub fn read_own_params(&self, _ent: &str, _ir: &str, _pr: &str) {}

    pub fn write_own_params(&self, _ent: &str, _iw: &str) {}

    pub fn own_shared(&self, _ent: &str) -> Vec<String> {
        vec![]
    }

    pub fn own_correct(&self, _ent: &str) -> bool {
        true
    }

    pub fn dir_checker(&self, _ent: &str) -> String {
        "ExternalRefFileName_DirChecker".to_string()
    }

    pub fn own_check(&self, _ent: &str, _shares: &str) {}

    pub fn own_copy(&self, _ent_from: &str, _ent_to: &str, _tc: &str) {}

    pub fn own_dump(&self, ent: &str, _dumper: &str, _own: i32) -> String {
        format!("ExternalRefFileName dump: {}", ent)
    }
}

impl Default for IgesBasicToolExternalRefFileName {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = IgesBasicToolExternalRefFileName::new();
        assert_eq!(tool.dir_checker("test"), "ExternalRefFileName_DirChecker");
    }
}
