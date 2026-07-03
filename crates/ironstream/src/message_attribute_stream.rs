// FILE: message_attribute_stream.rs
// occt: Message_AttributeStream

/// Stream attribute for message data.
pub struct MessageAttributeStream {
    name: String,
    data: Vec<u8>,
}

impl MessageAttributeStream {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn add_data(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stream = MessageAttributeStream::new("stream");
        assert_eq!(stream.name(), "stream");
        assert!(stream.data().is_empty());
    }

    #[test]
    fn test_add_data() {
        let mut stream = MessageAttributeStream::new("stream");
        stream.add_data(b"test");
        assert_eq!(stream.data(), b"test");
    }
}
