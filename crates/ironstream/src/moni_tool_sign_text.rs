// FILE: moni_tool_sign_text.rs
// occt: MoniTool_SignText

/// Signs text
pub struct MoniToolSignText {
    text: String,
}

impl MoniToolSignText {
    pub fn new(text: &str) -> Self {
        MoniToolSignText {
            text: text.to_string(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl Default for MoniToolSignText {
    fn default() -> Self {
        MoniToolSignText {
            text: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sign = MoniToolSignText::new("test");
        assert_eq!(sign.text(), "test");
    }
}
