// FILE: ais_global_status.rs
// occt: AIS_GlobalStatus

#[derive(Clone, Debug, Default)]
pub struct GlobalStatus {
    display_mode: i32,
    is_hilited: bool,
    sub_intensity: bool,
    selection_modes: Vec<i32>,
}

impl GlobalStatus {
    pub fn new() -> Self {
        Self { display_mode: 0, is_hilited: false, sub_intensity: false, selection_modes: Vec::new() }
    }
    pub fn display_mode(&self) -> i32 { self.display_mode }
    pub fn set_display_mode(&mut self, mode: i32) { self.display_mode = mode; }
    pub fn is_hilighted(&self) -> bool { self.is_hilited }
    pub fn set_hilight_status(&mut self, status: bool) { self.is_hilited = status; }
    pub fn is_sub_intensity_on(&self) -> bool { self.sub_intensity }
    pub fn set_sub_intensity(&mut self, is_on: bool) { self.sub_intensity = is_on; }
    pub fn selection_modes(&self) -> &[i32] { &self.selection_modes }
    pub fn is_smode_in(&self, mode: i32) -> bool { self.selection_modes.contains(&mode) }
    pub fn add_selection_mode(&mut self, mode: i32) -> bool {
        if !self.selection_modes.contains(&mode) { self.selection_modes.push(mode); true } else { false }
    }
    pub fn remove_selection_mode(&mut self, mode: i32) -> bool {
        if let Some(pos) = self.selection_modes.iter().position(|&m| m == mode) {
            self.selection_modes.remove(pos);
            true
        } else { false }
    }
    pub fn clear_selection_modes(&mut self) { self.selection_modes.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = GlobalStatus::new();
        assert_eq!(s.display_mode(), 0);
    }
}
