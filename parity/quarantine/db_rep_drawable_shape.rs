// FILE: db_rep_drawable_shape.rs
// occt: DBRep_DrawableShape

//! Drawable structure to display a shape.
//! Contains lists of edges and faces for visualization.

use std::collections::HashMap;

/// A color representation (RGB).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl DrawColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        DrawColor { r, g, b }
    }

    pub fn white() -> Self {
        DrawColor {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn black() -> Self {
        DrawColor { r: 0, g: 0, b: 0 }
    }

    pub fn red() -> Self {
        DrawColor {
            r: 255,
            g: 0,
            b: 0,
        }
    }
}

/// A 3D point (for normals).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpPnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GpPnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GpPnt { x, y, z }
    }
}

/// A pair of points representing a normal vector.
pub type NormalPair = (GpPnt, GpPnt);

/// Placeholder for shape representation.
#[derive(Clone, Debug)]
pub struct TopodsShape {
    id: u32,
}

impl TopodsShape {
    pub fn new(id: u32) -> Self {
        TopodsShape { id }
    }
}

/// Placeholder for face representation.
#[derive(Clone, Debug)]
pub struct TopodsF ace {
    id: u32,
}

impl TopodsF ace {
    pub fn new(id: u32) -> Self {
        TopodsF ace { id }
    }
}

/// Placeholder for Draw display.
#[derive(Clone, Debug)]
pub struct DrawDisplay;

impl DrawDisplay {
    pub fn new() -> Self {
        DrawDisplay
    }
}

impl Default for DrawDisplay {
    fn default() -> Self {
        Self::new()
    }
}

/// DBRep_DrawableShape: a drawable representation of a shape.
#[derive(Clone, Debug)]
pub struct DbrepDrawableShape {
    shape: TopodsShape,
    free_color: DrawColor,
    conn_color: DrawColor,
    edge_color: DrawColor,
    isos_color: DrawColor,
    size: f64,
    nb_isos: i32,
    discret: i32,
    display_orientation: bool,
    display_triangulation: bool,
    display_polygons: bool,
    hlr_enabled: bool,
    rg1_enabled: bool,
    rgn_enabled: bool,
    hid_enabled: bool,
    hlr_angle: f64,
    last_pick_shape: Option<TopodsShape>,
    last_pick_u: f64,
    last_pick_v: f64,
}

impl DbrepDrawableShape {
    /// Create a new drawable shape with initial parameters.
    pub fn new(
        shape: TopodsShape,
        free_col: DrawColor,
        conn_col: DrawColor,
        edge_col: DrawColor,
        isos_col: DrawColor,
        size: f64,
        nb_isos: i32,
        discret: i32,
    ) -> Self {
        DbrepDrawableShape {
            shape,
            free_color: free_col,
            conn_color: conn_col,
            edge_color: edge_col,
            isos_color: isos_col,
            size,
            nb_isos,
            discret,
            display_orientation: false,
            display_triangulation: false,
            display_polygons: false,
            hlr_enabled: false,
            rg1_enabled: false,
            rgn_enabled: false,
            hid_enabled: false,
            hlr_angle: 0.57,
            last_pick_shape: None,
            last_pick_u: 0.0,
            last_pick_v: 0.0,
        }
    }

    /// Change the number of isoparametric curves.
    pub fn change_nb_isos(&mut self, nb: i32) {
        self.nb_isos = nb;
    }

    /// Get the number of isoparametric curves.
    pub fn nb_isos(&self) -> i32 {
        self.nb_isos
    }

    /// Change the discretisation value.
    pub fn change_discret(&mut self, discret: i32) {
        self.discret = discret;
    }

    /// Get the discretisation value.
    pub fn discret(&self) -> i32 {
        self.discret
    }

    /// Get the shape.
    pub fn shape(&self) -> &TopodsShape {
        &self.shape
    }

    /// Enable/disable orientation display.
    pub fn display_orientation(&mut self, display: bool) {
        self.display_orientation = display;
    }

    /// Enable/disable triangulation display.
    pub fn display_triangulation(&mut self, display: bool) {
        self.display_triangulation = display;
    }

    /// Get triangulation display state.
    pub fn triangulation_displayed(&self) -> bool {
        self.display_triangulation
    }

    /// Enable/disable polygon display.
    pub fn display_polygons(&mut self, display: bool) {
        self.display_polygons = display;
    }

    /// Get polygon display state.
    pub fn polygons_displayed(&self) -> bool {
        self.display_polygons
    }

    /// Configure HLR (Hidden Line Removal) display.
    pub fn display_hlr(
        &mut self,
        with_hlr: bool,
        with_rg1: bool,
        with_rgn: bool,
        with_hid: bool,
        ang: f64,
    ) {
        self.hlr_enabled = with_hlr;
        self.rg1_enabled = with_rg1;
        self.rgn_enabled = with_rgn;
        self.hid_enabled = with_hid;
        self.hlr_angle = ang;
    }

    /// Get HLR display configuration.
    pub fn get_display_hlr(&self) -> (bool, bool, bool, bool, f64) {
        (
            self.hlr_enabled,
            self.rg1_enabled,
            self.rgn_enabled,
            self.hid_enabled,
            self.hlr_angle,
        )
    }

    /// Draw the shape on the display.
    pub fn draw_on(&self, _dis: &DrawDisplay) {
        // In real implementation: render shape based on display flags
    }

    /// Display hidden lines.
    pub fn display_hidden_lines(&self, _dis: &DrawDisplay) {
        // In real implementation: perform HLR computation and display
    }

    /// Create a copy of this drawable shape.
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// Dump drawable shape information.
    pub fn dump(&self) -> String {
        format!(
            "DBRep_DrawableShape: NbIsos={}, Discret={}, HLR={}, Triangulation={}",
            self.nb_isos, self.discret, self.hlr_enabled, self.display_triangulation
        )
    }

    /// Get the last picked subshape and its parameters.
    pub fn last_pick() -> Option<(TopodsShape, f64, f64)> {
        // This would access a global last pick state in real implementation
        None
    }

    /// Set last pick information.
    pub fn set_last_pick(&mut self, shape: TopodsShape, u: f64, v: f64) {
        self.last_pick_shape = Some(shape);
        self.last_pick_u = u;
        self.last_pick_v = v;
    }

    /// Add mesh normals to a vector.
    pub fn add_mesh_normals(
        _face: &TopodsF ace,
        _length: f64,
    ) -> Result<Vec<NormalPair>, String> {
        // In real implementation: compute normals from mesh
        Ok(Vec::new())
    }

    /// Add surface normals distributed over a face.
    pub fn add_surface_normals(
        _face: &TopodsF ace,
        _length: f64,
        _nb_along_u: i32,
        _nb_along_v: i32,
    ) -> Result<Vec<NormalPair>, String> {
        // In real implementation: compute surface normals
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawable_shape_creation() {
        let shape = TopodsShape::new(1);
        let drawable = DbrepDrawableShape::new(
            shape,
            DrawColor::white(),
            DrawColor::black(),
            DrawColor::red(),
            DrawColor::new(100, 100, 100),
            1.0,
            10,
            30,
        );

        assert_eq!(drawable.nb_isos(), 10);
        assert_eq!(drawable.discret(), 30);
    }

    #[test]
    fn test_change_nb_isos() {
        let shape = TopodsShape::new(1);
        let mut drawable = DbrepDrawableShape::new(
            shape,
            DrawColor::white(),
            DrawColor::black(),
            DrawColor::red(),
            DrawColor::new(100, 100, 100),
            1.0,
            10,
            30,
        );

        drawable.change_nb_isos(20);
        assert_eq!(drawable.nb_isos(), 20);
    }

    #[test]
    fn test_change_discret() {
        let shape = TopodsShape::new(1);
        let mut drawable = DbrepDrawableShape::new(
            shape,
            DrawColor::white(),
            DrawColor::black(),
            DrawColor::red(),
            DrawColor::new(100, 100, 100),
            1.0,
            10,
            30,
        );

        drawable.change_discret(50);
        assert_eq!(drawable.discret(), 50);
    }

    #[test]
    fn test_display_flags() {
        let shape = TopodsShape::new(1);
        let mut drawable = DbrepDrawableShape::new(
            shape,
            DrawColor::white(),
            DrawColor::black(),
            DrawColor::red(),
            DrawColor::new(100, 100, 100),
            1.0,
            10,
            30,
        );

        assert!(!drawable.triangulation_displayed());
        drawable.display_triangulation(true);
        assert!(drawable.triangulation_displayed());

        assert!(!drawable.polygons_displayed());
        drawable.display_polygons(true);
        assert!(drawable.polygons_displayed());
    }

    #[test]
    fn test_hlr_configuration() {
        let shape = TopodsShape::new(1);
        let mut drawable = DbrepDrawableShape::new(
            shape,
            DrawColor::white(),
            DrawColor::black(),
            DrawColor::red(),
            DrawColor::new(100, 100, 100),
            1.0,
            10,
            30,
        );

        drawable.display_hlr(true, true, false, true, 0.5);
        let (hlr, rg1, rgn, hid, ang) = drawable.get_display_hlr();

        assert!(hlr);
        assert!(rg1);
        assert!(!rgn);
        assert!(hid);
        assert!((ang - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_copy() {
        let shape = TopodsShape::new(1);
        let drawable = DbrepDrawableShape::new(
            shape,
            DrawColor::white(),
            DrawColor::black(),
            DrawColor::red(),
            DrawColor::new(100, 100, 100),
            1.0,
            10,
            30,
        );

        let copied = drawable.copy();
        assert_eq!(copied.nb_isos(), drawable.nb_isos());
        assert_eq!(copied.discret(), drawable.discret());
    }

    #[test]
    fn test_dump() {
        let shape = TopodsShape::new(1);
        let drawable = DbrepDrawableShape::new(
            shape,
            DrawColor::white(),
            DrawColor::black(),
            DrawColor::red(),
            DrawColor::new(100, 100, 100),
            1.0,
            10,
            30,
        );

        let dump = drawable.dump();
        assert!(dump.contains("NbIsos=10"));
        assert!(dump.contains("Discret=30"));
    }

    #[test]
    fn test_last_pick() {
        let shape = TopodsShape::new(1);
        let mut drawable = DbrepDrawableShape::new(
            shape,
            DrawColor::white(),
            DrawColor::black(),
            DrawColor::red(),
            DrawColor::new(100, 100, 100),
            1.0,
            10,
            30,
        );

        let shape_to_pick = TopodsShape::new(42);
        drawable.set_last_pick(shape_to_pick, 1.5, 2.5);

        assert_eq!(drawable.last_pick_u, 1.5);
        assert_eq!(drawable.last_pick_v, 2.5);
    }

    #[test]
    fn test_draw_color() {
        let color = DrawColor::new(128, 64, 32);
        assert_eq!(color.r, 128);
        assert_eq!(color.g, 64);
        assert_eq!(color.b, 32);

        let white = DrawColor::white();
        assert_eq!(white.r, 255);

        let black = DrawColor::black();
        assert_eq!(black.r, 0);
    }
}
