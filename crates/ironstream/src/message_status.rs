// FILE: message_status.rs
// occt: Message_Status

/// Status information for messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageStatus {
    code: i32,
    message: String,
}

impl MessageStatus {
    pub fn new(code: i32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    pub fn is_error(&self) -> bool {
        self.code != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let status = MessageStatus::new(0, "OK");
        assert_eq!(status.code(), 0);
        assert_eq!(status.message(), "OK");
    }

    #[test]
    fn test_is_success() {
        let ok = MessageStatus::new(0, "OK");
        assert!(ok.is_success());
        assert!(!ok.is_error());

        let err = MessageStatus::new(1, "Error");
        assert!(!err.is_success());
        assert!(err.is_error());
    }
}
