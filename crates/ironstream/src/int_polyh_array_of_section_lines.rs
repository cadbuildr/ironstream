// FILE: int_polyh_array_of_section_lines.rs
// occt: IntPolyh_ArrayOfSectionLines

/// Implementation of IntPolyh_ArrayOfSectionLines
pub struct IntPolyh_ArrayOfSectionLines;

impl IntPolyh_ArrayOfSectionLines {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPolyh_ArrayOfSectionLines
    }
}

impl Default for IntPolyh_ArrayOfSectionLines {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPolyh_ArrayOfSectionLines::new();
    }
}
