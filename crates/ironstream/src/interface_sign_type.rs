// FILE: interface_sign_type.rs
// occt: Interface_SignType

/// Signs types
pub struct InterfaceSignType {
    typename: String,
}

impl InterfaceSignType {
    pub fn new(typename: &str) -> Self {
        InterfaceSignType {
            typename: typename.to_string(),
        }
    }

    pub fn typename(&self) -> &str {
        &self.typename
    }

    pub fn set_typename(&mut self, typename: &str) {
        self.typename = typename.to_string();
    }
}

impl Default for InterfaceSignType {
    fn default() -> Self {
        InterfaceSignType {
            typename: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sign = InterfaceSignType::new("Entity");
        assert_eq!(sign.typename(), "Entity");
    }

    #[test]
    fn test_set_typename() {
        let mut sign = InterfaceSignType::new("Type1");
        sign.set_typename("Type2");
        assert_eq!(sign.typename(), "Type2");
    }
}
