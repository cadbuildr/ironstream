// FILE: interface_sign_label.rs
// occt: Interface_SignLabel

/// Signs labels
pub struct InterfaceSignLabel {
    label: String,
}

impl InterfaceSignLabel {
    pub fn new(label: &str) -> Self {
        InterfaceSignLabel {
            label: label.to_string(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }
}

impl Default for InterfaceSignLabel {
    fn default() -> Self {
        InterfaceSignLabel {
            label: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sign = InterfaceSignLabel::new("test");
        assert_eq!(sign.label(), "test");
    }

    #[test]
    fn test_set_label() {
        let mut sign = InterfaceSignLabel::new("old");
        sign.set_label("new");
        assert_eq!(sign.label(), "new");
    }
}
