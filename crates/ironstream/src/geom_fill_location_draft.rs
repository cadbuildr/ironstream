// FILE: geom_fill_location_draft.rs
// occt: GeomFill_LocationDraft

/// Location law for draft surface
pub struct LocationDraft;

impl LocationDraft {
    pub fn new() -> Self {
        LocationDraft
    }
}

impl Default for LocationDraft {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _loc = LocationDraft::new();
    }

    #[test]
    fn test_default() {
        let _loc = LocationDraft::default();
    }
}
