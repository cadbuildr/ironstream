// FILE: xs_algo_shape_processor.rs
// occt: XSAlgo_ShapeProcessor

/// Processor for shape-related operations in the exchange framework.
/// Provides methods for processing and transforming shapes during transfer.
#[derive(Clone, Debug)]
pub struct XSAlgoShapeProcessor {
    /// Processor identifier
    processor_id: u32,
    /// Processing tolerance
    tolerance: f64,
    /// Processing flags
    flags: u32,
}

impl XSAlgoShapeProcessor {
    /// Creates a new shape processor.
    pub fn new() -> Self {
        Self {
            processor_id: 1,
            tolerance: 0.00001,
            flags: 0,
        }
    }

    /// Returns the processor ID.
    pub fn id(&self) -> u32 {
        self.processor_id
    }

    /// Returns the processing tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Sets the processing tolerance.
    pub fn set_tolerance(&mut self, tolerance: f64) {
        if tolerance > 0.0 {
            self.tolerance = tolerance;
        }
    }

    /// Returns the processing flags.
    pub fn flags(&self) -> u32 {
        self.flags
    }

    /// Sets the processing flags.
    pub fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }
}

impl Default for XSAlgoShapeProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let processor = XSAlgoShapeProcessor::new();
        assert_eq!(processor.id(), 1);
        assert_eq!(processor.tolerance(), 0.00001);
        assert_eq!(processor.flags(), 0);
    }

    #[test]
    fn test_set_tolerance() {
        let mut processor = XSAlgoShapeProcessor::new();
        processor.set_tolerance(0.0001);
        assert_eq!(processor.tolerance(), 0.0001);

        processor.set_tolerance(-1.0);
        assert_eq!(processor.tolerance(), 0.0001); // Should not change for negative
    }

    #[test]
    fn test_set_flags() {
        let mut processor = XSAlgoShapeProcessor::new();
        processor.set_flags(0xFF);
        assert_eq!(processor.flags(), 0xFF);
    }
}
