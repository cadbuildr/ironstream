// FILE: message_o.rs
// occt: Message

/// Message interface base class for application messaging and alerts.
pub struct Message {
    text: String,
}

impl Message {
    /// Constructs a message.
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }

    /// Returns the message text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Sets the message text.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let msg = Message::new("Test message");
        assert_eq!(msg.text(), "Test message");
    }

    #[test]
    fn test_set_text() {
        let mut msg = Message::new("Initial");
        msg.set_text("Updated");
        assert_eq!(msg.text(), "Updated");
    }
}
