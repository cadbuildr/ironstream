// FILE: message_printer_system_log.rs
// occt: Message_PrinterSystemLog

/// System log printer for messages.
pub struct MessagePrinterSystemLog {
    name: String,
}

impl MessagePrinterSystemLog {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn print(&self, msg: &str) {
        eprintln!("[{}] {}", self.name, msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let printer = MessagePrinterSystemLog::new("syslog");
        assert_eq!(printer.name(), "syslog");
    }

    #[test]
    fn test_print() {
        let printer = MessagePrinterSystemLog::new("syslog");
        printer.print("test message");
    }
}
