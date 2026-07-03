extern crate ironstream;
use ironstream::topo_shape_tools::*;

#[test]
fn test_topo_shape_tools_basic() {
    // ShapeType display
    let st = ShapeType::Edge;
    assert_eq!(format!("{}", st), "Edge");
    assert_eq!(shape_type_name(&ShapeType::Vertex), "Vertex");
    assert_eq!(shape_type_name(&ShapeType::Face), "Face");
    assert_eq!(shape_type_name(&ShapeType::Solid), "Solid");

    // ShapeInfo
    let info = ShapeInfo::new(ShapeType::Wire, "my_wire");
    assert_eq!(info.shape_type, ShapeType::Wire);
    assert_eq!(info.name, "my_wire");
    assert!(!info.is_null);
    assert!(info.is_closed());

    let face_info = ShapeInfo::new(ShapeType::Face, "face_1");
    assert!(!face_info.is_closed()); // faces are not closed in this stub

    // ShapeTools
    let outer = ShapeTools::outer_wire("face_A");
    assert!(outer.contains("face_A"));

    let data = ShapeTools::write_shape("my_shape");
    assert!(!data.is_empty());
    let read_back = ShapeTools::read_shape(&data);
    assert_eq!(read_back, "my_shape");

    let (min, max) = ShapeTools::bounds("test_shape");
    // bounds are deterministic stubs
    assert!(min[0] <= max[0]);

    // map_shapes
    let subs = map_shapes("root", ShapeType::Edge);
    assert!(!subs.is_empty());
    for s in &subs {
        assert!(s.contains("root"));
    }
}
