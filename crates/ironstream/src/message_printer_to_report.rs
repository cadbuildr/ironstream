// FILE: message_printer_to_report.rs
// occt: Message_PrinterToReport

/// Report printer for messages.
pub struct MessagePrinterToReport {
    name: String,
    report: Vec<String>,
}

impl MessagePrinterToReport {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            report: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn print(&mut self, msg: &str) {
        self.report.push(msg.to_string());
    }

    pub fn report(&self) -> String {
        self.report.join("\n")
    }

    pub fn clear(&mut self) {
        self.report.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let printer = MessagePrinterToReport::new("report");
        assert_eq!(printer.name(), "report");
    }

    #[test]
    fn test_print() {
        let mut printer = MessagePrinterToReport::new("report");
        printer.print("line 1");
        printer.print("line 2");
        assert!(printer.report().contains("line 1"));
        assert!(printer.report().contains("line 2"));
    }
}
