// FILE: vrml.rs
// occt: Vrml

#[derive(Clone, Debug)]
pub struct Vrml {
    version: String,
}

impl Vrml {
    pub fn new() -> Self {
        Vrml {
            version: "2.0".to_string(),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn set_version(&mut self, ver: &str) {
        self.version = ver.to_string();
    }
}

impl Default for Vrml {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let v = Vrml::new();
        assert_eq!(v.version(), "2.0");
    }

    #[test]
    fn test_set_version() {
        let mut v = Vrml::new();
        v.set_version("1.0");
        assert_eq!(v.version(), "1.0");
    }
}
