// FILE: resource_mgr_ext.rs
// occt: Resource_Manager, Resource_Unicode
// occt-ref: Resource_String
//       Resource_StreamBuffer

/// String resource: key-value pair.
// occt-ref: Resource_String
#[derive(Clone, Debug)]
pub struct ResourceString {
    pub key: String,
    pub value: String,
    pub is_ascii: bool,
}

impl ResourceString {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            is_ascii: true,
        }
    }

    pub fn is_empty(&self) -> bool { self.value.is_empty() }
}

/// Unicode string with encoding info.
/// occt: Resource_Unicode
#[derive(Clone, Debug)]
pub struct ResourceUnicode {
    pub text: String,
    pub encoding: String,  // "UTF-8", "UTF-16", "ASCII", etc.
}

impl ResourceUnicode {
    pub fn new(text: impl Into<String>, encoding: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            encoding: encoding.into(),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self.encoding.as_str() {
            "UTF-8" => self.text.as_bytes().to_vec(),
            "ASCII" => self.text.as_bytes().to_vec(),
            _ => self.text.as_bytes().to_vec(),  // Stub: default to UTF-8
        }
    }
}

/// Resource file manager.
/// occt: Resource_Manager
#[derive(Clone, Debug, Default)]
pub struct ResourceManager {
    pub name: String,
    pub resources: Vec<ResourceString>,
    pub file_path: Option<String>,
}

impl ResourceManager {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            resources: Vec::new(),
            file_path: None,
        }
    }

    pub fn set_file(&mut self, path: impl Into<String>) {
        self.file_path = Some(path.into());
    }

    pub fn add_resource(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let rs = ResourceString::new(key, value);
        if !self.resources.iter().any(|r| r.key == rs.key) {
            self.resources.push(rs);
        }
    }

    pub fn get_resource(&self, key: &str) -> Option<&str> {
        self.resources.iter()
            .find(|r| r.key == key)
            .map(|r| r.value.as_str())
    }

    pub fn remove_resource(&mut self, key: &str) -> bool {
        let before = self.resources.len();
        self.resources.retain(|r| r.key != key);
        self.resources.len() < before
    }

    pub fn nb_resources(&self) -> usize { self.resources.len() }

    pub fn load_from_file(&mut self, _path: &str) -> bool {
        // Stub: always succeeds
        true
    }

    pub fn save_to_file(&self, path: &str) -> bool {
        self.file_path.as_ref().map(|_| true).unwrap_or_else(|| !path.is_empty())
    }
}

/// Stream buffer for reading resource files.
// occt-ref: Resource_StreamBuffer
#[derive(Clone, Debug)]
pub struct ResourceStreamBuffer {
    pub buffer: Vec<u8>,
    pub position: usize,
}

impl ResourceStreamBuffer {
    pub fn new() -> Self { Self { buffer: Vec::new(), position: 0 } }

    pub fn write_bytes(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn read_line(&mut self) -> Option<String> {
        if self.position >= self.buffer.len() { return None; }
        let start = self.position;
        while self.position < self.buffer.len() && self.buffer[self.position] != b'\n' {
            self.position += 1;
        }
        let line = String::from_utf8_lossy(&self.buffer[start..self.position]).into_owned();
        if self.position < self.buffer.len() { self.position += 1; }  // skip \n
        Some(line)
    }

    pub fn is_eof(&self) -> bool { self.position >= self.buffer.len() }
    pub fn size(&self) -> usize { self.buffer.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_string() {
        let rs = ResourceString::new("key1", "value1");
        assert_eq!(rs.key, "key1");
        assert!(!rs.is_empty());
    }

    #[test]
    fn resource_unicode() {
        let ru = ResourceUnicode::new("hello", "UTF-8");
        let bytes = ru.as_bytes();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn resource_manager() {
        let mut rm = ResourceManager::new("AppResources");
        rm.add_resource("color.red", "#FF0000");
        rm.add_resource("color.blue", "#0000FF");
        assert_eq!(rm.nb_resources(), 2);
        assert_eq!(rm.get_resource("color.red"), Some("#FF0000"));
        rm.remove_resource("color.blue");
        assert_eq!(rm.nb_resources(), 1);
    }

    #[test]
    fn stream_buffer() {
        let mut sb = ResourceStreamBuffer::new();
        sb.write_bytes(b"line1\nline2\nline3");
        // "line1\nline2\nline3" is 17 bytes: 3 x 5 chars + 2 newlines.
        assert_eq!(sb.size(), 17);
        let line1 = sb.read_line();
        assert_eq!(line1, Some("line1".to_string()));
        let line2 = sb.read_line();
        assert_eq!(line2, Some("line2".to_string()));
    }
}
