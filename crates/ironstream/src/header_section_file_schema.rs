// FILE: header_section_file_schema.rs
// occt: HeaderSection_FileSchema

#[derive(Clone, Debug)]
pub struct HeaderSection_FileSchema {
    pub schema_identifiers: Vec<String>,
}

impl HeaderSection_FileSchema {
    pub fn new() -> Self {
        HeaderSection_FileSchema {
            schema_identifiers: vec![],
        }
    }

    pub fn set_schema_identifiers(&mut self, schemas: Vec<String>) {
        self.schema_identifiers = schemas;
    }

    pub fn get_schema_identifiers(&self) -> &[String] {
        &self.schema_identifiers
    }
}

impl Default for HeaderSection_FileSchema {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let fs = HeaderSection_FileSchema::new();
        assert!(fs.schema_identifiers.is_empty());
    }

    #[test]
    fn test_set_schema_identifiers() {
        let mut fs = HeaderSection_FileSchema::new();
        fs.set_schema_identifiers(vec!["AP203".to_string(), "AP214".to_string()]);
        assert_eq!(fs.schema_identifiers.len(), 2);
    }
}
