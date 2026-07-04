// FILE: t_doc_std_path_parser.rs
// occt: TDocStd_PathParser

/// Parses and manages document file paths and entries.
#[derive(Clone, Debug)]
pub struct TDocStd_PathParser {
    full_path: String,
    file_path: String,
    entry: String,
}

impl TDocStd_PathParser {
    /// Create a new path parser.
    pub fn new() -> Self {
        Self {
            full_path: String::new(),
            file_path: String::new(),
            entry: String::new(),
        }
    }

    /// Parse a full path and entry.
    pub fn parse(full_path: &str) -> Self {
        // Simple parsing: split by '#'
        let parts: Vec<&str> = full_path.splitn(2, '#').collect();
        let (file_path, entry) = if parts.len() == 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            (full_path.to_string(), String::new())
        };

        Self {
            full_path: full_path.to_string(),
            file_path,
            entry,
        }
    }

    /// Get the full path.
    pub fn full_path(&self) -> &str {
        &self.full_path
    }

    /// Get the file path.
    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    /// Get the entry.
    pub fn entry(&self) -> &str {
        &self.entry
    }

    /// Set file path and entry.
    pub fn set_path(&mut self, file_path: String, entry: String) {
        self.file_path = file_path.clone();
        self.entry = entry;
        self.full_path = format!("{}#{}", file_path, self.entry);
    }
}

impl Default for TDocStd_PathParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_parser() {
        let parser = TDocStd_PathParser::new();
        assert_eq!(parser.full_path(), "");
    }

    #[test]
    fn test_parse_simple() {
        let parser = TDocStd_PathParser::parse("file.xml");
        assert_eq!(parser.file_path(), "file.xml");
        assert_eq!(parser.entry(), "");
    }

    #[test]
    fn test_parse_with_entry() {
        let parser = TDocStd_PathParser::parse("file.xml#0:1:2");
        assert_eq!(parser.file_path(), "file.xml");
        assert_eq!(parser.entry(), "0:1:2");
    }

    #[test]
    fn test_set_path() {
        let mut parser = TDocStd_PathParser::new();
        parser.set_path("doc.xml".to_string(), "0:1:3".to_string());
        assert_eq!(parser.file_path(), "doc.xml");
        assert_eq!(parser.entry(), "0:1:3");
    }

    #[test]
    fn test_default() {
        let parser = TDocStd_PathParser::default();
        assert_eq!(parser.file_path(), "");
    }
}
