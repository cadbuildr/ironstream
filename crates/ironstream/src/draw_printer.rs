// FILE: draw_printer.rs
// occt: Draw_Printer

//! Printer interface for Draw application output.

/// Draw printer for handling output
pub struct DrawPrinter {
    name: String,
}

impl DrawPrinter {
    /// Create a new printer
    pub fn new(name: impl Into<String>) -> Self {
        DrawPrinter {
            name: name.into(),
        }
    }

    /// Get the printer name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer_creation() {
        let printer = DrawPrinter::new("default");
        assert_eq!(printer.name(), "default");
    }
}
