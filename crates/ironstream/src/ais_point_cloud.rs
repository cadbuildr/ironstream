// FILE: ais_point_cloud.rs
// occt: AIS_PointCloud

use std::rc::Rc;

/// Display modes supported by this Point Cloud object (AIS_PointCloud::DisplayMode).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisPointCloudDisplayMode {
    #[default]
    Points = 0,
    BndBox = 2,
}

/// Selection modes supported by this Point Cloud object (AIS_PointCloud::SelectionMode).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisPointCloudSelectionMode {
    #[default]
    Points = 0,
    SubsetOfPoints = 1,
    BndBox = 2,
}

// ---- local plumbing (gp_Pnt / Quantity_Color / Bnd_Box / Graphic3d_ArrayOfPoints) ----

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/// Minimal Bnd_Box: void until a point is added.
#[derive(Clone, Copy, Debug)]
pub struct BndBox {
    is_void: bool,
    pub xmin: f64,
    pub ymin: f64,
    pub zmin: f64,
    pub xmax: f64,
    pub ymax: f64,
    pub zmax: f64,
}

impl BndBox {
    pub fn new() -> Self {
        BndBox {
            is_void: true,
            xmin: 0.0,
            ymin: 0.0,
            zmin: 0.0,
            xmax: 0.0,
            ymax: 0.0,
            zmax: 0.0,
        }
    }
    pub fn is_void(&self) -> bool {
        self.is_void
    }
    pub fn add(&mut self, p: &Pnt) {
        if self.is_void {
            self.is_void = false;
            self.xmin = p.x;
            self.xmax = p.x;
            self.ymin = p.y;
            self.ymax = p.y;
            self.zmin = p.z;
            self.zmax = p.z;
        } else {
            self.xmin = self.xmin.min(p.x);
            self.xmax = self.xmax.max(p.x);
            self.ymin = self.ymin.min(p.y);
            self.ymax = self.ymax.max(p.y);
            self.zmin = self.zmin.min(p.z);
            self.zmax = self.zmax.max(p.z);
        }
    }
}

/// Minimal Graphic3d_ArrayOfPoints: vertices with optional per-vertex colors.
#[derive(Clone, Debug)]
pub struct ArrayOfPoints {
    verts: Vec<Pnt>,
    colors: Vec<Option<Color>>,
}

impl ArrayOfPoints {
    pub fn new() -> Self {
        ArrayOfPoints {
            verts: Vec::new(),
            colors: Vec::new(),
        }
    }
    pub fn add_vertex(&mut self, p: Pnt) {
        self.verts.push(p);
        self.colors.push(None);
    }
    pub fn add_vertex_with_color(&mut self, p: Pnt, c: Color) {
        self.verts.push(p);
        self.colors.push(Some(c));
    }
    pub fn vertex_number(&self) -> usize {
        self.verts.len()
    }
    pub fn vertice(&self, index1: usize) -> Pnt {
        self.verts[index1 - 1]
    }
    pub fn vertex_color(&self, index1: usize) -> Option<Color> {
        self.colors[index1 - 1]
    }
    pub fn has_vertex_colors(&self) -> bool {
        self.colors.iter().any(|c| c.is_some())
    }
}

/// Interactive object for a set of points (AIS_PointCloud).
#[derive(Clone, Debug)]
pub struct AisPointCloud {
    points: Option<Rc<ArrayOfPoints>>,
    bnd_box: BndBox,
    custom_color: Option<Color>,
}

impl AisPointCloud {
    pub fn new() -> Self {
        AisPointCloud {
            points: None,
            bnd_box: BndBox::new(),
            custom_color: None,
        }
    }

    /// SetPoints(thePoints): array stored as handle (not copied); bounding box recomputed.
    pub fn set_points(&mut self, points: Rc<ArrayOfPoints>) {
        let mut bb = BndBox::new();
        for i in 1..=points.vertex_number() {
            bb.add(&points.vertice(i));
        }
        self.bnd_box = bb;
        self.points = Some(points);
    }

    /// SetPoints(theCoords, theColors): copies input into internal buffer.
    /// If colors are provided with mismatching length the presentation is not computed
    /// (points are reset), matching OCCT behavior of rejecting inconsistent input.
    pub fn set_points_with_colors(&mut self, coords: &[Pnt], colors: Option<&[Color]>) {
        if let Some(cols) = colors {
            if cols.len() != coords.len() {
                self.points = None;
                self.bnd_box = BndBox::new();
                return;
            }
        }
        let mut arr = ArrayOfPoints::new();
        for (i, p) in coords.iter().enumerate() {
            match colors {
                Some(cols) => arr.add_vertex_with_color(*p, cols[i]),
                None => arr.add_vertex(*p),
            }
        }
        self.set_points(Rc::new(arr));
    }

    /// GetPoints(): the stored points array, if any.
    pub fn get_points(&self) -> Option<Rc<ArrayOfPoints>> {
        self.points.clone()
    }

    /// GetBoundingBox(): bounding box for presentation.
    pub fn get_bounding_box(&self) -> BndBox {
        self.bnd_box
    }

    /// SetColor: custom color, affects presentation only when no per-point colors.
    pub fn set_color(&mut self, color: Color) {
        self.custom_color = Some(color);
    }

    /// UnsetColor: restore default color.
    pub fn unset_color(&mut self) {
        self.custom_color = None;
    }

    pub fn has_color(&self) -> bool {
        self.custom_color.is_some()
    }

    pub fn color(&self) -> Option<Color> {
        self.custom_color
    }
}

impl Default for AisPointCloud {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_selection_modes() {
        assert_eq!(AisPointCloudDisplayMode::Points as u32, 0);
        assert_eq!(AisPointCloudDisplayMode::BndBox as u32, 2);
        assert_eq!(AisPointCloudSelectionMode::Points as u32, 0);
        assert_eq!(AisPointCloudSelectionMode::SubsetOfPoints as u32, 1);
        assert_eq!(AisPointCloudSelectionMode::BndBox as u32, 2);
        assert_eq!(
            AisPointCloudDisplayMode::default(),
            AisPointCloudDisplayMode::Points
        );
    }

    #[test]
    fn test_new_empty() {
        let pc = AisPointCloud::new();
        assert!(pc.get_points().is_none());
        assert!(pc.get_bounding_box().is_void());
        assert!(!pc.has_color());
    }

    #[test]
    fn test_set_points_and_bnd_box() {
        let mut pc = AisPointCloud::new();
        let coords = [
            Pnt { x: 1.0, y: 2.0, z: 3.0 },
            Pnt { x: -1.0, y: 5.0, z: 0.5 },
            Pnt { x: 4.0, y: -2.0, z: 3.0 },
        ];
        pc.set_points_with_colors(&coords, None);
        let pts = pc.get_points().expect("points set");
        assert_eq!(pts.vertex_number(), 3);
        assert_eq!(pts.vertice(2), coords[1]);
        assert!(!pts.has_vertex_colors());
        let bb = pc.get_bounding_box();
        assert!(!bb.is_void());
        assert_eq!(bb.xmin, -1.0);
        assert_eq!(bb.xmax, 4.0);
        assert_eq!(bb.ymin, -2.0);
        assert_eq!(bb.ymax, 5.0);
        assert_eq!(bb.zmin, 0.5);
        assert_eq!(bb.zmax, 3.0);
    }

    #[test]
    fn test_set_points_with_colors() {
        let mut pc = AisPointCloud::new();
        let coords = [
            Pnt { x: 0.0, y: 0.0, z: 0.0 },
            Pnt { x: 1.0, y: 1.0, z: 1.0 },
        ];
        let cols = [
            Color { r: 1.0, g: 0.0, b: 0.0 },
            Color { r: 0.0, g: 1.0, b: 0.0 },
        ];
        pc.set_points_with_colors(&coords, Some(&cols));
        let pts = pc.get_points().unwrap();
        assert!(pts.has_vertex_colors());
        assert_eq!(pts.vertex_color(1), Some(cols[0]));
        assert_eq!(pts.vertex_color(2), Some(cols[1]));
    }

    #[test]
    fn test_mismatched_colors_rejected() {
        let mut pc = AisPointCloud::new();
        let coords = [
            Pnt { x: 0.0, y: 0.0, z: 0.0 },
            Pnt { x: 1.0, y: 1.0, z: 1.0 },
        ];
        let cols = [Color { r: 1.0, g: 0.0, b: 0.0 }];
        pc.set_points_with_colors(&coords, Some(&cols));
        assert!(pc.get_points().is_none());
        assert!(pc.get_bounding_box().is_void());
    }

    #[test]
    fn test_set_unset_color() {
        let mut pc = AisPointCloud::new();
        let c = Color { r: 0.2, g: 0.4, b: 0.6 };
        pc.set_color(c);
        assert!(pc.has_color());
        assert_eq!(pc.color(), Some(c));
        pc.unset_color();
        assert!(!pc.has_color());
        assert_eq!(pc.color(), None);
    }
}
