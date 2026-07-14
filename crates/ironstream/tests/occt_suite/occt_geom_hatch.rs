extern crate ironstream;
use ironstream::geom_hatch::*;

#[test]
fn test_geom_hatch_basic() {
    // HatchDomain construction and normalisation
    let d = HatchDomain::new(0.2, 0.8);
    assert_eq!(d.u_start, 0.2);
    assert_eq!(d.u_end, 0.8);
    assert_eq!(d.edge_index, -1);

    // swapped params normalise correctly
    let d2 = HatchDomain::new(1.0, 0.0);
    assert_eq!(d2.u_start, 0.0);
    assert_eq!(d2.u_end, 1.0);

    // length and contains
    assert!((d.length() - 0.6).abs() < 1e-12);
    assert!(d.contains(0.5));
    assert!(!d.contains(0.2)); // strict interior
    assert!(!d.contains(0.8)); // strict interior

    // Hatcher
    let mut h = Hatcher::new(1e-7);
    assert_eq!(h.tolerance, 1e-7);
    assert_eq!(h.nb_domains(), 0);
    assert!(!h.is_done());

    // add_element and trim
    h.add_element([0.0, 0.5], [1.0, 0.0]);
    h.trim(0.1, 0.9);
    assert_eq!(h.nb_domains(), 1);
    assert!(h.is_done());

    let dom = h.domain(0).expect("domain 0 exists");
    assert!(dom.u_start < dom.u_end);

    // hatch_pattern free function
    let segs = hatch_pattern(
        [0.0, 0.0],
        [1.0, 0.0],
        0.25,
        0.0,
        ([0.0, 0.0], [2.0, 2.0]),
    );
    // should produce at least one segment inside the bbox
    assert!(!segs.is_empty());
    for seg in &segs {
        let _ = seg[0]; // [[f64;2]; 2]
        let _ = seg[1];
    }
}
