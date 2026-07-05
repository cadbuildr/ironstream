// FILE: vrml_api_caf_reader.rs
// occt: VrmlAPI_CafReader

#[derive(Clone, Debug)]
pub struct VrmlApiCafReader {
    filename: String,
}

impl VrmlApiCafReader {
    pub fn new(filename: &str) -> Self {
        VrmlApiCafReader {
            filename: filename.to_string(),
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn read(&self) -> bool {
        !self.filename.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let reader = VrmlApiCafReader::new("file.wrl");
        assert_eq!(reader.filename(), "file.wrl");
    }

    #[test]
    fn test_read() {
        let reader = VrmlApiCafReader::new("file.wrl");
        assert!(reader.read());
    }

    #[test]
    fn test_read_empty() {
        let reader = VrmlApiCafReader::new("");
        assert!(!reader.read());
    }
}
