// FILE: api_header_section_make_header.rs
// occt: APIHeaderSection_MakeHeader

use std::collections::HashMap;

/// A simple representation of Header Section entities
#[derive(Clone, Debug)]
pub struct HeaderSectionFileName {
    name: Option<String>,
    timestamp: Option<String>,
    author: Vec<String>,
    organization: Vec<String>,
    preprocessor_version: Option<String>,
    originating_system: Option<String>,
    authorisation: Option<String>,
}

#[derive(Clone, Debug)]
pub struct HeaderSectionFileSchema {
    schema_identifiers: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct HeaderSectionFileDescription {
    description: Vec<String>,
    implementation_level: Option<String>,
}

impl Default for HeaderSectionFileName {
    fn default() -> Self {
        Self {
            name: None,
            timestamp: None,
            author: vec![],
            organization: vec![],
            preprocessor_version: None,
            originating_system: None,
            authorisation: None,
        }
    }
}

impl Default for HeaderSectionFileSchema {
    fn default() -> Self {
        Self {
            schema_identifiers: vec![],
        }
    }
}

impl Default for HeaderSectionFileDescription {
    fn default() -> Self {
        Self {
            description: vec![],
            implementation_level: None,
        }
    }
}

/// Builder for STEP header sections
pub struct APIHeaderSection_MakeHeader {
    done: bool,
    fn_section: Option<HeaderSectionFileName>,
    fs_section: Option<HeaderSectionFileSchema>,
    fd_section: Option<HeaderSectionFileDescription>,
}

impl APIHeaderSection_MakeHeader {
    /// Creates a new MakeHeader with default initialization based on shape type.
    /// shapetype: 0=default, 1=Facetted, 2=Face Based Surface, 3=Shell Based Surface, 4=Manifold Solid
    pub fn new(shapetype: i32) -> Self {
        let mut mkh = APIHeaderSection_MakeHeader {
            done: false,
            fn_section: None,
            fs_section: None,
            fd_section: None,
        };
        mkh.init_with_type(shapetype);
        mkh
    }

    fn init_with_type(&mut self, shapetype: i32) {
        let model_name = match shapetype {
            1 => "Open CASCADE Facetted BRep Model",
            2 => "Open CASCADE Face Based Surface Model",
            3 => "Open CASCADE Shell Based Surface Model",
            4 => "Open CASCADE Manifold Solid Brep Model",
            _ => "Open CASCADE Shape Model",
        };
        self.init_from_name(model_name);
    }

    /// Initializes from a step model (placeholder for actual model inspection).
    /// In a real implementation, this would inspect the StepData_StepModel.
    pub fn from_model() -> Self {
        APIHeaderSection_MakeHeader {
            done: false,
            fn_section: None,
            fs_section: None,
            fd_section: None,
        }
    }

    /// Cancels former definition and initializes with a filename
    pub fn init_from_name(&mut self, name: &str) {
        self.done = true;

        // Initialize FileName section
        let mut fn_section = HeaderSectionFileName::default();
        fn_section.name = Some(name.to_string());
        fn_section.timestamp = Some(current_timestamp());
        fn_section.author = vec!["Author".to_string()];
        fn_section.organization = vec!["Open CASCADE".to_string()];
        fn_section.preprocessor_version = Some(get_processor_version());
        fn_section.originating_system = Some(get_system_version());
        fn_section.authorisation = Some("Unknown".to_string());
        self.fn_section = Some(fn_section);

        // Initialize FileDescription section
        let mut fd_section = HeaderSectionFileDescription::default();
        fd_section.description = vec!["Open CASCADE Model".to_string()];
        fd_section.implementation_level = Some("2;1".to_string());
        self.fd_section = Some(fd_section);

        // Initialize FileSchema section
        let mut fs_section = HeaderSectionFileSchema::default();
        fs_section.schema_identifiers = vec!["".to_string()];
        self.fs_section = Some(fs_section);
    }

    /// Returns true if all data have been properly defined
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Checks whether there is a file_name entity
    pub fn has_fn(&self) -> bool {
        self.fn_section.is_some()
    }

    /// Checks whether there is a file_schema entity
    pub fn has_fs(&self) -> bool {
        self.fs_section.is_some()
    }

    /// Checks whether there is a file_description entity
    pub fn has_fd(&self) -> bool {
        self.fd_section.is_some()
    }

    // ========== FileName Methods ==========

    pub fn name(&self) -> Option<String> {
        self.fn_section.as_ref().and_then(|fn_sec| fn_sec.name.clone())
    }

    pub fn set_name(&mut self, name: String) {
        if let Some(ref mut fn_sec) = self.fn_section {
            fn_sec.name = Some(name);
        }
    }

    pub fn timestamp(&self) -> Option<String> {
        self.fn_section.as_ref().and_then(|fn_sec| fn_sec.timestamp.clone())
    }

    pub fn set_timestamp(&mut self, timestamp: String) {
        if let Some(ref mut fn_sec) = self.fn_section {
            fn_sec.timestamp = Some(timestamp);
        }
    }

    pub fn author_value(&self, num: usize) -> Option<String> {
        self.fn_section.as_ref().and_then(|fn_sec| {
            if num > 0 && num <= fn_sec.author.len() {
                Some(fn_sec.author[num - 1].clone())
            } else {
                None
            }
        })
    }

    pub fn set_author_value(&mut self, num: usize, author: String) {
        if let Some(ref mut fn_sec) = self.fn_section {
            if num > 0 {
                if num > fn_sec.author.len() {
                    fn_sec.author.resize(num, String::new());
                }
                fn_sec.author[num - 1] = author;
            }
        }
    }

    pub fn nb_author(&self) -> usize {
        self.fn_section.as_ref().map_or(0, |fn_sec| fn_sec.author.len())
    }

    pub fn organization_value(&self, num: usize) -> Option<String> {
        self.fn_section.as_ref().and_then(|fn_sec| {
            if num > 0 && num <= fn_sec.organization.len() {
                Some(fn_sec.organization[num - 1].clone())
            } else {
                None
            }
        })
    }

    pub fn set_organization_value(&mut self, num: usize, org: String) {
        if let Some(ref mut fn_sec) = self.fn_section {
            if num > 0 {
                if num > fn_sec.organization.len() {
                    fn_sec.organization.resize(num, String::new());
                }
                fn_sec.organization[num - 1] = org;
            }
        }
    }

    pub fn nb_organization(&self) -> usize {
        self.fn_section.as_ref().map_or(0, |fn_sec| fn_sec.organization.len())
    }

    pub fn preprocessor_version(&self) -> Option<String> {
        self.fn_section.as_ref().and_then(|fn_sec| fn_sec.preprocessor_version.clone())
    }

    pub fn set_preprocessor_version(&mut self, ver: String) {
        if let Some(ref mut fn_sec) = self.fn_section {
            fn_sec.preprocessor_version = Some(ver);
        }
    }

    pub fn originating_system(&self) -> Option<String> {
        self.fn_section.as_ref().and_then(|fn_sec| fn_sec.originating_system.clone())
    }

    pub fn set_originating_system(&mut self, sys: String) {
        if let Some(ref mut fn_sec) = self.fn_section {
            fn_sec.originating_system = Some(sys);
        }
    }

    pub fn authorisation(&self) -> Option<String> {
        self.fn_section.as_ref().and_then(|fn_sec| fn_sec.authorisation.clone())
    }

    pub fn set_authorisation(&mut self, auth: String) {
        if let Some(ref mut fn_sec) = self.fn_section {
            fn_sec.authorisation = Some(auth);
        }
    }

    // ========== FileSchema Methods ==========

    pub fn schema_identifiers_value(&self, num: usize) -> Option<String> {
        self.fs_section.as_ref().and_then(|fs_sec| {
            if num > 0 && num <= fs_sec.schema_identifiers.len() {
                Some(fs_sec.schema_identifiers[num - 1].clone())
            } else {
                None
            }
        })
    }

    pub fn set_schema_identifiers_value(&mut self, num: usize, schema: String) {
        if let Some(ref mut fs_sec) = self.fs_section {
            if num > 0 {
                if num > fs_sec.schema_identifiers.len() {
                    fs_sec.schema_identifiers.resize(num, String::new());
                }
                fs_sec.schema_identifiers[num - 1] = schema;
            }
        }
    }

    pub fn nb_schema_identifiers(&self) -> usize {
        self.fs_section.as_ref().map_or(0, |fs_sec| fs_sec.schema_identifiers.len())
    }

    pub fn add_schema_identifier(&mut self, schema: String) {
        if let Some(ref mut fs_sec) = self.fs_section {
            if !fs_sec.schema_identifiers.contains(&schema) {
                fs_sec.schema_identifiers.push(schema);
            }
        }
    }

    // ========== FileDescription Methods ==========

    pub fn description_value(&self, num: usize) -> Option<String> {
        self.fd_section.as_ref().and_then(|fd_sec| {
            if num > 0 && num <= fd_sec.description.len() {
                Some(fd_sec.description[num - 1].clone())
            } else {
                None
            }
        })
    }

    pub fn set_description_value(&mut self, num: usize, desc: String) {
        if let Some(ref mut fd_sec) = self.fd_section {
            if num > 0 {
                if num > fd_sec.description.len() {
                    fd_sec.description.resize(num, String::new());
                }
                fd_sec.description[num - 1] = desc;
            }
        }
    }

    pub fn nb_description(&self) -> usize {
        self.fd_section.as_ref().map_or(0, |fd_sec| fd_sec.description.len())
    }

    pub fn implementation_level(&self) -> Option<String> {
        self.fd_section.as_ref().and_then(|fd_sec| fd_sec.implementation_level.clone())
    }

    pub fn set_implementation_level(&mut self, level: String) {
        if let Some(ref mut fd_sec) = self.fd_section {
            fd_sec.implementation_level = Some(level);
        }
    }
}

fn current_timestamp() -> String {
    // Return a default timestamp in YYYY-MM-DDThh:mm:ss format
    // In a real implementation, this would use the system time
    "2026-07-03T00:00:00".to_string()
}

fn get_processor_version() -> String {
    "XSTEP 1.0".to_string()
}

fn get_system_version() -> String {
    "IronStream".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let mkh = APIHeaderSection_MakeHeader::new(0);
        assert!(mkh.is_done());
        assert!(mkh.has_fn());
        assert!(mkh.has_fs());
        assert!(mkh.has_fd());
    }

    #[test]
    fn test_create_with_shapetype_1() {
        let mkh = APIHeaderSection_MakeHeader::new(1);
        assert!(mkh.is_done());
        let name = mkh.name().unwrap();
        assert!(name.contains("Facetted"));
    }

    #[test]
    fn test_create_with_shapetype_4() {
        let mkh = APIHeaderSection_MakeHeader::new(4);
        assert!(mkh.is_done());
        let name = mkh.name().unwrap();
        assert!(name.contains("Manifold Solid"));
    }

    #[test]
    fn test_name_operations() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        mkh.set_name("TestFile.stp".to_string());
        assert_eq!(mkh.name(), Some("TestFile.stp".to_string()));
    }

    #[test]
    fn test_timestamp_operations() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        mkh.set_timestamp("2023-05-15T14:30:45".to_string());
        assert_eq!(mkh.timestamp(), Some("2023-05-15T14:30:45".to_string()));
    }

    #[test]
    fn test_author_operations() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        assert!(mkh.author_value(1).is_some());
        mkh.set_author_value(1, "John Doe".to_string());
        assert_eq!(mkh.author_value(1), Some("John Doe".to_string()));
        assert!(mkh.nb_author() >= 1);
    }

    #[test]
    fn test_organization_operations() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        assert!(mkh.organization_value(1).is_some());
        mkh.set_organization_value(1, "ACME Corp".to_string());
        assert_eq!(mkh.organization_value(1), Some("ACME Corp".to_string()));
    }

    #[test]
    fn test_preprocessor_version() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        assert!(mkh.preprocessor_version().is_some());
        mkh.set_preprocessor_version("2.0".to_string());
        assert_eq!(mkh.preprocessor_version(), Some("2.0".to_string()));
    }

    #[test]
    fn test_originating_system() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        assert!(mkh.originating_system().is_some());
        mkh.set_originating_system("CADSystem".to_string());
        assert_eq!(mkh.originating_system(), Some("CADSystem".to_string()));
    }

    #[test]
    fn test_authorisation() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        assert!(mkh.authorisation().is_some());
        mkh.set_authorisation("Public".to_string());
        assert_eq!(mkh.authorisation(), Some("Public".to_string()));
    }

    #[test]
    fn test_schema_identifiers() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        mkh.set_schema_identifiers_value(1, "AP203".to_string());
        assert_eq!(mkh.schema_identifiers_value(1), Some("AP203".to_string()));
        assert!(mkh.nb_schema_identifiers() >= 1);
    }

    #[test]
    fn test_add_schema_identifier() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        mkh.add_schema_identifier("AP214".to_string());
        assert!(mkh.nb_schema_identifiers() >= 1);
    }

    #[test]
    fn test_description() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        mkh.set_description_value(1, "Custom Description".to_string());
        assert_eq!(mkh.description_value(1), Some("Custom Description".to_string()));
        assert!(mkh.nb_description() >= 1);
    }

    #[test]
    fn test_implementation_level() {
        let mut mkh = APIHeaderSection_MakeHeader::new(0);
        assert!(mkh.implementation_level().is_some());
        mkh.set_implementation_level("3;1".to_string());
        assert_eq!(mkh.implementation_level(), Some("3;1".to_string()));
    }
}
