// FILE: iges_graph_line_font_def_pattern_o.rs
// occt: IGESGraph_LineFontDefPattern

/// Represents an IGES Line Font Definition Pattern entity (Type 304, Form 2).
/// Line Font is defined by repetition of a basic pattern of visible-blank
/// (on-off) segments superimposed on a line or curve.
pub struct IgesGraphLineFontDefPattern {
    segment_lengths: Vec<f64>,
    display_pattern: Option<String>,
}

impl IgesGraphLineFontDefPattern {
    /// Creates a new empty LineFontDefPattern entity.
    pub fn new() -> Self {
        IgesGraphLineFontDefPattern {
            segment_lengths: Vec::new(),
            display_pattern: None,
        }
    }

    /// Sets the fields of the LineFontDefPattern entity.
    ///
    /// # Arguments
    /// - `all_seg_length`: Vector of lengths of respective segments
    /// - `pattern`: Pattern string indicating visible-blank segments
    pub fn init(&mut self, all_seg_length: Vec<f64>, pattern: Option<String>) {
        self.segment_lengths = all_seg_length;
        self.display_pattern = pattern;
    }

    /// Returns the number of segments in the visible-blank pattern.
    pub fn nb_segments(&self) -> i32 {
        self.segment_lengths.len() as i32
    }

    /// Returns the length of the segment at the given index (1-based).
    ///
    /// # Arguments
    /// - `index`: 1-based index into segment lengths
    pub fn length(&self, index: i32) -> Option<f64> {
        if index > 0 && (index as usize) <= self.segment_lengths.len() {
            Some(self.segment_lengths[(index - 1) as usize])
        } else {
            None
        }
    }

    /// Returns the string indicating which segments are visible and which are blanked.
    ///
    /// Example: "2H16" means bits pattern = 10110 (hex 16), so segments 2, 3, 5 are visible.
    pub fn display_pattern(&self) -> Option<&str> {
        self.display_pattern.as_deref()
    }

    /// Checks if the segment at the given index is visible.
    ///
    /// # Arguments
    /// - `index`: 1-based segment index
    pub fn is_visible(&self, index: i32) -> bool {
        if index <= 0 || (index as usize) > self.segment_lengths.len() {
            return false;
        }

        // Parse the display pattern to determine visibility
        if let Some(pattern) = &self.display_pattern {
            // Extract hex value from pattern string
            // Format is typically "2H<hex>" where hex indicates visible segments
            if let Some(hex_part) = pattern.split('H').nth(1) {
                if let Ok(value) = i32::from_str_radix(hex_part, 16) {
                    // Right-justified bits: check if bit (index-1) is set
                    let bit_pos = index - 1;
                    return (value & (1 << bit_pos)) != 0;
                }
            }
        }
        false
    }

    /// Returns all segment lengths.
    pub fn segment_lengths(&self) -> &[f64] {
        &self.segment_lengths
    }
}

impl Default for IgesGraphLineFontDefPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_font_def_pattern_creation() {
        let lfdp = IgesGraphLineFontDefPattern::new();
        assert_eq!(lfdp.nb_segments(), 0);
        assert_eq!(lfdp.display_pattern(), None);
    }

    #[test]
    fn test_line_font_def_pattern_init() {
        let mut lfdp = IgesGraphLineFontDefPattern::new();
        lfdp.init(vec![1.0, 2.0, 3.0], Some("2H16".to_string()));
        assert_eq!(lfdp.nb_segments(), 3);
        assert_eq!(lfdp.display_pattern(), Some("2H16"));
    }

    #[test]
    fn test_line_font_def_pattern_length() {
        let mut lfdp = IgesGraphLineFontDefPattern::new();
        lfdp.init(vec![1.5, 2.5, 3.5], None);
        assert_eq!(lfdp.length(1), Some(1.5));
        assert_eq!(lfdp.length(2), Some(2.5));
        assert_eq!(lfdp.length(3), Some(3.5));
        assert_eq!(lfdp.length(0), None);
        assert_eq!(lfdp.length(4), None);
    }

    #[test]
    fn test_line_font_def_pattern_is_visible() {
        let mut lfdp = IgesGraphLineFontDefPattern::new();
        lfdp.init(vec![1.0, 2.0, 3.0, 4.0, 5.0], Some("2H16".to_string()));
        // 0x16 = 10110 in binary (right-justified bits)
        // Bit 0 (segment 1): 0 (blank)
        // Bit 1 (segment 2): 1 (visible)
        // Bit 2 (segment 3): 1 (visible)
        // Bit 3 (segment 4): 0 (blank)
        // Bit 4 (segment 5): 1 (visible)
        assert!(!lfdp.is_visible(1));
        assert!(lfdp.is_visible(2));
        assert!(lfdp.is_visible(3));
        assert!(!lfdp.is_visible(4));
        assert!(lfdp.is_visible(5));
    }

    #[test]
    fn test_line_font_def_pattern_is_visible_invalid_index() {
        let mut lfdp = IgesGraphLineFontDefPattern::new();
        lfdp.init(vec![1.0, 2.0], Some("2H3".to_string()));
        assert!(!lfdp.is_visible(0));
        assert!(!lfdp.is_visible(3));
    }

    #[test]
    fn test_line_font_def_pattern_is_visible_no_pattern() {
        let mut lfdp = IgesGraphLineFontDefPattern::new();
        lfdp.init(vec![1.0, 2.0], None);
        assert!(!lfdp.is_visible(1));
    }
}
