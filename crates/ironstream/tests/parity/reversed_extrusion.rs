//! Regression: extruding a face *against* its normal (foundation Extrusion with
//! start > end and no taper) must give an outward-oriented solid, so the next
//! boolean keeps the body. Before the fix the side walls kept the forward
//! winding while the caps flipped, and a cut deleted ~all of NEMA17.
use ironstream::brep_algo_api::{cut, fuse};
use ironstream::brep_prim_api::{make_box, make_prism};
use ironstream::gp::Pnt;
use ironstream::topods::{Face, Wire};

fn square(z: f64, s: f64) -> Wire {
    Wire::new(vec![
        Pnt::new(0.0, 0.0, z),
        Pnt::new(s, 0.0, z),
        Pnt::new(s, s, z),
        Pnt::new(0.0, s, z),
    ])
}

fn consistent(sol: &ironstream::topods::Solid) {
    // closed and consistently wound
    let m = sol.mesh();
    let mut edges = std::collections::HashMap::new();
    for t in &m.tris {
        for k in 0..3 {
            *edges.entry((t[k], t[(k + 1) % 3])).or_insert(0) += 1;
        }
    }
    // keyhole-bridged caps may repeat an edge, so compare multiplicities:
    // a consistently oriented closed mesh uses a->b exactly as often as b->a.
    for (&(a, b), &n) in &edges {
        let back = edges.get(&(b, a)).copied().unwrap_or(0);
        assert_eq!(n, back, "edge {a}->{b} x{n} but {b}->{a} x{back}");
    }
}

#[test]
fn downward_prism_is_outward() {
    let down = make_prism(&Face::new(square(10.0, 10.0)), Pnt::new(0.0, 0.0, -10.0));
    consistent(&down);
    assert!((down.mesh().signed_volume() - 1000.0).abs() < 1e-6);
    let bb = down.bbox();
    assert!((bb.min[2] - 0.0).abs() < 1e-9 && (bb.max[2] - 10.0).abs() < 1e-9);
}

#[test]
fn downward_prism_with_hole_is_outward() {
    let hole = Wire::new(vec![
        Pnt::new(3.0, 3.0, 5.0),
        Pnt::new(3.0, 7.0, 5.0),
        Pnt::new(7.0, 7.0, 5.0),
        Pnt::new(7.0, 3.0, 5.0),
    ]);
    let f = Face::with_holes(square(5.0, 10.0), vec![hole]);
    let down = make_prism(&f, Pnt::new(0.0, 0.0, -5.0));
    consistent(&down);
    assert!((down.mesh().signed_volume() - (100.0 - 16.0) * 5.0).abs() < 1e-6);
}

#[test]
fn cut_after_downward_prism_keeps_body() {
    // NEMA17-style: body extruded downward, then a pocket cut from the top.
    let body = make_prism(&Face::new(square(42.0, 42.0)), Pnt::new(0.0, 0.0, -40.0));
    let pocket = make_box(Pnt::new(10.0, 10.0, 30.0), 22.0, 22.0, 20.0);
    let out = cut(&body, &pocket);
    let want = 42.0 * 42.0 * 40.0 - 22.0 * 22.0 * 12.0;
    assert!((out.volume() - want).abs() / want < 1e-6, "{} vs {}", out.volume(), want);
    let both = fuse(&body, &make_box(Pnt::new(0.0, 0.0, -10.0), 42.0, 42.0, 12.0));
    let want = 42.0 * 42.0 * 52.0;
    assert!((both.volume() - want).abs() / want < 1e-6, "{} vs {}", both.volume(), want);
}

#[test]
fn upward_prism_with_hole_is_outward() {
    let hole = Wire::new(vec![
        Pnt::new(3.0, 3.0, 0.0),
        Pnt::new(3.0, 7.0, 0.0),
        Pnt::new(7.0, 7.0, 0.0),
        Pnt::new(7.0, 3.0, 0.0),
    ]);
    let f = Face::with_holes(square(0.0, 10.0), vec![hole]);
    let up = make_prism(&f, Pnt::new(0.0, 0.0, 5.0));
    consistent(&up);
    assert!((up.mesh().signed_volume() - (100.0 - 16.0) * 5.0).abs() < 1e-6);
}

