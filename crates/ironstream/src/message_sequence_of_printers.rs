// FILE: message_sequence_of_printers.rs
// occt: Message_SequenceOfPrinters

use std::rc::Rc;
use std::cell::RefCell;

/// Message_Printer represents a printer object for outputting messages.
#[derive(Clone, Debug)]
pub struct MessagePrinter {
    name: String,
    enabled: bool,
}

impl MessagePrinter {
    pub fn new(name: String) -> Self {
        MessagePrinter { name, enabled: true }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn print(&self, message: &str) {
        if self.enabled {
            println!("[{}] {}", self.name, message);
        }
    }
}

/// A handle/reference-counted wrapper for Message_Printer.
pub type MessagePrinterHandle = Rc<RefCell<MessagePrinter>>;

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_Sequence<opencascade::handle<Message_Printer>>`
pub type MessageSequenceOfPrinters = Vec<MessagePrinterHandle>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer_creation() {
        let printer = MessagePrinter::new("ConsolePrinter".to_string());
        assert_eq!(printer.name(), "ConsolePrinter");
        assert!(printer.is_enabled());
    }

    #[test]
    fn test_printer_enable_disable() {
        let mut printer = MessagePrinter::new("TestPrinter".to_string());
        assert!(printer.is_enabled());

        printer.set_enabled(false);
        assert!(!printer.is_enabled());

        printer.set_enabled(true);
        assert!(printer.is_enabled());
    }

    #[test]
    fn test_sequence_creation() {
        let sequence: MessageSequenceOfPrinters = Vec::new();
        assert!(sequence.is_empty());
        assert_eq!(sequence.len(), 0);
    }

    #[test]
    fn test_sequence_push() {
        let mut sequence: MessageSequenceOfPrinters = Vec::new();

        let printer1 = Rc::new(RefCell::new(MessagePrinter::new(
            "Printer1".to_string(),
        )));
        let printer2 = Rc::new(RefCell::new(MessagePrinter::new(
            "Printer2".to_string(),
        )));

        sequence.push(printer1.clone());
        sequence.push(printer2.clone());

        assert_eq!(sequence.len(), 2);
        assert_eq!(sequence[0].borrow().name(), "Printer1");
        assert_eq!(sequence[1].borrow().name(), "Printer2");
    }

    #[test]
    fn test_sequence_access() {
        let mut sequence: MessageSequenceOfPrinters = Vec::new();

        let printer = Rc::new(RefCell::new(MessagePrinter::new(
            "TestPrinter".to_string(),
        )));
        sequence.push(printer.clone());

        let retrieved = sequence.get(0).unwrap();
        assert_eq!(retrieved.borrow().name(), "TestPrinter");
        assert!(retrieved.borrow().is_enabled());
    }

    #[test]
    fn test_sequence_iteration() {
        let mut sequence: MessageSequenceOfPrinters = Vec::new();

        for i in 1..=5 {
            let printer = Rc::new(RefCell::new(MessagePrinter::new(
                format!("Printer{}", i),
            )));
            sequence.push(printer);
        }

        assert_eq!(sequence.len(), 5);

        let mut names = Vec::new();
        for printer_handle in &sequence {
            names.push(printer_handle.borrow().name().to_string());
        }
        assert_eq!(names.len(), 5);
    }

    #[test]
    fn test_sequence_remove() {
        let mut sequence: MessageSequenceOfPrinters = Vec::new();

        let printer1 = Rc::new(RefCell::new(MessagePrinter::new("P1".to_string())));
        let printer2 = Rc::new(RefCell::new(MessagePrinter::new("P2".to_string())));
        let printer3 = Rc::new(RefCell::new(MessagePrinter::new("P3".to_string())));

        sequence.push(printer1);
        sequence.push(printer2);
        sequence.push(printer3);

        assert_eq!(sequence.len(), 3);
        sequence.remove(1);
        assert_eq!(sequence.len(), 2);
        assert_eq!(sequence[0].borrow().name(), "P1");
        assert_eq!(sequence[1].borrow().name(), "P3");
    }

    #[test]
    fn test_sequence_with_disabled_printers() {
        let mut sequence: MessageSequenceOfPrinters = Vec::new();

        let printer1 = Rc::new(RefCell::new(MessagePrinter::new("Enabled".to_string())));
        let printer2 = Rc::new(RefCell::new(MessagePrinter::new(
            "Disabled".to_string(),
        )));

        printer2.borrow_mut().set_enabled(false);

        sequence.push(printer1);
        sequence.push(printer2);

        let enabled_count = sequence
            .iter()
            .filter(|p| p.borrow().is_enabled())
            .count();

        assert_eq!(enabled_count, 1);
    }
}
