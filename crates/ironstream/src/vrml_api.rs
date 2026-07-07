// FILE: vrml_api.rs
// occt: VrmlAPI

#[derive(Clone, Debug)]
pub struct VrmlApi {}

impl VrmlApi {
    pub fn new() -> Self {
        VrmlApi {}
    }

    pub fn version() -> &'static str {
        "2.0"
    }
}

impl Default for VrmlApi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let api = VrmlApi::new();
        assert_eq!(VrmlApi::version(), "2.0");
    }

    #[test]
    fn test_default() {
        let api = VrmlApi::default();
        assert_eq!(VrmlApi::version(), "2.0");
    }
}
