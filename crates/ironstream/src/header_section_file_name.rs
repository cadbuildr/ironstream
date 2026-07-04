// FILE: header_section_file_name.rs
// occt: HeaderSection_FileName

#[derive(Clone, Debug)]
pub struct HeaderSection_FileName {
    pub name: Option<String>,
    pub timestamp: Option<String>,
    pub author: Vec<String>,
    pub organization: Vec<String>,
    pub preprocessor_version: Option<String>,
    pub originating_system: Option<String>,
    pub authorization: Option<String>,
}

impl HeaderSection_FileName {
    pub fn new() -> Self {
        HeaderSection_FileName {
            name: None,
            timestamp: None,
            author: vec![],
            organization: vec![],
            preprocessor_version: None,
            originating_system: None,
            authorization: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_timestamp(&mut self, timestamp: String) {
        self.timestamp = Some(timestamp);
    }

    pub fn set_author(&mut self, author: Vec<String>) {
        self.author = author;
    }

    pub fn set_organization(&mut self, org: Vec<String>) {
        self.organization = org;
    }

    pub fn set_preprocessor_version(&mut self, ver: String) {
        self.preprocessor_version = Some(ver);
    }

    pub fn set_originating_system(&mut self, sys: String) {
        self.originating_system = Some(sys);
    }

    pub fn set_authorization(&mut self, auth: String) {
        self.authorization = Some(auth);
    }
}

impl Default for HeaderSection_FileName {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let fn_sec = HeaderSection_FileName::new();
        assert!(fn_sec.name.is_none());
    }

    #[test]
    fn test_set_name() {
        let mut fn_sec = HeaderSection_FileName::new();
        fn_sec.set_name("test.stp".to_string());
        assert_eq!(fn_sec.name, Some("test.stp".to_string()));
    }

    #[test]
    fn test_set_author() {
        let mut fn_sec = HeaderSection_FileName::new();
        fn_sec.set_author(vec!["John Doe".to_string()]);
        assert_eq!(fn_sec.author.len(), 1);
    }
}
