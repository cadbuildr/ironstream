// FILE: header_section_file_description.rs
// occt: HeaderSection_FileDescription

#[derive(Clone, Debug)]
pub struct HeaderSection_FileDescription {
    pub description: Vec<String>,
    pub implementation_level: Option<String>,
}

impl HeaderSection_FileDescription {
    pub fn new() -> Self {
        HeaderSection_FileDescription {
            description: vec![],
            implementation_level: None,
        }
    }

    pub fn set_description(&mut self, desc: Vec<String>) {
        self.description = desc;
    }

    pub fn set_implementation_level(&mut self, level: String) {
        self.implementation_level = Some(level);
    }
}

impl Default for HeaderSection_FileDescription {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let fd = HeaderSection_FileDescription::new();
        assert!(fd.description.is_empty());
    }

    #[test]
    fn test_set_description() {
        let mut fd = HeaderSection_FileDescription::new();
        fd.set_description(vec!["Test Description".to_string()]);
        assert_eq!(fd.description.len(), 1);
    }

    #[test]
    fn test_set_implementation_level() {
        let mut fd = HeaderSection_FileDescription::new();
        fd.set_implementation_level("2;1".to_string());
        assert_eq!(fd.implementation_level, Some("2;1".to_string()));
    }
}
