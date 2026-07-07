// FILE: graphic3d_c_view.rs
// occt: Graphic3d_CView

/// Base class of a graphical view that carries out rendering process for a concrete
/// implementation of graphical driver. Provides virtual interfaces for redrawing its
/// contents, management of displayed structures and render settings.
pub struct Graphic3dCView {
    /// Identification number of the view
    id: i32,
    /// Activity flag of the view
    is_active: bool,
    /// Whether the view was removed
    is_removed: bool,
    /// Computed HLR mode state
    is_in_computed_mode: bool,
    /// Shading model (default: Phong)
    shading_model: i32,
    /// Backfacing model
    backfacing_model: i32,
    /// Visualization type
    visualization_type: i32,
    /// ZLayerId target
    z_layer_target: i32,
    /// ZLayerId redraw mode
    z_layer_redraw_mode: bool,
    /// Number of displayed structures
    num_displayed_structures: i32,
}

impl Graphic3dCView {
    /// Create a new view with the given ID.
    pub fn new(id: i32) -> Self {
        Self {
            id,
            is_active: false,
            is_removed: false,
            is_in_computed_mode: false,
            shading_model: 0, // Graphic3d_TypeOfShadingModel_Phong (default)
            backfacing_model: 0, // Graphic3d_TypeOfBackfacingModel_Auto
            visualization_type: 0, // Graphic3d_TypeOfVisualization
            z_layer_target: -1, // Unknown
            z_layer_redraw_mode: false,
            num_displayed_structures: 0,
        }
    }

    /// Returns the identification number of the view.
    pub fn identification(&self) -> i32 {
        self.id
    }

    /// Activates the view.
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Deactivates the view.
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Returns the activity flag of the view.
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Marks the view as removed.
    pub fn remove(&mut self) {
        self.is_removed = true;
    }

    /// Returns true if the view was removed.
    pub fn is_removed(&self) -> bool {
        self.is_removed
    }

    /// Returns default Shading Model of the view.
    pub fn shading_model(&self) -> i32 {
        self.shading_model
    }

    /// Sets default Shading Model of the view.
    pub fn set_shading_model(&mut self, model: i32) {
        self.shading_model = model;
    }

    /// Return backfacing model used for the view.
    pub fn backfacing_model(&self) -> i32 {
        self.backfacing_model
    }

    /// Sets backfacing model for the view.
    pub fn set_backfacing_model(&mut self, model: i32) {
        self.backfacing_model = model;
    }

    /// Returns visualization type of the view.
    pub fn visualization_type(&self) -> i32 {
        self.visualization_type
    }

    /// Sets visualization type of the view.
    pub fn set_visualization_type(&mut self, visualization_type: i32) {
        self.visualization_type = visualization_type;
    }

    /// Returns ZLayerId target.
    pub fn z_layer_target(&self) -> i32 {
        self.z_layer_target
    }

    /// Sets ZLayerId target.
    pub fn set_z_layer_target(&mut self, target: i32) {
        self.z_layer_target = target;
    }

    /// Returns ZLayerId redraw mode.
    pub fn z_layer_redraw_mode(&self) -> bool {
        self.z_layer_redraw_mode
    }

    /// Sets ZLayerId redraw mode.
    pub fn set_z_layer_redraw_mode(&mut self, mode: bool) {
        self.z_layer_redraw_mode = mode;
    }

    /// Switches computed HLR mode in the view.
    pub fn set_computed_mode(&mut self, mode: bool) {
        self.is_in_computed_mode = mode;
    }

    /// Returns the computed HLR mode state.
    pub fn computed_mode(&self) -> bool {
        self.is_in_computed_mode
    }

    /// Returns number of displayed structures in the view.
    pub fn number_of_displayed_structures(&self) -> i32 {
        self.num_displayed_structures
    }

    /// Set the number of displayed structures.
    pub fn set_number_of_displayed_structures(&mut self, count: i32) {
        self.num_displayed_structures = count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_view() {
        let v = Graphic3dCView::new(10);
        assert_eq!(v.identification(), 10);
        assert!(!v.is_active());
        assert!(!v.is_removed());
        assert!(!v.computed_mode());
    }

    #[test]
    fn test_activation() {
        let mut v = Graphic3dCView::new(10);
        assert!(!v.is_active());
        v.activate();
        assert!(v.is_active());
        v.deactivate();
        assert!(!v.is_active());
    }

    #[test]
    fn test_remove() {
        let mut v = Graphic3dCView::new(10);
        assert!(!v.is_removed());
        v.remove();
        assert!(v.is_removed());
    }

    #[test]
    fn test_shading_model() {
        let mut v = Graphic3dCView::new(10);
        let original = v.shading_model();
        v.set_shading_model(1);
        assert_eq!(v.shading_model(), 1);
        assert_ne!(v.shading_model(), original);
    }

    #[test]
    fn test_backfacing_model() {
        let mut v = Graphic3dCView::new(10);
        let original = v.backfacing_model();
        v.set_backfacing_model(2);
        assert_eq!(v.backfacing_model(), 2);
        assert_ne!(v.backfacing_model(), original);
    }

    #[test]
    fn test_computed_mode() {
        let mut v = Graphic3dCView::new(10);
        assert!(!v.computed_mode());
        v.set_computed_mode(true);
        assert!(v.computed_mode());
    }

    #[test]
    fn test_z_layer_target() {
        let mut v = Graphic3dCView::new(10);
        assert_eq!(v.z_layer_target(), -1);
        v.set_z_layer_target(5);
        assert_eq!(v.z_layer_target(), 5);
    }

    #[test]
    fn test_z_layer_redraw_mode() {
        let mut v = Graphic3dCView::new(10);
        assert!(!v.z_layer_redraw_mode());
        v.set_z_layer_redraw_mode(true);
        assert!(v.z_layer_redraw_mode());
    }

    #[test]
    fn test_number_of_displayed_structures() {
        let mut v = Graphic3dCView::new(10);
        assert_eq!(v.number_of_displayed_structures(), 0);
        v.set_number_of_displayed_structures(5);
        assert_eq!(v.number_of_displayed_structures(), 5);
    }
}
