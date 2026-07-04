// FILE: iges_data_file_protocol.rs
// occt: IGESData_FileProtocol

//! Defines complex protocols to treat various sub-sets of the IGES Norm.
//! Can combine multiple IGES protocol resources while admitting UndefinedEntity.

/// FileProtocol manages a chain of IGES protocol resources
#[derive(Clone, Debug)]
pub struct FileProtocol {
    resources: Vec<String>,
}

impl FileProtocol {
    /// Returns an empty FileProtocol
    pub fn new() -> Self {
        FileProtocol {
            resources: Vec::new(),
        }
    }

    /// Adds a protocol resource by name
    pub fn add(&mut self, protocol_name: &str) {
        self.resources.push(protocol_name.to_string());
    }

    /// Gives the count of Resources: the count of Added Protocols
    pub fn nb_resources(&self) -> usize {
        self.resources.len()
    }

    /// Returns a Resource, given a rank (index into added protocols)
    pub fn resource(&self, num: usize) -> Option<String> {
        if num < self.resources.len() {
            Some(self.resources[num].clone())
        } else {
            None
        }
    }

    /// Returns all resources as a vector
    pub fn resources(&self) -> &[String] {
        &self.resources
    }

    /// Clears all added resources
    pub fn clear(&mut self) {
        self.resources.clear();
    }
}

impl Default for FileProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fp = FileProtocol::new();
        assert_eq!(fp.nb_resources(), 0);
    }

    #[test]
    fn test_add() {
        let mut fp = FileProtocol::new();
        assert_eq!(fp.nb_resources(), 0);

        fp.add("iges");
        assert_eq!(fp.nb_resources(), 1);

        fp.add("solid");
        assert_eq!(fp.nb_resources(), 2);
    }

    #[test]
    fn test_resource() {
        let mut fp = FileProtocol::new();
        fp.add("protocol1");
        fp.add("protocol2");

        assert_eq!(fp.resource(0), Some("protocol1".to_string()));
        assert_eq!(fp.resource(1), Some("protocol2".to_string()));
        assert_eq!(fp.resource(2), None);
        assert_eq!(fp.resource(10), None);
    }

    #[test]
    fn test_resources() {
        let mut fp = FileProtocol::new();
        fp.add("a");
        fp.add("b");
        fp.add("c");

        let res = fp.resources();
        assert_eq!(res.len(), 3);
        assert_eq!(res[0], "a");
        assert_eq!(res[1], "b");
        assert_eq!(res[2], "c");
    }

    #[test]
    fn test_clear() {
        let mut fp = FileProtocol::new();
        fp.add("test");
        assert_eq!(fp.nb_resources(), 1);

        fp.clear();
        assert_eq!(fp.nb_resources(), 0);
    }

    #[test]
    fn test_default() {
        let fp = FileProtocol::default();
        assert_eq!(fp.nb_resources(), 0);
    }
}
