// FILE: prs3d_scene.rs
// occt: PrsMgr_PresentationManager, PrsMgr_Presentation, StdPrs_WFShape, StdPrs_ShadedShape

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayMode {
    WireFrame   = 0,
    Shaded      = 1,
    ShadedWireFrame = 2,
    Bbox        = 3,
    Transparent = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresentationState {
    Displayed,
    Erased,
    Highlighted,
}

/// A presentation (visual representation) of a shape.
#[derive(Clone, Debug)]
pub struct Presentation {
    pub id: u64,
    pub shape_id: u64,
    pub display_mode: DisplayMode,
    pub state: PresentationState,
    pub highlight_color: [f32; 4],
    pub layer: i32,
    pub nb_triangles: usize,
    pub nb_edges: usize,
}

impl Presentation {
    pub fn new(id: u64, shape_id: u64, mode: DisplayMode) -> Self {
        Self {
            id, shape_id, display_mode: mode, state: PresentationState::Erased,
            highlight_color: [1.0, 0.85, 0.0, 1.0], layer: 0,
            nb_triangles: 0, nb_edges: 0,
        }
    }

    pub fn display(&mut self) { self.state = PresentationState::Displayed; }
    pub fn erase(&mut self) { self.state = PresentationState::Erased; }
    pub fn highlight(&mut self) { self.state = PresentationState::Highlighted; }
    pub fn unhighlight(&mut self) { self.state = PresentationState::Displayed; }
    pub fn is_displayed(&self) -> bool { self.state != PresentationState::Erased }
    pub fn is_highlighted(&self) -> bool { self.state == PresentationState::Highlighted }
}

// occt: PrsMgr_PresentationManager
#[derive(Clone, Debug, Default)]
pub struct PresentationManager {
    pub presentations: Vec<Presentation>,
    next_id: u64,
}

impl PresentationManager {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, shape_id: u64, mode: DisplayMode) -> u64 {
        let id = self.next_id; self.next_id += 1;
        self.presentations.push(Presentation::new(id, shape_id, mode));
        id
    }

    pub fn display(&mut self, id: u64) {
        if let Some(p) = self.presentations.iter_mut().find(|p| p.id == id) { p.display(); }
    }

    pub fn erase(&mut self, id: u64) {
        if let Some(p) = self.presentations.iter_mut().find(|p| p.id == id) { p.erase(); }
    }

    pub fn highlight(&mut self, id: u64) {
        if let Some(p) = self.presentations.iter_mut().find(|p| p.id == id) { p.highlight(); }
    }

    pub fn find(&self, id: u64) -> Option<&Presentation> { self.presentations.iter().find(|p| p.id == id) }
    pub fn nb_displayed(&self) -> usize { self.presentations.iter().filter(|p| p.is_displayed()).count() }
    pub fn nb_highlighted(&self) -> usize { self.presentations.iter().filter(|p| p.is_highlighted()).count() }
    pub fn erase_all(&mut self) { for p in &mut self.presentations { p.erase(); } }
}

// occt: StdPrs_WFShape / StdPrs_ShadedShape helper
pub struct SceneBuilder;

impl SceneBuilder {
    /// Build a wireframe presentation (counts edges).
    pub fn wireframe(nb_edges: usize) -> Presentation {
        let mut p = Presentation::new(0, 0, DisplayMode::WireFrame);
        p.nb_edges = nb_edges;
        p.display();
        p
    }

    /// Build a shaded presentation (counts triangles).
    pub fn shaded(nb_triangles: usize) -> Presentation {
        let mut p = Presentation::new(0, 0, DisplayMode::Shaded);
        p.nb_triangles = nb_triangles;
        p.display();
        p
    }

    /// Compute "coverage" metric: higher = more detail.
    pub fn coverage(p: &Presentation) -> f64 {
        match p.display_mode {
            DisplayMode::WireFrame => p.nb_edges as f64,
            DisplayMode::Shaded => p.nb_triangles as f64 * 3.0,
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presentation_display_erase() {
        let mut p = Presentation::new(1, 10, DisplayMode::Shaded);
        p.display();
        assert!(p.is_displayed());
        p.erase();
        assert!(!p.is_displayed());
    }

    #[test]
    fn presentation_highlight() {
        let mut p = Presentation::new(1, 10, DisplayMode::WireFrame);
        p.display();
        p.highlight();
        assert!(p.is_highlighted());
        p.unhighlight();
        assert!(!p.is_highlighted());
    }

    #[test]
    fn manager_basic() {
        let mut mgr = PresentationManager::new();
        let id = mgr.add(5, DisplayMode::Shaded);
        mgr.display(id);
        assert_eq!(mgr.nb_displayed(), 1);
        mgr.erase(id);
        assert_eq!(mgr.nb_displayed(), 0);
    }

    #[test]
    fn scene_builder() {
        let wf = SceneBuilder::wireframe(100);
        assert!(wf.is_displayed());
        assert_eq!(wf.nb_edges, 100);
        let sh = SceneBuilder::shaded(500);
        assert!(sh.is_displayed());
        assert!(SceneBuilder::coverage(&sh) > SceneBuilder::coverage(&wf));
    }
}
