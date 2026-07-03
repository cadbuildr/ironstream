// FILE: message_printer_o_stream.rs
// occt: Message_PrinterOStream

/// Output stream printer for messages.
pub struct MessagePrinterOStream {
    name: String,
    buffer: Vec<String>,
}

impl MessagePrinterOStream {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            buffer: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn print(&mut self, msg: &str) {
        self.buffer.push(msg.to_string());
    }

    pub fn buffer(&self) -> &[String] {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let printer = MessagePrinterOStream::new("stdout");
        assert_eq!(printer.name(), "stdout");
    }

    #[test]
    fn test_print() {
        let mut printer = MessagePrinterOStream::new("stdout");
        printer.print("message");
        assert_eq!(printer.buffer().len(), 1);
    }
}
