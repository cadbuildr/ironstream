// FILE: tests/occt_top_exp.rs
extern crate ironstream;

use ironstream::top_exp::{
    TopExp, TopExpExplorer, IndexedShapeMap, AncestorMap, ShapeEnum,
    TopoDS_Shape, TopoDS_Compound, TopoDS_Solid, TopoDS_Shell,
    TopoDS_Face, TopoDS_Wire, TopoDS_Edge, TopoDS_Vertex,
};

// ---- helpers ----

fn vertex(id: u64) -> TopoDS_Shape {
    TopoDS_Shape::Vertex(TopoDS_Vertex { id, x: 0.0, y: 0.0, z: 0.0 })
}

fn edge(id: u64, v0: u64, v1: u64) -> TopoDS_Shape {
    TopoDS_Shape::Edge(TopoDS_Edge {
        id,
        vertices: vec![
            TopoDS_Vertex { id: v0, x: 0.0, y: 0.0, z: 0.0 },
            TopoDS_Vertex { id: v1, x: 1.0, y: 0.0, z: 0.0 },
        ],
    })
}

fn wire(id: u64, edges: Vec<(u64, u64, u64)>) -> TopoDS_Wire {
    TopoDS_Wire {
        id,
        edges: edges.into_iter().map(|(eid, v0, v1)| TopoDS_Edge {
            id: eid,
            vertices: vec![
                TopoDS_Vertex { id: v0, x: 0.0, y: 0.0, z: 0.0 },
                TopoDS_Vertex { id: v1, x: 1.0, y: 0.0, z: 0.0 },
            ],
        }).collect(),
    }
}

fn face(id: u64, w: TopoDS_Wire) -> TopoDS_Face {
    TopoDS_Face { id, wires: vec![w] }
}

fn shell(id: u64, faces: Vec<TopoDS_Face>) -> TopoDS_Shell {
    TopoDS_Shell { id, faces }
}

fn solid(id: u64, sh: TopoDS_Shell) -> TopoDS_Solid {
    TopoDS_Solid { id, shells: vec![sh] }
}

fn compound(id: u64, shapes: Vec<TopoDS_Shape>) -> TopoDS_Shape {
    TopoDS_Shape::Compound(TopoDS_Compound { id, shapes })
}

// ---- integration tests ----

#[test]
fn test_map_shapes_single_edge_two_vertices() {
    let e = edge(1, 10, 11);
    let mut map = IndexedShapeMap::new();
    TopExp::map_shapes(&e, ShapeEnum::Vertex, &mut map);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_map_shapes_compound_mixed() {
    let e1 = edge(1, 10, 11);
    let e2 = edge(2, 20, 21);
    let c = compound(1, vec![e1, e2]);

    let mut edge_map = IndexedShapeMap::new();
    TopExp::map_shapes(&c, ShapeEnum::Edge, &mut edge_map);
    assert_eq!(edge_map.len(), 2);

    let mut vert_map = IndexedShapeMap::new();
    TopExp::map_shapes(&c, ShapeEnum::Vertex, &mut vert_map);
    assert_eq!(vert_map.len(), 4);
}

#[test]
fn test_map_shapes_nested_compounds() {
    let e1 = edge(1, 10, 11);
    let e2 = edge(2, 12, 13);
    let e3 = edge(3, 14, 15);
    let inner = compound(10, vec![e1, e2]);
    let outer = compound(20, vec![inner, e3]);

    let mut map = IndexedShapeMap::new();
    TopExp::map_shapes(&outer, ShapeEnum::Edge, &mut map);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_map_shapes_shared_vertex_dedup() {
    // Wire: e1(v10-v11), e2(v11-v12) — vertex 11 is shared
    let w = wire(1, vec![(1, 10, 11), (2, 11, 12)]);
    let shape = TopoDS_Shape::Wire(w);
    let mut map = IndexedShapeMap::new();
    TopExp::map_shapes(&shape, ShapeEnum::Vertex, &mut map);
    // 3 unique vertex ids
    assert_eq!(map.len(), 3);
}

#[test]
fn test_map_shapes_and_ancestors_wire_edges_to_vertices() {
    // Wire: e1(v10-v11), e2(v11-v12)
    // v11 belongs to both e1 and e2
    let w = wire(1, vec![(1, 10, 11), (2, 11, 12)]);
    let shape = TopoDS_Shape::Wire(w);

    let mut amap = AncestorMap::new();
    TopExp::map_shapes_and_ancestors(&shape, ShapeEnum::Vertex, ShapeEnum::Edge, &mut amap);

    assert_eq!(amap.find_map.len(), 3); // v10, v11, v12

    let v11 = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 11, x: 0.0, y: 0.0, z: 0.0 });
    let ancs = amap.ancestors_of(&v11);
    assert_eq!(ancs.len(), 2, "v11 should appear in 2 edges");

    let v10 = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 10, x: 0.0, y: 0.0, z: 0.0 });
    let ancs10 = amap.ancestors_of(&v10);
    assert_eq!(ancs10.len(), 1);
}

#[test]
fn test_map_shapes_and_ancestors_shell_faces_to_edges() {
    let w1 = wire(1, vec![(10, 100, 101), (11, 101, 102), (12, 102, 100)]);
    let w2 = wire(2, vec![(20, 200, 201), (21, 201, 202), (22, 202, 200)]);
    let f1 = face(1, w1);
    let f2 = face(2, w2);
    let sh = shell(1, vec![f1, f2]);
    let shape = TopoDS_Shape::Shell(sh);

    let mut amap = AncestorMap::new();
    TopExp::map_shapes_and_ancestors(&shape, ShapeEnum::Edge, ShapeEnum::Face, &mut amap);

    // 6 distinct edges, each with exactly 1 ancestor face
    assert_eq!(amap.find_map.len(), 6);

    let e10 = TopoDS_Shape::Edge(TopoDS_Edge {
        id: 10,
        vertices: vec![
            TopoDS_Vertex { id: 100, x: 0.0, y: 0.0, z: 0.0 },
            TopoDS_Vertex { id: 101, x: 1.0, y: 0.0, z: 0.0 },
        ],
    });
    let ancs = amap.ancestors_of(&e10);
    assert_eq!(ancs.len(), 1);
}

#[test]
fn test_explorer_vertices_from_compound() {
    let e1 = edge(1, 10, 11);
    let e2 = edge(2, 12, 13);
    let c = compound(1, vec![e1, e2]);

    let mut ex = TopExpExplorer::new();
    ex.init(c, ShapeEnum::Vertex, None);
    let mut count = 0;
    while ex.more() {
        count += 1;
        ex.next();
    }
    assert_eq!(count, 4);
}

#[test]
fn test_explorer_faces_from_solid() {
    let w1 = wire(1, vec![(10, 100, 101), (11, 101, 102), (12, 102, 100)]);
    let w2 = wire(2, vec![(20, 200, 201), (21, 201, 202), (22, 202, 200)]);
    let w3 = wire(3, vec![(30, 300, 301), (31, 301, 302), (32, 302, 300)]);
    let f1 = face(1, w1);
    let f2 = face(2, w2);
    let f3 = face(3, w3);
    let sh = shell(1, vec![f1, f2, f3]);
    let sol = solid(1, sh);
    let shape = TopoDS_Shape::Solid(sol);

    let mut ex = TopExpExplorer::new();
    ex.init(shape, ShapeEnum::Face, None);
    let faces = ex.collect_all();
    assert_eq!(faces.len(), 3);
}

#[test]
fn test_explorer_to_avoid_wires_skips_edges() {
    // Solid → Shell → Face → Wire → Edge
    // If we avoid Wire, we should not find Edges
    let w1 = wire(1, vec![(10, 100, 101), (11, 101, 102)]);
    let f1 = face(1, w1);
    let sh = shell(1, vec![f1]);
    let sol = solid(1, sh);
    let shape = TopoDS_Shape::Solid(sol);

    let mut ex = TopExpExplorer::new();
    ex.init(shape, ShapeEnum::Edge, Some(ShapeEnum::Wire));
    assert!(!ex.more(), "should find no edges when wires are avoided");
}

#[test]
fn test_explorer_to_avoid_allows_sibling_path() {
    // Compound has a wire (with edges) and a standalone vertex
    // Avoid edges → standalone vertex still found, edges not found
    let w = wire(1, vec![(10, 100, 101)]);
    let wire_shape = TopoDS_Shape::Wire(w);
    let v = vertex(200);
    let c = compound(1, vec![wire_shape, v.clone()]);

    let mut ex = TopExpExplorer::new();
    ex.init(c, ShapeEnum::Vertex, Some(ShapeEnum::Edge));
    let found = ex.collect_all();
    // Vertex 200 is directly in compound (no edge barrier), and vertices
    // inside the wire are blocked by edge avoidance.
    // Wire → Edge (avoided) → Vertex blocked
    // Compound → Vertex(200) directly
    assert!(found.iter().any(|s| {
        if let TopoDS_Shape::Vertex(vv) = s { vv.id == 200 } else { false }
    }));
}

#[test]
fn test_explorer_reinit_multiple_times() {
    let e = edge(1, 10, 11);

    let mut ex = TopExpExplorer::new();
    for _ in 0..3 {
        ex.init(e.clone(), ShapeEnum::Vertex, None);
        let verts = ex.collect_all();
        assert_eq!(verts.len(), 2, "should find 2 vertices each reinit");
    }
}

#[test]
fn test_explorer_no_match_type_above() {
    // looking for solid within a wire — impossible
    let w = wire(1, vec![(10, 100, 101)]);
    let shape = TopoDS_Shape::Wire(w);
    let mut ex = TopExpExplorer::new();
    ex.init(shape, ShapeEnum::Solid, None);
    assert!(!ex.more());
}

#[test]
fn test_map_shapes_dedup_across_compound() {
    // Two edges sharing vertex id 11 in a compound
    let e1 = edge(1, 10, 11);
    let e2 = edge(2, 11, 12);
    let c = compound(1, vec![e1, e2]);

    let mut map = IndexedShapeMap::new();
    TopExp::map_shapes(&c, ShapeEnum::Vertex, &mut map);
    // ids: 10, 11, 12 — vertex 11 appears twice but deduped by id
    assert_eq!(map.len(), 3);
}

#[test]
fn test_indexed_shape_map_is_empty() {
    let map = IndexedShapeMap::new();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[test]
fn test_ancestor_map_absent_shape() {
    let amap = AncestorMap::new();
    let v = vertex(999);
    let ancs = amap.ancestors_of(&v);
    assert!(ancs.is_empty());
}

#[test]
fn test_shape_enum_eq() {
    assert_eq!(ShapeEnum::Face, ShapeEnum::Face);
    assert_ne!(ShapeEnum::Face, ShapeEnum::Edge);
}

#[test]
fn test_explorer_compound_of_compounds_vertices() {
    let e1 = edge(1, 10, 11);
    let e2 = edge(2, 12, 13);
    let c1 = compound(1, vec![e1]);
    let c2 = compound(2, vec![e2]);
    let outer = compound(3, vec![c1, c2]);

    let mut ex = TopExpExplorer::new();
    ex.init(outer, ShapeEnum::Vertex, None);
    let verts = ex.collect_all();
    assert_eq!(verts.len(), 4);
}

#[test]
fn test_map_shapes_self_match_compound() {
    // Ask for Compound from a Compound — should return itself
    let c = compound(1, vec![]);
    let mut map = IndexedShapeMap::new();
    TopExp::map_shapes(&c, ShapeEnum::Compound, &mut map);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_map_shapes_self_match_edge() {
    let e = edge(1, 10, 11);
    let mut map = IndexedShapeMap::new();
    TopExp::map_shapes(&e, ShapeEnum::Edge, &mut map);
    assert_eq!(map.len(), 1);
}
