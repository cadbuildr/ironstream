// FILE: graphic3d_poly.rs
// occt: Graphic3d_ArrayOfPolygons, Graphic3d_ArrayOfPolylines,
//       Graphic3d_ArrayOfPoints, Graphic3d_ArrayOfSegments

/// Primitive type for a graphics vertex array.
/// occt: Graphic3d_TypeOfPrimitiveArray
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Graphic3dPrimitiveType {
    #[default]
    Triangles,
    TrianglesStrip,
    TrianglesFan,
    Polygons,
    Segments,
    Polylines,
    Points,
    Quads,
    QuadsStrip,
}

/// Vertex data flags.
#[derive(Clone, Copy, Debug, Default)]
pub struct VertexFlags {
    pub has_normals: bool,
    pub has_tex_coords: bool,
    pub has_colors: bool,
}

/// A vertex in a primitive array.
#[derive(Clone, Copy, Debug, Default)]
pub struct Graphic3dVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
    pub color: [f32; 4],
}

impl Graphic3dVertex {
    pub fn from_pos(p: [f32; 3]) -> Self { Self { position: p, ..Default::default() } }

    pub fn with_normal(mut self, n: [f32; 3]) -> Self { self.normal = n; self }
    pub fn with_tex(mut self, uv: [f32; 2]) -> Self { self.tex_coord = uv; self }
    pub fn with_color(mut self, c: [f32; 4]) -> Self { self.color = c; self }
}

/// Generic vertex array for 3D primitives.
/// occt: Graphic3d_ArrayOfPrimitives
#[derive(Clone, Debug)]
pub struct Graphic3dArrayOfPrimitives {
    pub primitive_type: Graphic3dPrimitiveType,
    pub flags: VertexFlags,
    pub vertices: Vec<Graphic3dVertex>,
    pub indices: Vec<u32>,
    pub bounds: Vec<u32>,   // polygon/polyline boundary lengths
}

impl Graphic3dArrayOfPrimitives {
    pub fn new(primitive_type: Graphic3dPrimitiveType, flags: VertexFlags) -> Self {
        Self { primitive_type, flags, vertices: Vec::new(), indices: Vec::new(), bounds: Vec::new() }
    }

    pub fn add_vertex(&mut self, v: Graphic3dVertex) -> u32 {
        let idx = self.vertices.len() as u32;
        self.vertices.push(v);
        idx
    }

    pub fn add_edge(&mut self, idx: u32) { self.indices.push(idx); }

    pub fn add_bound(&mut self, nb: u32) { self.bounds.push(nb); }

    pub fn nb_vertices(&self) -> usize { self.vertices.len() }
    pub fn nb_edges(&self) -> usize { self.indices.len() }
    pub fn nb_bounds(&self) -> usize { self.bounds.len() }

    pub fn is_indexed(&self) -> bool { !self.indices.is_empty() }

    pub fn vertex_position(&self, idx: usize) -> Option<[f32; 3]> {
        self.vertices.get(idx).map(|v| v.position)
    }
}

/// Convenience alias for polygon arrays.
/// occt: Graphic3d_ArrayOfPolygons
pub struct Graphic3dArrayOfPolygons(pub Graphic3dArrayOfPrimitives);

impl Graphic3dArrayOfPolygons {
    pub fn new(has_normals: bool, has_tex: bool) -> Self {
        let flags = VertexFlags { has_normals, has_tex_coords: has_tex, has_colors: false };
        Self(Graphic3dArrayOfPrimitives::new(Graphic3dPrimitiveType::Polygons, flags))
    }

    pub fn add_vertex(&mut self, pos: [f32; 3]) -> u32 {
        self.0.add_vertex(Graphic3dVertex::from_pos(pos))
    }

    pub fn add_bound(&mut self, nb: u32) { self.0.add_bound(nb); }
    pub fn nb_vertices(&self) -> usize { self.0.nb_vertices() }
    pub fn nb_bounds(&self) -> usize { self.0.nb_bounds() }
}

/// Convenience alias for polyline arrays.
/// occt: Graphic3d_ArrayOfPolylines
pub struct Graphic3dArrayOfPolylines(pub Graphic3dArrayOfPrimitives);

impl Graphic3dArrayOfPolylines {
    pub fn new(has_colors: bool) -> Self {
        let flags = VertexFlags { has_normals: false, has_tex_coords: false, has_colors };
        Self(Graphic3dArrayOfPrimitives::new(Graphic3dPrimitiveType::Polylines, flags))
    }

    pub fn add_vertex(&mut self, pos: [f32; 3]) -> u32 {
        self.0.add_vertex(Graphic3dVertex::from_pos(pos))
    }

    pub fn add_bound(&mut self, nb: u32) { self.0.add_bound(nb); }
    pub fn nb_vertices(&self) -> usize { self.0.nb_vertices() }
    pub fn nb_bounds(&self) -> usize { self.0.nb_bounds() }
}

/// Convenience alias for point arrays.
/// occt: Graphic3d_ArrayOfPoints
pub struct Graphic3dArrayOfPoints(pub Graphic3dArrayOfPrimitives);

impl Graphic3dArrayOfPoints {
    pub fn new(has_normals: bool) -> Self {
        let flags = VertexFlags { has_normals, has_tex_coords: false, has_colors: true };
        Self(Graphic3dArrayOfPrimitives::new(Graphic3dPrimitiveType::Points, flags))
    }

    pub fn add_vertex(&mut self, pos: [f32; 3]) -> u32 {
        self.0.add_vertex(Graphic3dVertex::from_pos(pos))
    }

    pub fn nb_vertices(&self) -> usize { self.0.nb_vertices() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives_add_vertices() {
        let mut arr = Graphic3dArrayOfPrimitives::new(
            Graphic3dPrimitiveType::Triangles,
            VertexFlags::default());
        let i0 = arr.add_vertex(Graphic3dVertex::from_pos([0.0, 0.0, 0.0]));
        let i1 = arr.add_vertex(Graphic3dVertex::from_pos([1.0, 0.0, 0.0]));
        arr.add_edge(i0);
        arr.add_edge(i1);
        assert_eq!(arr.nb_vertices(), 2);
        assert_eq!(arr.nb_edges(), 2);
        assert!(arr.is_indexed());
    }

    #[test]
    fn polygons_add() {
        let mut p = Graphic3dArrayOfPolygons::new(true, false);
        p.add_vertex([0.0, 0.0, 0.0]);
        p.add_vertex([1.0, 0.0, 0.0]);
        p.add_vertex([0.5, 1.0, 0.0]);
        p.add_bound(3);
        assert_eq!(p.nb_vertices(), 3);
        assert_eq!(p.nb_bounds(), 1);
    }

    #[test]
    fn polylines_add() {
        let mut p = Graphic3dArrayOfPolylines::new(false);
        p.add_vertex([0.0, 0.0, 0.0]);
        p.add_vertex([1.0, 1.0, 0.0]);
        p.add_bound(2);
        assert_eq!(p.nb_vertices(), 2);
    }

    #[test]
    fn points_add() {
        let mut pts = Graphic3dArrayOfPoints::new(false);
        for i in 0..5 { pts.add_vertex([i as f32, 0.0, 0.0]); }
        assert_eq!(pts.nb_vertices(), 5);
    }

    #[test]
    fn vertex_builder() {
        let v = Graphic3dVertex::from_pos([1.0, 2.0, 3.0])
            .with_normal([0.0, 0.0, 1.0])
            .with_tex([0.5, 0.5]);
        assert_eq!(v.position, [1.0, 2.0, 3.0]);
        assert_eq!(v.normal, [0.0, 0.0, 1.0]);
        assert_eq!(v.tex_coord, [0.5, 0.5]);
    }
}
