// FILE: interface_float_writer.rs
// occt: Interface_FloatWriter

/// Writes floating-point numbers to files with controlled precision.
#[derive(Clone, Debug)]
pub struct InterfaceFloatWriter {
    precision: usize,
}

impl InterfaceFloatWriter {
    /// Creates a FloatWriter with precision
    pub fn new(precision: usize) -> Self {
        Self { precision }
    }

    /// Returns the precision
    pub fn precision(&self) -> usize {
        self.precision
    }

    /// Formats a float value according to precision
    pub fn write(&self, value: f64) -> String {
        format!("{:.prec$}", value, prec = self.precision)
    }
}

impl Default for InterfaceFloatWriter {
    fn default() -> Self {
        Self { precision: 6 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let writer = InterfaceFloatWriter::new(3);
        assert_eq!(writer.precision(), 3);
    }

    #[test]
    fn test_write() {
        let writer = InterfaceFloatWriter::new(2);
        assert_eq!(writer.write(3.14159), "3.14");
    }

    #[test]
    fn test_default() {
        let writer = InterfaceFloatWriter::default();
        assert_eq!(writer.precision(), 6);
    }
}
