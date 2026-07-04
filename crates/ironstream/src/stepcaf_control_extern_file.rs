// FILE: stepcaf_control_extern_file.rs
// occt: STEPCAFControl_ExternFile

/// Auxiliary class serving as container for data resulting from translation of external file
pub struct STEPCAFControl_ExternFile {
    ws: Option<()>, // Placeholder for XSControl_WorkSession handle
    load_status: i32,
    transfer_status: bool,
    write_status: i32,
    name: Option<String>,
    label: (),
}

impl STEPCAFControl_ExternFile {
    /// Creates an empty structure
    pub fn new() -> Self {
        STEPCAFControl_ExternFile {
            ws: None,
            load_status: 0, // IFSelect_RetVoid
            transfer_status: false,
            write_status: 0, // IFSelect_RetVoid
            name: None,
            label: (),
        }
    }

    pub fn set_ws(&mut self, _ws: ()) {
        self.ws = Some(());
    }

    pub fn get_ws(&self) -> Option<()> {
        self.ws
    }

    pub fn set_load_status(&mut self, stat: i32) {
        self.load_status = stat;
    }

    pub fn get_load_status(&self) -> i32 {
        self.load_status
    }

    pub fn set_transfer_status(&mut self, isok: bool) {
        self.transfer_status = isok;
    }

    pub fn get_transfer_status(&self) -> bool {
        self.transfer_status
    }

    pub fn set_write_status(&mut self, stat: i32) {
        self.write_status = stat;
    }

    pub fn get_write_status(&self) -> i32 {
        self.write_status
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    pub fn set_label(&mut self, _label: ()) {
        self.label = ();
    }

    pub fn get_label(&self) -> () {
        self.label
    }
}

impl Default for STEPCAFControl_ExternFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty() {
        let file = STEPCAFControl_ExternFile::new();
        assert_eq!(file.get_load_status(), 0);
        assert_eq!(file.get_transfer_status(), false);
        assert_eq!(file.get_write_status(), 0);
        assert_eq!(file.get_name(), &None);
    }

    #[test]
    fn test_set_get_transfer_status() {
        let mut file = STEPCAFControl_ExternFile::new();
        file.set_transfer_status(true);
        assert_eq!(file.get_transfer_status(), true);
    }

    #[test]
    fn test_set_get_load_status() {
        let mut file = STEPCAFControl_ExternFile::new();
        file.set_load_status(42);
        assert_eq!(file.get_load_status(), 42);
    }

    #[test]
    fn test_set_get_name() {
        let mut file = STEPCAFControl_ExternFile::new();
        let name = Some("test.stp".to_string());
        file.set_name(name.clone());
        assert_eq!(file.get_name(), &name);
    }
}
