// FILE: loc_ope_wires_on_shape.rs
// occt: LocOpe_WiresOnShape

/// Container for wires on a shape in local operations.
pub struct LocOpeWiresOnShape {
    shape_id: usize,
    wire_ids: Vec<usize>,
}

impl LocOpeWiresOnShape {
    /// Creates a new wires on shape container.
    pub fn new() -> Self {
        LocOpeWiresOnShape {
            shape_id: 0,
            wire_ids: Vec::new(),
        }
    }

    /// Initializes with a shape.
    pub fn init(&mut self, _shape_id: usize) {
        // Initialize
    }

    /// Adds a wire.
    pub fn add_wire(&mut self, _wire_id: usize) {
        // Add wire
    }

    /// Returns the number of wires.
    pub fn nb_wires(&self) -> usize {
        self.wire_ids.len()
    }

    /// Gets the wire at index.
    pub fn wire(&self, _index: usize) -> Option<usize> {
        None
    }
}

impl Default for LocOpeWiresOnShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wires_on_shape_creation() {
        let wos = LocOpeWiresOnShape::new();
        assert_eq!(wos.nb_wires(), 0);
    }
}
