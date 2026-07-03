// FILE: ais.rs

// occt: AIS_DisplayMode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisDisplayMode {
    Wireframe = 0,
    Shaded = 1,
}

// occt: AIS_SelectionMode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisSelectionMode {
    Default = 0,
    Vertex = 1,
    Edge = 2,
    Wire = 3,
    Face = 4,
    Shell = 5,
    Solid = 6,
}

// occt: AIS_TypeOfIso
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisTypeOfIso {
    U,
    V,
    Both,
}

// occt: AIS_InteractiveObject
pub struct AisInteractiveObject {
    display_mode: AisDisplayMode,
    is_displayed: bool,
    transparency: f64,
}

impl AisInteractiveObject {
    pub fn new() -> Self {
        AisInteractiveObject {
            display_mode: AisDisplayMode::Shaded,
            is_displayed: false,
            transparency: 0.0,
        }
    }

    pub fn display_mode(&self) -> AisDisplayMode {
        self.display_mode
    }

    pub fn set_display_mode(&mut self, mode: AisDisplayMode) {
        self.display_mode = mode;
    }

    pub fn is_displayed(&self) -> bool {
        self.is_displayed
    }

    pub fn set_displayed(&mut self, v: bool) {
        self.is_displayed = v;
    }

    pub fn transparency(&self) -> f64 {
        self.transparency
    }

    pub fn set_transparency(&mut self, t: f64) {
        self.transparency = t;
    }
}

impl Default for AisInteractiveObject {
    fn default() -> Self {
        Self::new()
    }
}

// occt: AIS_Shape
pub struct AisShape {
    base: AisInteractiveObject,
    shape_type: String,
}

impl AisShape {
    pub fn new(shape_type_name: &str) -> Self {
        AisShape {
            base: AisInteractiveObject::new(),
            shape_type: shape_type_name.to_string(),
        }
    }

    pub fn shape_type(&self) -> &str {
        &self.shape_type
    }

    pub fn display_mode(&self) -> AisDisplayMode {
        self.base.display_mode()
    }

    pub fn set_display_mode(&mut self, mode: AisDisplayMode) {
        self.base.set_display_mode(mode);
    }

    pub fn is_displayed(&self) -> bool {
        self.base.is_displayed()
    }

    pub fn set_displayed(&mut self, v: bool) {
        self.base.set_displayed(v);
    }

    pub fn transparency(&self) -> f64 {
        self.base.transparency()
    }

    pub fn set_transparency(&mut self, t: f64) {
        self.base.set_transparency(t);
    }
}

// occt: AIS_InteractiveContext
pub struct AisInteractiveContext {
    objects: Vec<AisInteractiveObject>,
    selection_mode: AisSelectionMode,
}

impl AisInteractiveContext {
    pub fn new() -> Self {
        AisInteractiveContext {
            objects: Vec::new(),
            selection_mode: AisSelectionMode::Default,
        }
    }

    pub fn display(&mut self, mut obj: AisInteractiveObject) {
        obj.set_displayed(true);
        self.objects.push(obj);
    }

    pub fn erase_all(&mut self) {
        self.objects.clear();
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn selection_mode(&self) -> AisSelectionMode {
        self.selection_mode
    }

    pub fn set_selection_mode(&mut self, mode: AisSelectionMode) {
        self.selection_mode = mode;
    }
}

impl Default for AisInteractiveContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_mode_variants_eq() {
        assert_eq!(AisDisplayMode::Wireframe, AisDisplayMode::Wireframe);
        assert_ne!(AisDisplayMode::Wireframe, AisDisplayMode::Shaded);
    }

    #[test]
    fn test_display_mode_discriminants() {
        assert_eq!(AisDisplayMode::Wireframe as i32, 0);
        assert_eq!(AisDisplayMode::Shaded as i32, 1);
    }

    #[test]
    fn test_selection_mode_discriminants() {
        assert_eq!(AisSelectionMode::Default as i32, 0);
        assert_eq!(AisSelectionMode::Solid as i32, 6);
    }

    #[test]
    fn test_type_of_iso_variants_distinct() {
        assert_ne!(AisTypeOfIso::U, AisTypeOfIso::V);
        assert_ne!(AisTypeOfIso::U, AisTypeOfIso::Both);
        assert_ne!(AisTypeOfIso::V, AisTypeOfIso::Both);
    }

    #[test]
    fn test_interactive_object_defaults() {
        let obj = AisInteractiveObject::new();
        assert_eq!(obj.display_mode(), AisDisplayMode::Shaded);
        assert!(!obj.is_displayed());
        assert_eq!(obj.transparency(), 0.0);
    }

    #[test]
    fn test_interactive_object_set_display_mode() {
        let mut obj = AisInteractiveObject::new();
        obj.set_display_mode(AisDisplayMode::Wireframe);
        assert_eq!(obj.display_mode(), AisDisplayMode::Wireframe);
    }

    #[test]
    fn test_interactive_object_toggle_displayed() {
        let mut obj = AisInteractiveObject::new();
        assert!(!obj.is_displayed());
        obj.set_displayed(true);
        assert!(obj.is_displayed());
        obj.set_displayed(false);
        assert!(!obj.is_displayed());
    }

    #[test]
    fn test_interactive_object_transparency() {
        let mut obj = AisInteractiveObject::new();
        obj.set_transparency(0.5);
        assert_eq!(obj.transparency(), 0.5);
        obj.set_transparency(1.0);
        assert_eq!(obj.transparency(), 1.0);
    }

    #[test]
    fn test_ais_shape_new() {
        let s = AisShape::new("Solid");
        assert_eq!(s.shape_type(), "Solid");
        assert!(!s.is_displayed());
        assert_eq!(s.display_mode(), AisDisplayMode::Shaded);
        assert_eq!(s.transparency(), 0.0);
    }

    #[test]
    fn test_ais_shape_delegates_to_base() {
        let mut s = AisShape::new("Face");
        s.set_display_mode(AisDisplayMode::Wireframe);
        s.set_displayed(true);
        s.set_transparency(0.3);
        assert_eq!(s.display_mode(), AisDisplayMode::Wireframe);
        assert!(s.is_displayed());
        assert_eq!(s.transparency(), 0.3);
    }

    #[test]
    fn test_context_starts_empty() {
        let ctx = AisInteractiveContext::new();
        assert_eq!(ctx.object_count(), 0);
        assert_eq!(ctx.selection_mode(), AisSelectionMode::Default);
    }

    #[test]
    fn test_context_display_increments_count() {
        let mut ctx = AisInteractiveContext::new();
        ctx.display(AisInteractiveObject::new());
        assert_eq!(ctx.object_count(), 1);
        ctx.display(AisInteractiveObject::new());
        assert_eq!(ctx.object_count(), 2);
    }

    #[test]
    fn test_context_erase_all() {
        let mut ctx = AisInteractiveContext::new();
        ctx.display(AisInteractiveObject::new());
        ctx.display(AisInteractiveObject::new());
        ctx.erase_all();
        assert_eq!(ctx.object_count(), 0);
    }

    #[test]
    fn test_context_selection_mode_change() {
        let mut ctx = AisInteractiveContext::new();
        ctx.set_selection_mode(AisSelectionMode::Face);
        assert_eq!(ctx.selection_mode(), AisSelectionMode::Face);
        ctx.set_selection_mode(AisSelectionMode::Edge);
        assert_eq!(ctx.selection_mode(), AisSelectionMode::Edge);
    }

    #[test]
    fn test_context_display_marks_object_displayed() {
        let mut ctx = AisInteractiveContext::new();
        let obj = AisInteractiveObject::new();
        assert!(!obj.is_displayed());
        ctx.display(obj);
        assert_eq!(ctx.object_count(), 1);
    }
}
