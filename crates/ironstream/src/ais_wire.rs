// FILE: ais_wire.rs
// occt: AIS_ConnectedInteractive, AIS_Wire, AIS_Face, AIS_Plane,
//       AIS_PlaneTrihedron, AIS_EdgeRelation

/// Display mode for AIS interactive objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AisDisplayMode {
    #[default]
    Wireframe,
    Shaded,
    ShadedWithEdges,
}

/// Selection priority for interactive objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SelectionPriority(pub i32);

impl SelectionPriority {
    pub fn default_priority() -> Self { Self(0) }
    pub fn high() -> Self { Self(10) }
}

/// Presentation of a wire (edge loop).
/// occt: AIS_Wire
#[derive(Clone, Debug)]
pub struct AisWire {
    pub wire_id: u32,
    pub edge_ids: Vec<u32>,
    pub color: Option<[f32; 3]>,
    pub line_width: f32,
    pub is_selected: bool,
    pub priority: SelectionPriority,
}

impl AisWire {
    pub fn new(wire_id: u32) -> Self {
        Self {
            wire_id,
            edge_ids: Vec::new(),
            color: None,
            line_width: 1.0,
            is_selected: false,
            priority: SelectionPriority::default_priority(),
        }
    }

    pub fn add_edge(&mut self, edge_id: u32) {
        if !self.edge_ids.contains(&edge_id) {
            self.edge_ids.push(edge_id);
        }
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) { self.color = Some([r, g, b]); }
    pub fn set_line_width(&mut self, w: f32) { self.line_width = w.max(0.0); }
    pub fn select(&mut self) { self.is_selected = true; }
    pub fn deselect(&mut self) { self.is_selected = false; }

    pub fn nb_edges(&self) -> usize { self.edge_ids.len() }
    pub fn is_closed(&self) -> bool { self.nb_edges() >= 3 }
}

/// Presentation of a face.
/// occt: AIS_Face
#[derive(Clone, Debug)]
pub struct AisFace {
    pub face_id: u32,
    pub surface_id: u32,
    pub wire_id: Option<u32>,
    pub display_mode: AisDisplayMode,
    pub color: Option<[f32; 4]>,    // RGBA
    pub transparency: f32,
    pub is_selected: bool,
}

impl AisFace {
    pub fn new(face_id: u32, surface_id: u32) -> Self {
        Self {
            face_id,
            surface_id,
            wire_id: None,
            display_mode: AisDisplayMode::Shaded,
            color: None,
            transparency: 0.0,
            is_selected: false,
        }
    }

    pub fn set_wire(&mut self, wire_id: u32) { self.wire_id = Some(wire_id); }
    pub fn set_display_mode(&mut self, m: AisDisplayMode) { self.display_mode = m; }
    pub fn set_color(&mut self, rgba: [f32; 4]) { self.color = Some(rgba); }
    pub fn set_transparency(&mut self, t: f32) { self.transparency = t.clamp(0.0, 1.0); }
    pub fn select(&mut self) { self.is_selected = true; }
    pub fn deselect(&mut self) { self.is_selected = false; }
}

/// Infinite plane presentation.
/// occt: AIS_Plane
#[derive(Clone, Debug)]
pub struct AisPlane {
    pub plane_id: u32,
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub is_xoy: bool,
    pub is_selected: bool,
    pub size: f64,
}

impl AisPlane {
    pub fn new(plane_id: u32, origin: [f64; 3], normal: [f64; 3]) -> Self {
        Self { plane_id, origin, normal, is_xoy: false, is_selected: false, size: 1.0 }
    }

    pub fn xoy() -> Self { Self::new(1, [0.0; 3], [0.0, 0.0, 1.0]) }
    pub fn xoz() -> Self { Self::new(2, [0.0; 3], [0.0, 1.0, 0.0]) }
    pub fn yoz() -> Self { Self::new(3, [0.0; 3], [1.0, 0.0, 0.0]) }

    pub fn set_size(&mut self, s: f64) { self.size = s.max(0.0); }
    pub fn select(&mut self) { self.is_selected = true; }

    /// Project a point onto the plane.
    pub fn project_point(&self, p: [f64; 3]) -> [f64; 3] {
        let n = self.normal;
        let len2 = n[0]*n[0] + n[1]*n[1] + n[2]*n[2];
        if len2 < 1e-14 { return p; }
        let op = [p[0]-self.origin[0], p[1]-self.origin[1], p[2]-self.origin[2]];
        let dot = op[0]*n[0] + op[1]*n[1] + op[2]*n[2];
        let t = dot / len2;
        [p[0] - t*n[0], p[1] - t*n[1], p[2] - t*n[2]]
    }
}

/// Plane trihedron (axis indicator for a plane).
/// occt: AIS_PlaneTrihedron
#[derive(Clone, Debug)]
pub struct AisPlaneTrihedron {
    pub plane_id: u32,
    pub size: f64,
    pub x_label: String,
    pub y_label: String,
}

impl AisPlaneTrihedron {
    pub fn new(plane_id: u32, size: f64) -> Self {
        Self { plane_id, size: size.max(0.0), x_label: "X".into(), y_label: "Y".into() }
    }

    pub fn set_size(&mut self, s: f64) { self.size = s.max(0.0); }
    pub fn set_x_label(&mut self, l: impl Into<String>) { self.x_label = l.into(); }
    pub fn set_y_label(&mut self, l: impl Into<String>) { self.y_label = l.into(); }
}

/// Edge relationship dimension (distance, angle, etc.).
/// occt: AIS_EqualRadiusRelation / AIS_IdenticRelation (simplified)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AisEdgeRelationKind {
    #[default]
    Distance,
    Coincident,
    Parallel,
    Perpendicular,
    EqualLength,
    EqualRadius,
    Tangent,
}

#[derive(Clone, Debug)]
pub struct AisEdgeRelation {
    pub edge1_id: u32,
    pub edge2_id: u32,
    pub kind: AisEdgeRelationKind,
    pub value: f64,
    pub is_displayed: bool,
}

impl AisEdgeRelation {
    pub fn new(edge1_id: u32, edge2_id: u32, kind: AisEdgeRelationKind) -> Self {
        Self { edge1_id, edge2_id, kind, value: 0.0, is_displayed: false }
    }

    pub fn set_value(&mut self, v: f64) { self.value = v; }
    pub fn display(&mut self) { self.is_displayed = true; }
    pub fn erase(&mut self) { self.is_displayed = false; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ais_wire_add_edges() {
        let mut w = AisWire::new(1);
        w.add_edge(10);
        w.add_edge(11);
        w.add_edge(12);
        w.add_edge(10); // dup
        assert_eq!(w.nb_edges(), 3);
        assert!(w.is_closed());
    }

    #[test]
    fn ais_wire_not_closed() {
        let mut w = AisWire::new(2);
        w.add_edge(5);
        assert!(!w.is_closed());
    }

    #[test]
    fn ais_face_transparency_clamp() {
        let mut f = AisFace::new(1, 10);
        f.set_transparency(1.5);
        assert!((f.transparency - 1.0).abs() < 1e-6);
        f.set_transparency(-0.2);
        assert!((f.transparency).abs() < 1e-6);
    }

    #[test]
    fn ais_plane_project_point() {
        let p = AisPlane::xoy();
        let proj = p.project_point([3.0, 4.0, 5.0]);
        assert!((proj[0] - 3.0).abs() < 1e-10);
        assert!((proj[1] - 4.0).abs() < 1e-10);
        assert!((proj[2]).abs() < 1e-10);
    }

    #[test]
    fn ais_plane_types() {
        let xoy = AisPlane::xoy();
        assert!((xoy.normal[2] - 1.0).abs() < 1e-14);
        let yoz = AisPlane::yoz();
        assert!((yoz.normal[0] - 1.0).abs() < 1e-14);
    }

    #[test]
    fn ais_plane_trihedron() {
        let mut t = AisPlaneTrihedron::new(1, 100.0);
        t.set_x_label("U");
        t.set_y_label("V");
        assert_eq!(t.x_label, "U");
        assert_eq!(t.size, 100.0);
    }

    #[test]
    fn ais_edge_relation() {
        let mut r = AisEdgeRelation::new(1, 2, AisEdgeRelationKind::Parallel);
        r.display();
        assert!(r.is_displayed);
        r.erase();
        assert!(!r.is_displayed);
    }
}
