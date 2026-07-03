// FILE: ais_exclusion_filter.rs
// occt: AIS_ExclusionFilter

#[derive(Clone, Debug, Default)]
pub struct ExclusionFilter {
    excluded_ids: Vec<u32>,
}

impl ExclusionFilter {
    pub fn new() -> Self { Self { excluded_ids: Vec::new() } }
    pub fn add_excluded(&mut self, id: u32) {
        if !self.excluded_ids.contains(&id) { self.excluded_ids.push(id); }
    }
    pub fn remove_excluded(&mut self, id: u32) { self.excluded_ids.retain(|&x| x != id); }
    pub fn is_excluded(&self, id: u32) -> bool { self.excluded_ids.contains(&id) }
    pub fn clear(&mut self) { self.excluded_ids.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let f = ExclusionFilter::new();
        assert!(!f.is_excluded(1));
    }
}
