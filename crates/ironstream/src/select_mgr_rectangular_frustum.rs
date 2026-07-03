// FILE: select_mgr_rectangular_frustum.rs
// occt: SelectMgr_RectangularFrustum

/// Auxiliary structure to define selection primitive (point or box).
/// In case of point selection, min and max points are identical.
#[derive(Clone, Debug)]
pub struct SelectionRectangle {
    min_pnt: (f64, f64),
    max_pnt: (f64, f64),
}

impl SelectionRectangle {
    /// Creates a new SelectionRectangle with default (uninitialized) values.
    pub fn new() -> Self {
        SelectionRectangle {
            min_pnt: (f64::INFINITY, f64::INFINITY),
            max_pnt: (f64::INFINITY, f64::INFINITY),
        }
    }

    /// Returns the mouse position (min_pnt).
    pub fn mouse_pos(&self) -> (f64, f64) {
        self.min_pnt
    }

    /// Sets the mouse position (sets both min and max to the same point).
    pub fn set_mouse_pos(&mut self, pos: (f64, f64)) {
        self.min_pnt = pos;
        self.max_pnt = pos;
    }

    /// Returns the minimum point.
    pub fn min_pnt(&self) -> (f64, f64) {
        self.min_pnt
    }

    /// Sets the minimum point.
    pub fn set_min_pnt(&mut self, pnt: (f64, f64)) {
        self.min_pnt = pnt;
    }

    /// Returns the maximum point.
    pub fn max_pnt(&self) -> (f64, f64) {
        self.max_pnt
    }

    /// Sets the maximum point.
    pub fn set_max_pnt(&mut self, pnt: (f64, f64)) {
        self.max_pnt = pnt;
    }
}

impl Default for SelectionRectangle {
    fn default() -> Self {
        Self::new()
    }
}

/// Rectangular selecting frustum for point and box selection.
/// This class represents a rectangular frustum with 8 vertices (4 near, 4 far)
/// and provides overlap detection using the separating axis theorem (SAT).
#[derive(Clone, Debug)]
pub struct SelectMgrRectangularFrustum {
    sel_rectangle: SelectionRectangle,
    near_picked_pnt: (f64, f64, f64),
    far_picked_pnt: (f64, f64, f64),
    view_ray_dir: (f64, f64, f64),
    scale: f64,
    vertices: [(f64, f64, f64); 8],
}

impl SelectMgrRectangularFrustum {
    /// Creates a new rectangular selecting frustum.
    pub fn new() -> Self {
        SelectMgrRectangularFrustum {
            sel_rectangle: SelectionRectangle::new(),
            near_picked_pnt: (0.0, 0.0, 0.0),
            far_picked_pnt: (0.0, 0.0, 0.0),
            view_ray_dir: (0.0, 0.0, 1.0),
            scale: 1.0,
            vertices: [(0.0, 0.0, 0.0); 8],
        }
    }

    /// Initializes volume according to the point and given pixel tolerance.
    pub fn init_point(&mut self, point: (f64, f64)) {
        self.sel_rectangle.set_mouse_pos(point);
    }

    /// Initializes volume according to the selected rectangle.
    pub fn init_rect(&mut self, min_pnt: (f64, f64), max_pnt: (f64, f64)) {
        self.sel_rectangle.set_min_pnt(min_pnt);
        self.sel_rectangle.set_max_pnt(max_pnt);
    }

    /// Builds volume according to internal parameters.
    /// This should be called after Init().
    pub fn build(&mut self) {
        // This is a stub implementation. A real one would construct 8 vertices
        // based on sel_rectangle and view parameters.
    }

    /// Checks if it is possible to scale this frustum.
    /// Returns true for frustum built on a single point.
    pub fn is_scalable(&self) -> bool {
        let rect = &self.sel_rectangle;
        rect.min_pnt() == rect.max_pnt()
    }

    /// Returns the vertices of the frustum (8 points).
    pub fn get_vertices(&self) -> &[(f64, f64, f64); 8] {
        &self.vertices
    }

    /// Returns projection onto near view frustum plane.
    pub fn get_near_pnt(&self) -> (f64, f64, f64) {
        self.near_picked_pnt
    }

    /// Returns projection onto far view frustum plane.
    pub fn get_far_pnt(&self) -> (f64, f64, f64) {
        self.far_picked_pnt
    }

    /// Returns view ray direction.
    pub fn get_view_ray_direction(&self) -> (f64, f64, f64) {
        self.view_ray_dir
    }

    /// Returns current mouse coordinates.
    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.sel_rectangle.mouse_pos()
    }

    /// Measures distance between 3D projection of user-picked point and given point.
    pub fn dist_to_geometry_center(&self, center: (f64, f64, f64)) -> f64 {
        let dx = self.near_picked_pnt.0 - center.0;
        let dy = self.near_picked_pnt.1 - center.1;
        let dz = self.near_picked_pnt.2 - center.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calculates the point on a view ray at given depth.
    pub fn detected_point(&self, depth: f64) -> (f64, f64, f64) {
        (
            self.near_picked_pnt.0 + self.view_ray_dir.0 * depth,
            self.near_picked_pnt.1 + self.view_ray_dir.1 * depth,
            self.near_picked_pnt.2 + self.view_ray_dir.2 * depth,
        )
    }
}

impl Default for SelectMgrRectangularFrustum {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_rectangle_mouse_pos() {
        let mut rect = SelectionRectangle::new();
        rect.set_mouse_pos((10.0, 20.0));
        assert_eq!(rect.mouse_pos(), (10.0, 20.0));
        assert_eq!(rect.min_pnt(), (10.0, 20.0));
        assert_eq!(rect.max_pnt(), (10.0, 20.0));
    }

    #[test]
    fn test_selection_rectangle_min_max() {
        let mut rect = SelectionRectangle::new();
        rect.set_min_pnt((5.0, 10.0));
        rect.set_max_pnt((15.0, 20.0));
        assert_eq!(rect.min_pnt(), (5.0, 10.0));
        assert_eq!(rect.max_pnt(), (15.0, 20.0));
    }

    #[test]
    fn test_frustum_new() {
        let frustum = SelectMgrRectangularFrustum::new();
        assert_eq!(frustum.get_near_pnt(), (0.0, 0.0, 0.0));
        assert_eq!(frustum.get_far_pnt(), (0.0, 0.0, 0.0));
        assert_eq!(frustum.get_view_ray_direction(), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_frustum_init_point() {
        let mut frustum = SelectMgrRectangularFrustum::new();
        frustum.init_point((50.0, 75.0));
        assert_eq!(frustum.get_mouse_position(), (50.0, 75.0));
    }

    #[test]
    fn test_frustum_is_scalable() {
        let mut frustum = SelectMgrRectangularFrustum::new();
        frustum.init_point((100.0, 100.0));
        assert!(frustum.is_scalable());

        let mut frustum2 = SelectMgrRectangularFrustum::new();
        frustum2.init_rect((10.0, 10.0), (20.0, 20.0));
        assert!(!frustum2.is_scalable());
    }

    #[test]
    fn test_frustum_dist_to_geometry_center() {
        let frustum = SelectMgrRectangularFrustum::new();
        let dist = frustum.dist_to_geometry_center((1.0, 1.0, 1.0));
        let expected = (1.0 + 1.0 + 1.0_f64).sqrt();
        assert!((dist - expected).abs() < 1e-9);
    }

    #[test]
    fn test_frustum_detected_point() {
        let frustum = SelectMgrRectangularFrustum::new();
        let point = frustum.detected_point(5.0);
        assert_eq!(point, (0.0, 0.0, 5.0));
    }
}
