// FILE: vrml_info.rs
// occt: Vrml_Info
//
// Faithful port of OCCT Vrml_Info (DataExchange/TKDEVRML/Vrml/Vrml_Info.hxx):
// the VRML 1.0 `Info` node, used to embed string annotations. Default string
// is empty. Print emits the string field with quoted value.

/// Port of Vrml_Info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VrmlInfo {
    my_string: String,
}

impl VrmlInfo {
    /// Vrml_Info(): default string is empty.
    pub fn new() -> Self {
        VrmlInfo {
            my_string: String::new(),
        }
    }

    /// Vrml_Info(const TCollection_AsciiString& aString).
    pub fn with_string(a_string: String) -> Self {
        VrmlInfo {
            my_string: a_string,
        }
    }

    pub fn set_string(&mut self, a_string: String) {
        self.my_string = a_string;
    }

    pub fn string(&self) -> &str {
        &self.my_string
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("Info {\n");

        if !self.my_string.is_empty() {
            an_ostream.push_str("    string\t\"");
            // Escape special characters: " becomes \", \ becomes \\
            for ch in self.my_string.chars() {
                match ch {
                    '"' => an_ostream.push_str("\\\""),
                    '\\' => an_ostream.push_str("\\\\"),
                    _ => an_ostream.push(ch),
                }
            }
            an_ostream.push_str("\"\n");
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_empty_string() {
        let info = VrmlInfo::new();
        assert_eq!(info.string(), "");
        let mut out = String::new();
        info.print(&mut out);
        assert_eq!(out, "Info {\n}\n");
    }

    #[test]
    fn custom_string_prints_field() {
        let info = VrmlInfo::with_string("Hello World".to_string());
        let mut out = String::new();
        info.print(&mut out);
        assert_eq!(out, "Info {\n    string\t\"Hello World\"\n}\n");
    }

    #[test]
    fn string_with_quotes_escaped() {
        let info = VrmlInfo::with_string("Say \"Hi\"".to_string());
        let mut out = String::new();
        info.print(&mut out);
        assert_eq!(out, "Info {\n    string\t\"Say \\\"Hi\\\"\"\n}\n");
    }

    #[test]
    fn string_with_backslash_escaped() {
        let info = VrmlInfo::with_string("Path\\to\\file".to_string());
        let mut out = String::new();
        info.print(&mut out);
        assert_eq!(out, "Info {\n    string\t\"Path\\\\to\\\\file\"\n}\n");
    }

    #[test]
    fn setter_updates_string() {
        let mut info = VrmlInfo::new();
        info.set_string("Test".to_string());
        assert_eq!(info.string(), "Test");
        let mut out = String::new();
        info.print(&mut out);
        assert_eq!(out, "Info {\n    string\t\"Test\"\n}\n");
    }
}
