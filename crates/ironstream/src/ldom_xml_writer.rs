// FILE: ldom_xml_writer.rs
// occt: LDOM_XmlWriter

/// XML writer for serializing LDOM documents
pub struct LDOMXmlWriter {
    encoding: Option<String>,
    indent: usize,
    cur_indent: usize,
}

impl LDOMXmlWriter {
    /// Constructor with optional encoding
    pub fn new(encoding: Option<&str>) -> Self {
        LDOMXmlWriter {
            encoding: encoding.map(|s| s.to_string()),
            indent: 0,
            cur_indent: 0,
        }
    }

    /// Set indentation level for pretty-printing
    pub fn set_indentation(&mut self, indent: usize) {
        self.indent = indent;
    }

    /// Get the current indentation
    pub fn cur_indent(&self) -> usize {
        self.cur_indent
    }

    /// Write a document
    pub fn write_document(&mut self, output: &mut String) {
        if let Some(ref enc) = self.encoding {
            output.push_str(&format!("<?xml version=\"1.0\" encoding=\"{}\"?>\n", enc));
        } else {
            output.push_str("<?xml version=\"1.0\"?>\n");
        }
    }

    /// Write a string
    pub fn write_string(&mut self, output: &mut String, text: &str) {
        output.push_str(text);
    }

    /// Write a character
    pub fn write_char(&mut self, output: &mut String, c: char) {
        output.push(c);
    }

    /// Increase indentation
    pub fn increase_indent(&mut self) {
        if self.indent > 0 {
            self.cur_indent += self.indent;
        }
    }

    /// Decrease indentation
    pub fn decrease_indent(&mut self) {
        if self.indent > 0 && self.cur_indent >= self.indent {
            self.cur_indent -= self.indent;
        }
    }

    /// Get the encoding
    pub fn encoding(&self) -> Option<&str> {
        self.encoding.as_deref()
    }
}

impl Default for LDOMXmlWriter {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writer_creation() {
        let writer = LDOMXmlWriter::new(None);
        assert_eq!(writer.encoding(), None);
    }

    #[test]
    fn test_writer_with_encoding() {
        let writer = LDOMXmlWriter::new(Some("UTF-8"));
        assert_eq!(writer.encoding(), Some("UTF-8"));
    }

    #[test]
    fn test_set_indentation() {
        let mut writer = LDOMXmlWriter::new(None);
        writer.set_indentation(2);
        assert_eq!(writer.indent, 2);
    }

    #[test]
    fn test_write_document() {
        let mut writer = LDOMXmlWriter::new(Some("UTF-8"));
        let mut output = String::new();
        writer.write_document(&mut output);
        assert!(output.contains("<?xml version=\"1.0\""));
        assert!(output.contains("UTF-8"));
    }

    #[test]
    fn test_write_string() {
        let mut writer = LDOMXmlWriter::new(None);
        let mut output = String::new();
        writer.write_string(&mut output, "<tag>");
        assert_eq!(output, "<tag>");
    }

    #[test]
    fn test_indentation_increase_decrease() {
        let mut writer = LDOMXmlWriter::new(None);
        writer.set_indentation(2);
        writer.increase_indent();
        assert_eq!(writer.cur_indent(), 2);
        writer.decrease_indent();
        assert_eq!(writer.cur_indent(), 0);
    }
}
