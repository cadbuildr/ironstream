// FILE: prs_mgr.rs
// occt: PrsMgr_PresentationManager, PrsMgr_Presentation,
//       PrsMgr_PresentableObject, PrsMgr_TypeOfPresentation3d

/// Type of 3D presentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfPresentation3d {
    AllView,
    ProjectorDependant,
}

impl Default for TypeOfPresentation3d {
    fn default() -> Self { Self::AllView }
}

/// Highlight mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighlightMode {
    Normal,
    Highlight,
    SubIntensity,
}

impl Default for HighlightMode {
    fn default() -> Self { Self::Normal }
}

// occt: PrsMgr_Presentation — a single presentation (mode + structure) for an object
#[derive(Clone, Debug)]
pub struct Presentation {
    pub id: u32,
    pub mode: i32,
    pub is_displayed: bool,
    pub is_highlighted: bool,
    pub highlight_mode: HighlightMode,
    pub z_layer: i32,
    pub update_needed: bool,
}

impl Presentation {
    pub fn new(id: u32, mode: i32) -> Self {
        Self {
            id, mode,
            is_displayed: false,
            is_highlighted: false,
            highlight_mode: HighlightMode::Normal,
            z_layer: 0,
            update_needed: true,
        }
    }

    pub fn display(&mut self) { self.is_displayed = true; }
    pub fn erase(&mut self) { self.is_displayed = false; }
    pub fn highlight(&mut self, mode: HighlightMode) { self.is_highlighted = true; self.highlight_mode = mode; }
    pub fn unhighlight(&mut self) { self.is_highlighted = false; self.highlight_mode = HighlightMode::Normal; }
    pub fn set_z_layer(&mut self, z: i32) { self.z_layer = z; }
    pub fn mark_updated(&mut self) { self.update_needed = false; }
}

// occt: PrsMgr_PresentableObject — any object that can be presented
#[derive(Clone, Debug)]
pub struct PresentableObject {
    pub id: u32,
    pub presentations: Vec<Presentation>,
    pub current_mode: i32,
    pub transform: [[f64; 4]; 4],
    pub clip_planes: Vec<u32>,
    pub owner_id: Option<u32>,
    pub z_layer: i32,
    pub infinite: bool,
}

impl PresentableObject {
    pub fn new(id: u32) -> Self {
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Self {
            id,
            presentations: Vec::new(),
            current_mode: 0,
            transform: identity,
            clip_planes: Vec::new(),
            owner_id: None,
            z_layer: 0,
            infinite: false,
        }
    }

    pub fn add_presentation(&mut self, mode: i32) -> u32 {
        let prs_id = self.presentations.len() as u32;
        self.presentations.push(Presentation::new(prs_id, mode));
        prs_id
    }

    pub fn presentation(&self, mode: i32) -> Option<&Presentation> {
        self.presentations.iter().find(|p| p.mode == mode)
    }

    pub fn presentation_mut(&mut self, mode: i32) -> Option<&mut Presentation> {
        self.presentations.iter_mut().find(|p| p.mode == mode)
    }

    pub fn has_presentation(&self, mode: i32) -> bool {
        self.presentations.iter().any(|p| p.mode == mode)
    }

    pub fn set_z_layer(&mut self, z: i32) {
        self.z_layer = z;
        for p in &mut self.presentations { p.set_z_layer(z); }
    }

    pub fn add_clip_plane(&mut self, plane_id: u32) { self.clip_planes.push(plane_id); }
    pub fn remove_clip_plane(&mut self, plane_id: u32) { self.clip_planes.retain(|&p| p != plane_id); }
    pub fn nb_presentations(&self) -> usize { self.presentations.len() }
    pub fn is_infinite(&self) -> bool { self.infinite }
}

// occt: PrsMgr_PresentationManager
#[derive(Clone, Debug, Default)]
pub struct PresentationManager {
    pub objects: Vec<PresentableObject>,
    pub highlighted: Vec<u32>,     // IDs of highlighted objects
    pub displayed: Vec<u32>,       // IDs of currently displayed objects
}

impl PresentationManager {
    pub fn new() -> Self { Self::default() }

    pub fn add_object(&mut self, obj: PresentableObject) -> u32 {
        let id = obj.id;
        self.objects.push(obj);
        id
    }

    pub fn object(&self, id: u32) -> Option<&PresentableObject> {
        self.objects.iter().find(|o| o.id == id)
    }

    pub fn object_mut(&mut self, id: u32) -> Option<&mut PresentableObject> {
        self.objects.iter_mut().find(|o| o.id == id)
    }

    pub fn display(&mut self, id: u32, mode: i32) -> bool {
        if let Some(obj) = self.object_mut(id) {
            if !obj.has_presentation(mode) { obj.add_presentation(mode); }
            if let Some(prs) = obj.presentation_mut(mode) { prs.display(); }
            if !self.displayed.contains(&id) { self.displayed.push(id); }
            true
        } else {
            false
        }
    }

    pub fn erase(&mut self, id: u32, mode: i32) -> bool {
        if let Some(obj) = self.object_mut(id) {
            if let Some(prs) = obj.presentation_mut(mode) { prs.erase(); }
            self.displayed.retain(|&d| d != id);
            true
        } else {
            false
        }
    }

    pub fn highlight(&mut self, id: u32, mode: i32) -> bool {
        if let Some(obj) = self.object_mut(id) {
            if let Some(prs) = obj.presentation_mut(mode) {
                prs.highlight(HighlightMode::Highlight);
            }
            if !self.highlighted.contains(&id) { self.highlighted.push(id); }
            true
        } else {
            false
        }
    }

    pub fn unhighlight(&mut self, id: u32) -> bool {
        if let Some(obj) = self.object_mut(id) {
            for prs in &mut obj.presentations { prs.unhighlight(); }
            self.highlighted.retain(|&h| h != id);
            true
        } else {
            false
        }
    }

    pub fn is_displayed(&self, id: u32, mode: i32) -> bool {
        self.object(id)
            .and_then(|o| o.presentation(mode))
            .map(|p| p.is_displayed)
            .unwrap_or(false)
    }

    pub fn is_highlighted(&self, id: u32) -> bool { self.highlighted.contains(&id) }
    pub fn nb_objects(&self) -> usize { self.objects.len() }
    pub fn nb_displayed(&self) -> usize { self.displayed.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presentation_lifecycle() {
        let mut p = Presentation::new(0, 0);
        p.display();
        assert!(p.is_displayed);
        p.highlight(HighlightMode::Highlight);
        assert!(p.is_highlighted);
        p.unhighlight();
        assert!(!p.is_highlighted);
        p.erase();
        assert!(!p.is_displayed);
    }

    #[test]
    fn presentable_object() {
        let mut obj = PresentableObject::new(1);
        obj.add_presentation(0);
        obj.add_presentation(1);
        assert_eq!(obj.nb_presentations(), 2);
        assert!(obj.has_presentation(0));
        obj.presentation_mut(0).unwrap().display();
        assert!(obj.presentation(0).unwrap().is_displayed);
    }

    #[test]
    fn presentation_manager_display() {
        let mut mgr = PresentationManager::new();
        mgr.add_object(PresentableObject::new(1));
        assert!(mgr.display(1, 0));
        assert!(mgr.is_displayed(1, 0));
        assert_eq!(mgr.nb_displayed(), 1);
    }

    #[test]
    fn presentation_manager_highlight() {
        let mut mgr = PresentationManager::new();
        mgr.add_object(PresentableObject::new(5));
        mgr.display(5, 0);
        mgr.highlight(5, 0);
        assert!(mgr.is_highlighted(5));
        mgr.unhighlight(5);
        assert!(!mgr.is_highlighted(5));
    }

    #[test]
    fn presentation_manager_erase() {
        let mut mgr = PresentationManager::new();
        mgr.add_object(PresentableObject::new(2));
        mgr.display(2, 0);
        mgr.erase(2, 0);
        assert!(!mgr.is_displayed(2, 0));
        assert_eq!(mgr.nb_displayed(), 0);
    }
}
