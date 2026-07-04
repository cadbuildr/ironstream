// FILE: if_select_session_dumper.rs
// occt: IFSelect_SessionDumper

/// A session dumper manages writing/reading parameters for a set of classes.
/// Dumpers are organized in a library used by SessionFile.
#[derive(Clone, Debug)]
pub struct IFSelectSessionDumper {
    dumper_id: String,
}

impl IFSelectSessionDumper {
    /// Creates a SessionDumper with a given ID
    pub fn new(id: String) -> Self {
        Self { dumper_id: id }
    }

    /// Returns the ID of this dumper
    pub fn id(&self) -> &str {
        &self.dumper_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let dumper = IFSelectSessionDumper::new("test_dumper".to_string());
        assert_eq!(dumper.id(), "test_dumper");
    }

    #[test]
    fn test_multiple_dumpers() {
        let dumper1 = IFSelectSessionDumper::new("dumper1".to_string());
        let dumper2 = IFSelectSessionDumper::new("dumper2".to_string());
        assert_eq!(dumper1.id(), "dumper1");
        assert_eq!(dumper2.id(), "dumper2");
    }
}
