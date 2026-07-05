// FILE: db_rep_edge.rs
// occt: DBRep_Edge

//! Display of an edge with associated color.
//! Stores an edge and its display color.

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

    pub fn green() -> Self {
        DrawColor {
            r: 0,
            g: 255,
            b: 0,
        }
    }

    pub fn blue() -> Self {
        DrawColor {
            r: 0,
            g: 0,
            b: 255,
        }
    }
}

/// A simplified representation of a TopoDS_Edge.
#[derive(Clone, Debug, PartialEq)]
pub struct TopodsEdge {
    id: u32,
    vertices: (u32, u32),
}

impl TopodsEdge {
    pub fn new(id: u32, start_vertex: u32, end_vertex: u32) -> Self {
        TopodsEdge {
            id,
            vertices: (start_vertex, end_vertex),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn start_vertex(&self) -> u32 {
        self.vertices.0
    }

    pub fn end_vertex(&self) -> u32 {
        self.vertices.1
    }
}

/// DBRep_Edge: An edge with associated display color.
#[derive(Clone, Debug)]
pub struct DbrepEdge {
    edge: TopodsEdge,
    color: DrawColor,
}

impl DbrepEdge {
    /// Create a new edge with a color.
    pub fn new(edge: TopodsEdge, color: DrawColor) -> Self {
        DbrepEdge { edge, color }
    }

    /// Get a reference to the edge.
    pub fn edge(&self) -> &TopodsEdge {
        &self.edge
    }

    /// Set the edge.
    pub fn set_edge(&mut self, edge: TopodsEdge) {
        self.edge = edge;
    }

    /// Get the color.
    pub fn color(&self) -> DrawColor {
        self.color
    }

    /// Set the color.
    pub fn set_color(&mut self, color: DrawColor) {
        self.color = color;
    }

    /// Get edge ID.
    pub fn edge_id(&self) -> u32 {
        self.edge.id()
    }
}

impl Default for DbrepEdge {
    fn default() -> Self {
        DbrepEdge {
            edge: TopodsEdge::new(0, 0, 0),
            color: DrawColor::black(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_color_creation() {
        let color = DrawColor::new(100, 150, 200);
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
    }

    #[test]
    fn test_draw_color_predefined() {
        assert_eq!(DrawColor::white().r, 255);
        assert_eq!(DrawColor::black().r, 0);
        assert_eq!(DrawColor::red().g, 0);
        assert_eq!(DrawColor::green().r, 0);
        assert_eq!(DrawColor::blue().b, 255);
    }

    #[test]
    fn test_topods_edge_creation() {
        let edge = TopodsEdge::new(1, 10, 20);
        assert_eq!(edge.id(), 1);
        assert_eq!(edge.start_vertex(), 10);
        assert_eq!(edge.end_vertex(), 20);
    }

    #[test]
    fn test_dbrep_edge_creation() {
        let edge = TopodsEdge::new(42, 1, 2);
        let color = DrawColor::red();
        let dbrep_edge = DbrepEdge::new(edge.clone(), color);

        assert_eq!(dbrep_edge.edge_id(), 42);
        assert_eq!(dbrep_edge.color(), color);
        assert_eq!(dbrep_edge.edge(), &edge);
    }

    #[test]
    fn test_dbrep_edge_set_edge() {
        let mut dbrep_edge = DbrepEdge::default();
        let new_edge = TopodsEdge::new(99, 5, 6);

        dbrep_edge.set_edge(new_edge.clone());
        assert_eq!(dbrep_edge.edge(), &new_edge);
    }

    #[test]
    fn test_dbrep_edge_set_color() {
        let mut dbrep_edge = DbrepEdge::default();
        let new_color = DrawColor::blue();

        dbrep_edge.set_color(new_color);
        assert_eq!(dbrep_edge.color(), new_color);
    }

    #[test]
    fn test_dbrep_edge_default() {
        let dbrep_edge = DbrepEdge::default();
        assert_eq!(dbrep_edge.edge_id(), 0);
        assert_eq!(dbrep_edge.color(), DrawColor::black());
    }

    #[test]
    fn test_dbrep_edge_clone() {
        let edge = TopodsEdge::new(7, 3, 4);
        let color = DrawColor::green();
        let dbrep_edge = DbrepEdge::new(edge, color);

        let cloned = dbrep_edge.clone();
        assert_eq!(cloned.edge_id(), 7);
        assert_eq!(cloned.color(), color);
    }

    #[test]
    fn test_multiple_edges() {
        let edge1 = TopodsEdge::new(1, 10, 11);
        let edge2 = TopodsEdge::new(2, 20, 21);
        let edge3 = TopodsEdge::new(3, 30, 31);

        let dbrep1 = DbrepEdge::new(edge1, DrawColor::red());
        let dbrep2 = DbrepEdge::new(edge2, DrawColor::green());
        let dbrep3 = DbrepEdge::new(edge3, DrawColor::blue());

        assert_eq!(dbrep1.edge_id(), 1);
        assert_eq!(dbrep2.edge_id(), 2);
        assert_eq!(dbrep3.edge_id(), 3);

        assert_eq!(dbrep1.color(), DrawColor::red());
        assert_eq!(dbrep2.color(), DrawColor::green());
        assert_eq!(dbrep3.color(), DrawColor::blue());
    }

    #[test]
    fn test_draw_color_equality() {
        let color1 = DrawColor::new(50, 100, 150);
        let color2 = DrawColor::new(50, 100, 150);
        let color3 = DrawColor::new(50, 100, 151);

        assert_eq!(color1, color2);
        assert_ne!(color1, color3);
    }

    #[test]
    fn test_topods_edge_equality() {
        let edge1 = TopodsEdge::new(5, 10, 20);
        let edge2 = TopodsEdge::new(5, 10, 20);
        let edge3 = TopodsEdge::new(6, 10, 20);

        assert_eq!(edge1, edge2);
        assert_ne!(edge1, edge3);
    }
}
