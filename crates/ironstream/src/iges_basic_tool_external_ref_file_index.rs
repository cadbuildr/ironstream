// FILE: iges_basic_tool_external_ref_file_index.rs
// occt: IGESBasic_ToolExternalRefFileIndex

/// Tool to work on an ExternalRefFileIndex.
pub struct IgesBasicToolExternalRefFileIndex;

impl IgesBasicToolExternalRefFileIndex {
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
        "ExternalRefFileIndex_DirChecker".to_string()
    }

    pub fn own_check(&self, _ent: &str, _shares: &str) {}

    pub fn own_copy(&self, _ent_from: &str, _ent_to: &str, _tc: &str) {}

    pub fn own_dump(&self, ent: &str, _dumper: &str, _own: i32) -> String {
        format!("ExternalRefFileIndex dump: {}", ent)
    }
}

impl Default for IgesBasicToolExternalRefFileIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = IgesBasicToolExternalRefFileIndex::new();
        assert_eq!(tool.dir_checker("test"), "ExternalRefFileIndex_DirChecker");
    }

    #[test]
    fn test_own_correct() {
        let tool = IgesBasicToolExternalRefFileIndex::new();
        assert!(tool.own_correct("test"));
    }
}
