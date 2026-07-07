// FILE: top_ope_b_rep_ds_data_structure.rs
// occt: TopOpeBRepDS_DataStructure

use std::collections::HashMap;

/// Point data structure
#[derive(Clone, Debug)]
pub struct TopOpeBRepDSPoint {
    x: f64,
    y: f64,
    z: f64,
}

impl TopOpeBRepDSPoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        TopOpeBRepDSPoint { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

/// Curve data structure
#[derive(Clone, Debug)]
pub struct TopOpeBRepDSCurve {
    id: i32,
    keep: bool,
}

impl TopOpeBRepDSCurve {
    pub fn new(id: i32) -> Self {
        TopOpeBRepDSCurve { id, keep: true }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn is_keep(&self) -> bool {
        self.keep
    }

    pub fn set_keep(&mut self, keep: bool) {
        self.keep = keep;
    }
}

/// Surface data structure
#[derive(Clone, Debug)]
pub struct TopOpeBRepDSSurface {
    id: i32,
    keep: bool,
}

impl TopOpeBRepDSSurface {
    pub fn new(id: i32) -> Self {
        TopOpeBRepDSSurface { id, keep: true }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn is_keep(&self) -> bool {
        self.keep
    }

    pub fn set_keep(&mut self, keep: bool) {
        self.keep = keep;
    }
}

/// Main data structure for storing geometries and topologies with their interferences.
/// The DataStructure stores:
/// - New geometries: points, curves, and surfaces.
/// - Topological shapes: vertices, edges, faces.
/// - Interferences between geometries and shapes.
pub struct TopOpeBRepDSDataStructure {
    surfaces: HashMap<i32, TopOpeBRepDSSurface>,
    curves: HashMap<i32, TopOpeBRepDSCurve>,
    points: HashMap<i32, TopOpeBRepDSPoint>,
    next_surface_id: i32,
    next_curve_id: i32,
    next_point_id: i32,
}

impl TopOpeBRepDSDataStructure {
    /// Create a new data structure
    pub fn new() -> Self {
        TopOpeBRepDSDataStructure {
            surfaces: HashMap::new(),
            curves: HashMap::new(),
            points: HashMap::new(),
            next_surface_id: 1,
            next_curve_id: 1,
            next_point_id: 1,
        }
    }

    /// Reset the data structure
    pub fn init(&mut self) {
        self.surfaces.clear();
        self.curves.clear();
        self.points.clear();
        self.next_surface_id = 1;
        self.next_curve_id = 1;
        self.next_point_id = 1;
    }

    /// Insert a new surface. Returns the index.
    pub fn add_surface(&mut self, surface: TopOpeBRepDSSurface) -> i32 {
        let id = self.next_surface_id;
        let mut s = surface;
        s.id = id;
        self.surfaces.insert(id, s);
        self.next_surface_id += 1;
        id
    }

    /// Remove a surface by index
    pub fn remove_surface(&mut self, index: i32) {
        self.surfaces.remove(&index);
    }

    /// Check if surface should be kept
    pub fn keep_surface(&self, index: i32) -> bool {
        self.surfaces
            .get(&index)
            .map(|s| s.is_keep())
            .unwrap_or(false)
    }

    /// Change keep flag for surface
    pub fn change_keep_surface(&mut self, index: i32, keep: bool) {
        if let Some(surface) = self.surfaces.get_mut(&index) {
            surface.set_keep(keep);
        }
    }

    /// Insert a new curve. Returns the index.
    pub fn add_curve(&mut self, curve: TopOpeBRepDSCurve) -> i32 {
        let id = self.next_curve_id;
        let mut c = curve;
        c.id = id;
        self.curves.insert(id, c);
        self.next_curve_id += 1;
        id
    }

    /// Remove a curve by index
    pub fn remove_curve(&mut self, index: i32) {
        self.curves.remove(&index);
    }

    /// Check if curve should be kept
    pub fn keep_curve(&self, index: i32) -> bool {
        self.curves
            .get(&index)
            .map(|c| c.is_keep())
            .unwrap_or(false)
    }

    /// Change keep flag for curve
    pub fn change_keep_curve(&mut self, index: i32, keep: bool) {
        if let Some(curve) = self.curves.get_mut(&index) {
            curve.set_keep(keep);
        }
    }

    /// Insert a new point. Returns the index.
    pub fn add_point(&mut self, point: TopOpeBRepDSPoint) -> i32 {
        let id = self.next_point_id;
        self.points.insert(id, point);
        self.next_point_id += 1;
        id
    }

    /// Remove a point by index
    pub fn remove_point(&mut self, index: i32) {
        self.points.remove(&index);
    }

    /// Check if point should be kept
    pub fn keep_point(&self, index: i32) -> bool {
        self.points.contains_key(&index)
    }

    /// Get a surface by index
    pub fn surface(&self, index: i32) -> Option<&TopOpeBRepDSSurface> {
        self.surfaces.get(&index)
    }

    /// Get a curve by index
    pub fn curve(&self, index: i32) -> Option<&TopOpeBRepDSCurve> {
        self.curves.get(&index)
    }

    /// Get a point by index
    pub fn point(&self, index: i32) -> Option<&TopOpeBRepDSPoint> {
        self.points.get(&index)
    }

    /// Get number of surfaces
    pub fn nb_surfaces(&self) -> i32 {
        self.surfaces.len() as i32
    }

    /// Get number of curves
    pub fn nb_curves(&self) -> i32 {
        self.curves.len() as i32
    }

    /// Get number of points
    pub fn nb_points(&self) -> i32 {
        self.points.len() as i32
    }
}

impl Default for TopOpeBRepDSDataStructure {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_data_structure() {
        let ds = TopOpeBRepDSDataStructure::new();
        assert_eq!(ds.nb_surfaces(), 0);
        assert_eq!(ds.nb_curves(), 0);
        assert_eq!(ds.nb_points(), 0);
    }

    #[test]
    fn test_add_surface() {
        let mut ds = TopOpeBRepDSDataStructure::new();
        let surface = TopOpeBRepDSSurface::new(0);
        let id = ds.add_surface(surface);
        assert_eq!(id, 1);
        assert_eq!(ds.nb_surfaces(), 1);
    }

    #[test]
    fn test_add_curve() {
        let mut ds = TopOpeBRepDSDataStructure::new();
        let curve = TopOpeBRepDSCurve::new(0);
        let id = ds.add_curve(curve);
        assert_eq!(id, 1);
        assert_eq!(ds.nb_curves(), 1);
    }

    #[test]
    fn test_add_point() {
        let mut ds = TopOpeBRepDSDataStructure::new();
        let point = TopOpeBRepDSPoint::new(1.0, 2.0, 3.0);
        let id = ds.add_point(point);
        assert_eq!(id, 1);
        assert_eq!(ds.nb_points(), 1);
    }

    #[test]
    fn test_remove_surface() {
        let mut ds = TopOpeBRepDSDataStructure::new();
        let surface = TopOpeBRepDSSurface::new(0);
        let id = ds.add_surface(surface);
        ds.remove_surface(id);
        assert_eq!(ds.nb_surfaces(), 0);
    }

    #[test]
    fn test_keep_surface() {
        let mut ds = TopOpeBRepDSDataStructure::new();
        let surface = TopOpeBRepDSSurface::new(0);
        let id = ds.add_surface(surface);
        assert!(ds.keep_surface(id));
        ds.change_keep_surface(id, false);
        assert!(!ds.keep_surface(id));
    }

    #[test]
    fn test_keep_curve() {
        let mut ds = TopOpeBRepDSDataStructure::new();
        let curve = TopOpeBRepDSCurve::new(0);
        let id = ds.add_curve(curve);
        assert!(ds.keep_curve(id));
    }

    #[test]
    fn test_init_clears_all() {
        let mut ds = TopOpeBRepDSDataStructure::new();
        ds.add_surface(TopOpeBRepDSSurface::new(0));
        ds.add_curve(TopOpeBRepDSCurve::new(0));
        ds.add_point(TopOpeBRepDSPoint::new(0.0, 0.0, 0.0));
        ds.init();
        assert_eq!(ds.nb_surfaces(), 0);
        assert_eq!(ds.nb_curves(), 0);
        assert_eq!(ds.nb_points(), 0);
    }
}
