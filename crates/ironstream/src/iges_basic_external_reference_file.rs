// FILE: iges_basic_external_reference_file.rs
// occt: IGESBasic_ExternalReferenceFile

/// ExternalReferenceFile, Type <406> Form <12>
/// References definitions residing in another file.
pub struct IgesBasicExternalReferenceFile {
    names: Vec<String>,
}

impl IgesBasicExternalReferenceFile {
    /// Create a new ExternalReferenceFile with default values.
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
        }
    }

    /// Set the field of the class ExternalReferenceFile.
    /// - name_array: External Reference File Names
    pub fn init(&mut self, name_array: Vec<String>) {
        self.names = name_array;
    }

    /// Returns number of External Reference File Names.
    pub fn nb_list_entries(&self) -> i32 {
        self.names.len() as i32
    }

    /// Returns External Reference File Name.
    /// Raises exception if Index <= 0 or Index > NbListEntries().
    pub fn name(&self, index: i32) -> Option<&str> {
        if index <= 0 || index > self.nb_list_entries() {
            return None;
        }
        Some(&self.names[(index - 1) as usize])
    }
}

impl Default for IgesBasicExternalReferenceFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let erf = IgesBasicExternalReferenceFile::new();
        assert_eq!(erf.nb_list_entries(), 0);
    }

    #[test]
    fn test_init() {
        let mut erf = IgesBasicExternalReferenceFile::new();
        let names = vec!["file1.igs".to_string(), "file2.igs".to_string()];
        erf.init(names);
        assert_eq!(erf.nb_list_entries(), 2);
        assert_eq!(erf.name(1), Some("file1.igs"));
        assert_eq!(erf.name(2), Some("file2.igs"));
    }

    #[test]
    fn test_boundary_checks() {
        let mut erf = IgesBasicExternalReferenceFile::new();
        let names = vec!["file1.igs".to_string()];
        erf.init(names);
        assert_eq!(erf.name(0), None);
        assert_eq!(erf.name(2), None);
    }

    #[test]
    fn test_default() {
        let erf = IgesBasicExternalReferenceFile::default();
        assert_eq!(erf.nb_list_entries(), 0);
    }
}
