// FILE: graphic3d_present.rs
// occt: Graphic3d_Camera (extended), Graphic3d_Presentation,
//       Graphic3d_SequenceOfGroup, PrsMgr_PresentationManager

/// Priority of a presentation.
/// occt: PrsMgr_PresentationManager
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PresentationPriority(pub i32);

impl Default for PresentationPriority {
    fn default() -> Self { Self(0) }
}

impl PresentationPriority {
    pub const DEFAULT: Self = Self(0);
    pub const HIGH: Self = Self(5);
    pub const LOW: Self = Self(-5);

    pub fn value(self) -> i32 { self.0 }
}

/// Display mode index.
/// occt: AIS_DisplayMode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayMode {
    WireFrame = 0,
    Shading = 1,
    ShadingWithEdges = 2,
    Highlight = 3,
}

impl Default for DisplayMode {
    fn default() -> Self { Self::Shading }
}

/// A single group of primitives in a presentation (GL group).
/// occt: Graphic3d_Group
#[derive(Clone, Debug)]
pub struct Graphic3dGroup {
    pub group_id: u32,
    pub is_closed: bool,
    pub nb_primitives: usize,
    pub is_visible: bool,
    pub transformation: Option<[f64; 16]>,  // 4×4 optional local transform
}

impl Graphic3dGroup {
    pub fn new(group_id: u32) -> Self {
        Self { group_id, is_closed: false, nb_primitives: 0, is_visible: true, transformation: None }
    }

    pub fn add_primitive(&mut self) { self.nb_primitives += 1; }
    pub fn set_closed(&mut self, v: bool) { self.is_closed = v; }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
    pub fn set_transform(&mut self, t: [f64; 16]) { self.transformation = Some(t); }
    pub fn clear_transform(&mut self) { self.transformation = None; }

    pub fn is_visible(&self) -> bool { self.is_visible }
    pub fn nb_primitives(&self) -> usize { self.nb_primitives }
    pub fn is_closed(&self) -> bool { self.is_closed }
}

/// A 3D presentation: a collection of groups for one object + one display mode.
/// occt: Graphic3d_Structure / Prs3d_Presentation
#[derive(Clone, Debug)]
pub struct Graphic3dPresentation {
    pub presentation_id: u32,
    pub display_mode: DisplayMode,
    pub priority: PresentationPriority,
    pub groups: Vec<Graphic3dGroup>,
    pub is_displayed: bool,
    pub is_highlighted: bool,
    pub z_layer_id: i32,
    pub clipping_plane_ids: Vec<u32>,
}

impl Graphic3dPresentation {
    pub fn new(presentation_id: u32, mode: DisplayMode) -> Self {
        Self {
            presentation_id,
            display_mode: mode,
            priority: PresentationPriority::DEFAULT,
            groups: Vec::new(),
            is_displayed: false,
            is_highlighted: false,
            z_layer_id: 0,
            clipping_plane_ids: Vec::new(),
        }
    }

    pub fn new_group(&mut self) -> &mut Graphic3dGroup {
        let id = self.groups.len() as u32 + 1;
        self.groups.push(Graphic3dGroup::new(id));
        self.groups.last_mut().unwrap()
    }

    pub fn current_group(&mut self) -> Option<&mut Graphic3dGroup> {
        self.groups.last_mut()
    }

    pub fn nb_groups(&self) -> usize { self.groups.len() }

    pub fn display(&mut self) { self.is_displayed = true; }
    pub fn erase(&mut self) { self.is_displayed = false; }
    pub fn highlight(&mut self) { self.is_highlighted = true; }
    pub fn unhighlight(&mut self) { self.is_highlighted = false; }

    pub fn set_z_layer(&mut self, id: i32) { self.z_layer_id = id; }
    pub fn set_priority(&mut self, p: PresentationPriority) { self.priority = p; }

    pub fn add_clipping_plane(&mut self, plane_id: u32) {
        if !self.clipping_plane_ids.contains(&plane_id) {
            self.clipping_plane_ids.push(plane_id);
        }
    }

    pub fn remove_clipping_plane(&mut self, plane_id: u32) {
        self.clipping_plane_ids.retain(|&id| id != plane_id);
    }

    pub fn is_displayed(&self) -> bool { self.is_displayed }
    pub fn is_highlighted(&self) -> bool { self.is_highlighted }
    pub fn display_mode(&self) -> DisplayMode { self.display_mode }
}

/// Presentation manager: owns presentations for all interactive objects.
/// occt: PrsMgr_PresentationManager
#[derive(Clone, Debug, Default)]
pub struct PrsMgrPresentationManager {
    pub presentations: Vec<Graphic3dPresentation>,
}

impl PrsMgrPresentationManager {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, p: Graphic3dPresentation) -> u32 {
        let id = p.presentation_id;
        self.presentations.push(p);
        id
    }

    pub fn find(&self, id: u32) -> Option<&Graphic3dPresentation> {
        self.presentations.iter().find(|p| p.presentation_id == id)
    }

    pub fn find_mut(&mut self, id: u32) -> Option<&mut Graphic3dPresentation> {
        self.presentations.iter_mut().find(|p| p.presentation_id == id)
    }

    pub fn nb_presentations(&self) -> usize { self.presentations.len() }

    pub fn nb_displayed(&self) -> usize {
        self.presentations.iter().filter(|p| p.is_displayed).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presentation_groups() {
        let mut p = Graphic3dPresentation::new(1, DisplayMode::Shading);
        p.new_group();
        p.new_group();
        assert_eq!(p.nb_groups(), 2);
        p.display();
        assert!(p.is_displayed());
        p.highlight();
        assert!(p.is_highlighted());
    }

    #[test]
    fn presentation_clipping_planes() {
        let mut p = Graphic3dPresentation::new(1, DisplayMode::WireFrame);
        p.add_clipping_plane(10);
        p.add_clipping_plane(20);
        p.add_clipping_plane(10); // duplicate
        assert_eq!(p.clipping_plane_ids.len(), 2);
        p.remove_clipping_plane(10);
        assert_eq!(p.clipping_plane_ids.len(), 1);
    }

    #[test]
    fn presentation_manager_find() {
        let mut mgr = PrsMgrPresentationManager::new();
        mgr.add(Graphic3dPresentation::new(1, DisplayMode::Shading));
        mgr.add(Graphic3dPresentation::new(2, DisplayMode::WireFrame));
        assert_eq!(mgr.nb_presentations(), 2);
        let p = mgr.find(2).unwrap();
        assert_eq!(p.display_mode(), DisplayMode::WireFrame);
    }

    #[test]
    fn presentation_manager_nb_displayed() {
        let mut mgr = PrsMgrPresentationManager::new();
        let mut p1 = Graphic3dPresentation::new(1, DisplayMode::Shading);
        p1.display();
        mgr.add(p1);
        mgr.add(Graphic3dPresentation::new(2, DisplayMode::Shading));
        assert_eq!(mgr.nb_displayed(), 1);
    }

    #[test]
    fn priority_ordering() {
        assert!(PresentationPriority::HIGH > PresentationPriority::DEFAULT);
        assert!(PresentationPriority::DEFAULT > PresentationPriority::LOW);
    }
}
