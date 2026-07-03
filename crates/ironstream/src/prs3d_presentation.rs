// FILE: prs3d_presentation.rs

// occt: Graphic3d_Group
pub struct PresentationGroup {
    pub primitives: Vec<String>,
    pub is_filled: bool,
    pub color: [f64; 3],
}

impl PresentationGroup {
    pub fn new() -> Self {
        PresentationGroup {
            primitives: Vec::new(),
            is_filled: false,
            color: [1.0, 1.0, 1.0],
        }
    }

    pub fn add_primitive(&mut self, p: &str) {
        self.primitives.push(p.to_string());
    }

    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.color = [r, g, b];
    }

    pub fn clear(&mut self) {
        self.primitives.clear();
        self.is_filled = false;
        self.color = [1.0, 1.0, 1.0];
    }

    pub fn num_primitives(&self) -> usize {
        self.primitives.len()
    }
}

impl Default for PresentationGroup {
    fn default() -> Self {
        Self::new()
    }
}

// occt: Prs3d_Presentation
pub struct Presentation {
    pub groups: Vec<PresentationGroup>,
    pub visible: bool,
    pub display_priority: i32,
}

impl Presentation {
    pub fn new() -> Self {
        Presentation {
            groups: Vec::new(),
            visible: true,
            display_priority: 0,
        }
    }

    pub fn new_group(&mut self) -> &mut PresentationGroup {
        self.groups.push(PresentationGroup::new());
        self.groups.last_mut().unwrap()
    }

    pub fn clear(&mut self) {
        self.groups.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    pub fn set_visible(&mut self, b: bool) {
        self.visible = b;
    }

    pub fn num_groups(&self) -> usize {
        self.groups.len()
    }
}

impl Default for Presentation {
    fn default() -> Self {
        Self::new()
    }
}

// occt: Prs3d_Presentation
pub struct PrsMgr {
    pub presentations: Vec<Presentation>,
}

impl PrsMgr {
    pub fn new() -> Self {
        PrsMgr {
            presentations: Vec::new(),
        }
    }

    pub fn add(&mut self, p: Presentation) {
        self.presentations.push(p);
    }

    pub fn remove(&mut self, i: usize) {
        self.presentations.remove(i);
    }

    pub fn count(&self) -> usize {
        self.presentations.len()
    }
}

impl Default for PrsMgr {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_group_new() {
        let g = PresentationGroup::new();
        assert_eq!(g.num_primitives(), 0);
        assert!(!g.is_filled);
        assert_eq!(g.color, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_presentation_group_add_primitive() {
        let mut g = PresentationGroup::new();
        g.add_primitive("triangle");
        g.add_primitive("line");
        assert_eq!(g.num_primitives(), 2);
        assert_eq!(g.primitives[0], "triangle");
    }

    #[test]
    fn test_presentation_group_set_color() {
        let mut g = PresentationGroup::new();
        g.set_color(0.5, 0.25, 0.75);
        assert_eq!(g.color, [0.5, 0.25, 0.75]);
    }

    #[test]
    fn test_presentation_group_clear() {
        let mut g = PresentationGroup::new();
        g.add_primitive("point");
        g.set_color(0.1, 0.2, 0.3);
        g.is_filled = true;
        g.clear();
        assert_eq!(g.num_primitives(), 0);
        assert!(!g.is_filled);
        assert_eq!(g.color, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_presentation_new() {
        let p = Presentation::new();
        assert!(p.visible);
        assert_eq!(p.display_priority, 0);
        assert!(p.is_empty());
        assert_eq!(p.num_groups(), 0);
    }

    #[test]
    fn test_presentation_new_group() {
        let mut p = Presentation::new();
        {
            let g = p.new_group();
            g.add_primitive("sphere");
        }
        assert_eq!(p.num_groups(), 1);
        assert!(!p.is_empty());
        assert_eq!(p.groups[0].primitives[0], "sphere");
    }

    #[test]
    fn test_presentation_clear() {
        let mut p = Presentation::new();
        p.new_group();
        p.new_group();
        assert_eq!(p.num_groups(), 2);
        p.clear();
        assert!(p.is_empty());
    }

    #[test]
    fn test_presentation_set_visible() {
        let mut p = Presentation::new();
        p.set_visible(false);
        assert!(!p.visible);
        p.set_visible(true);
        assert!(p.visible);
    }

    #[test]
    fn test_prsmgr_add_remove_count() {
        let mut mgr = PrsMgr::new();
        assert_eq!(mgr.count(), 0);
        mgr.add(Presentation::new());
        mgr.add(Presentation::new());
        assert_eq!(mgr.count(), 2);
        mgr.remove(0);
        assert_eq!(mgr.count(), 1);
    }
}
