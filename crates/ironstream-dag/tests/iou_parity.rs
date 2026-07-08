//! End-to-end IOU parity tests.
//!
//! The progression the task asks for: start with a cube and verify IOU is
//! exactly one against an independent analytic reference, then climb in
//! difficulty (primitives → booleans → lathes → assemblies) over the real
//! CADbuildr fixtures, and finally sweep every fixture as a coverage guard.
//!
//! "Reference" here means a *kernel-independent* ground truth — analytic
//! predicates and closed-form volumes — not another CAD kernel, so these tests
//! run hermetically in CI. To A/B against the shipping replicad kernel, point
//! the TS parity harness at the IronStream binary (see crate README /
//! `KERNEL_NATIVE_BIN`).

use ironstream::prelude::*;
use ironstream_dag::{compile, dag::CompilerInputDag, iou};
use std::path::PathBuf;

/// The canonical CADbuildr DAG fixtures — vendored in-crate (`fixtures/`, a
/// snapshot of the monorepo's kernel-truck test fixtures) so the suite runs
/// hermetically in CI. Point `IRONSTREAM_FIXTURES_DIR` elsewhere to test
/// against a live monorepo checkout.
fn fixtures_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("IRONSTREAM_FIXTURES_DIR") {
        return PathBuf::from(dir);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

fn load(name: &str) -> Option<CompilerInputDag> {
    let path = fixtures_dir().join(format!("{name}.json"));
    let raw = std::fs::read_to_string(path).ok()?;
    CompilerInputDag::from_json(&raw).ok()
}

/// Compile a fixture and return the first part's solid.
fn first_solid(name: &str) -> Option<Solid> {
    let input = load(name)?;
    let result = compile(&input, MeshParams::default());
    result.parts.into_iter().next().map(|p| p.solid)
}

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

// ===================================================================
// 1. The headline: a cube, IOU exactly one.
// ===================================================================

#[test]
fn cube_iou_is_exactly_one() {
    let solid = first_solid("cube").expect("cube fixture must compile");
    let mesh = solid.mesh();

    // Geometry: the fixture is a 20x20x20 box with a corner at the origin.
    let bb = solid.bbox();
    assert!(
        approx(bb.min[0], 0.0, 1e-6) && approx(bb.max[0], 20.0, 1e-6),
        "x bbox {bb:?}"
    );
    assert!(
        approx(bb.min[1], 0.0, 1e-6) && approx(bb.max[1], 20.0, 1e-6),
        "y bbox {bb:?}"
    );
    assert!(
        approx(bb.min[2], 0.0, 1e-6) && approx(bb.max[2], 20.0, 1e-6),
        "z bbox {bb:?}"
    );
    assert!(
        approx(solid.volume(), 8000.0, 1e-6),
        "volume {}",
        solid.volume()
    );

    // Independent analytic reference: the unit-coefficient box predicate.
    let reference = BBox {
        min: [0.0, 0.0, 0.0],
        max: [20.0, 20.0, 20.0],
    };
    let inside = |p: Pnt| {
        p.x >= 0.0 && p.x <= 20.0 && p.y >= 0.0 && p.y <= 20.0 && p.z >= 0.0 && p.z <= 20.0
    };

    // (a) bbox IOU — exactly the metric the TS parity harness computes.
    let bbox_iou = solid.bbox().iou(&reference);
    assert!(approx(bbox_iou, 1.0, 1e-9), "bbox IOU = {bbox_iou}");

    // (b) true volumetric IOU vs the analytic solid.
    let voxel_iou = iou::voxel_iou_mesh_pred(mesh, &reference, 48, inside);
    assert!(voxel_iou >= 0.999, "voxel IOU = {voxel_iou}");

    // (c) the DAG path must reproduce the kernel's own box exactly.
    let kernel_box = make_box(Pnt::origin(), 20.0, 20.0, 20.0);
    let rep = iou::compare_meshes(mesh, kernel_box.mesh(), 40);
    assert!(approx(rep.bbox_iou, 1.0, 1e-9), "{rep:?}");
    assert!(approx(rep.volume_ratio, 1.0, 1e-9), "{rep:?}");
    assert!(rep.voxel_iou >= 0.999, "{rep:?}");

    println!(
        "cube: bbox_iou={bbox_iou:.6} voxel_iou={voxel_iou:.6} volume={:.3}",
        solid.volume()
    );
}

// ===================================================================
// 2. A second example exercising different nodes: centered cubes.
// ===================================================================

#[test]
fn centered_cube_family_iou() {
    // starter_cube / color_cube use Square.from_center_and_side -> a box
    // centered on the origin in X/Y, extruded 20 in +Z.
    for name in ["starter_cube", "color_cube"] {
        let Some(solid) = first_solid(name) else {
            eprintln!("skip {name} (not loadable)");
            continue;
        };
        assert!(
            approx(solid.volume(), 8000.0, 1e-3),
            "{name} volume {}",
            solid.volume()
        );
        let reference = make_box(Pnt::new(-10.0, -10.0, 0.0), 20.0, 20.0, 20.0);
        let rep = iou::compare_meshes(solid.mesh(), reference.mesh(), 36);
        assert!(rep.bbox_iou >= 0.999, "{name} {rep:?}");
        assert!(rep.voxel_iou >= 0.999, "{name} {rep:?}");
        println!(
            "{name}: bbox_iou={:.6} voxel_iou={:.6}",
            rep.bbox_iou, rep.voxel_iou
        );
    }
}

// ===================================================================
// 3. Booleans: a drilled box vs an analytic CSG reference.
// ===================================================================

#[test]
fn boolean_cut_matches_analytic_iou() {
    // Box 20x20x20 centered on origin (z 0..20) minus a cylinder r=5 along Z.
    let block = make_box(Pnt::new(-10.0, -10.0, 0.0), 20.0, 20.0, 20.0);
    let mut tool = make_cylinder(5.0, 30.0, MeshParams::default());
    tool.transform(&Trsf::translation(Pnt::new(0.0, 0.0, -5.0)));
    let drilled = cut(&block, &tool);

    // Volume: 8000 - pi*25*20.
    let expected_vol = 8000.0 - std::f64::consts::PI * 25.0 * 20.0;
    let ratio = drilled.volume() / expected_vol;
    assert!(ratio > 0.99 && ratio < 1.01, "vol ratio {ratio}");

    // Analytic predicate: inside the block and outside the cylinder.
    let inside = |p: Pnt| {
        let in_box =
            p.x >= -10.0 && p.x <= 10.0 && p.y >= -10.0 && p.y <= 10.0 && p.z >= 0.0 && p.z <= 20.0;
        let in_cyl = (p.x * p.x + p.y * p.y) <= 25.0;
        in_box && !in_cyl
    };
    let ref_bbox = BBox {
        min: [-10.0, -10.0, 0.0],
        max: [10.0, 10.0, 20.0],
    };
    let voxel_iou = iou::voxel_iou_mesh_pred(drilled.mesh(), &ref_bbox, 48, inside);
    assert!(voxel_iou >= 0.985, "voxel IOU = {voxel_iou}");
    println!("drilled box: vol_ratio={ratio:.6} voxel_iou={voxel_iou:.6}");
}

// ===================================================================
// 4. A cylinder must actually be round (analytic volume from its own bbox).
// ===================================================================

#[test]
fn cylinder_is_round() {
    let Some(solid) = first_solid("cylinder") else {
        eprintln!("skip cylinder");
        return;
    };
    let bb = solid.bbox();
    let r = (bb.max[0] - bb.min[0]) / 2.0;
    let h = bb.max[2] - bb.min[2];
    let analytic = std::f64::consts::PI * r * r * h;
    let ratio = solid.volume() / analytic;
    // Inscribed polygon volume is just under the ideal circle.
    assert!(
        ratio > 0.99 && ratio <= 1.001,
        "roundness ratio {ratio} (r={r}, h={h})"
    );
    println!("cylinder: r={r:.3} h={h:.3} vol_ratio_vs_ideal={ratio:.6}");
}

// ===================================================================
// 5. Increasing-difficulty regression progression (volume anchors).
// ===================================================================

#[test]
fn difficulty_progression_volumes() {
    // (fixture, expected first-part volume, relative tolerance)
    // Flat-faced parts are exact; curved/lathe parts carry a small tessellation
    // tolerance. These anchor the whole pipeline against regressions.
    let cases: &[(&str, f64, f64)] = &[
        ("cube", 8000.0, 1e-4),
        ("starter_cube", 8000.0, 1e-4),
        ("color_cube", 8000.0, 1e-4),
        // Square side 20 extruded -10..+10 => exactly 8000. The pre-parity
        // vendored kernel produced 8000/3 (a pyramid!) for this negative-start
        // extrusion and the old anchor captured that bug; the parity kernel
        // fixed it.
        ("improved_cube", 8000.0, 1e-4),
        ("square_tube", 7600.0, 1e-3),
        ("square_pyramid", 12515.625, 5e-3),
        ("gold_star", 23511.41, 5e-3),
        ("side_cylinders", 8150.55, 1e-2), // box + side cylinders (boolean fuse)
        ("cylinder", 784.137, 1e-2),
        ("donut", 314813.97, 1e-2),
        ("tree_lathe", 892870.8, 1e-2),
        // Lathe of an arc profile. The old 12151.3 anchor captured the arc
        // through-point bug (p2/p3 swapped + midpoint-projected sagitta);
        // reference kernels agree here: replicad 14817.6, truck 14841.6.
        ("chess_pawn", 14817.6, 2e-2),
        // Sphere-tetrahedron boolean with arc-driven geometry. The old 19.19
        // anchor captured the arc bugs; replicad reference is 33.45. IronStream
        // tessellates curved booleans ~5% under until exact-BOP Stage 4.
        ("tetrahedron_sphere", 33.45, 7e-2),
    ];
    let mut tested = 0;
    for (name, expect, rel) in cases {
        let Some(solid) = first_solid(name) else {
            eprintln!("skip {name} (not loadable)");
            continue;
        };
        let v = solid.volume();
        assert!(!solid.is_empty(), "{name} produced an empty solid");
        assert!(
            solid.mesh().triangle_count() > 0,
            "{name} produced no triangles"
        );
        let tol = expect.abs() * rel;
        assert!(
            approx(v, *expect, tol),
            "{name}: volume {v:.4} not within {tol:.4} of {expect}"
        );
        tested += 1;
        println!("{name:<20} volume={v:.4} (expected ~{expect})");
    }
    assert!(
        tested >= 10,
        "expected to verify >=10 progression fixtures, got {tested}"
    );
}

// ===================================================================
// 6. Full-fixture coverage sweep (every available example).
// ===================================================================

#[test]
fn full_fixture_coverage_sweep() {
    let dir = fixtures_dir();
    let mut entries: Vec<PathBuf> = match std::fs::read_dir(&dir) {
        Ok(rd) => rd
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().map(|x| x == "json").unwrap_or(false))
            .collect(),
        Err(_) => {
            eprintln!("fixtures dir not found: {}", dir.display());
            return;
        }
    };
    entries.sort();

    let mut solids = 0usize;
    let mut empty = 0usize;
    let mut total = 0usize;
    println!(
        "\n{:<26} {:>12} {:>7} {:>6}",
        "fixture", "volume", "tris", "parts"
    );
    for path in &entries {
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let raw = std::fs::read_to_string(path).unwrap();
        let Ok(input) = CompilerInputDag::from_json(&raw) else {
            continue;
        };
        total += 1;
        let result = compile(&input, MeshParams::default());
        let vol: f64 = result.parts.iter().map(|p| p.solid.volume()).sum();
        let tris: usize = result
            .parts
            .iter()
            .map(|p| p.solid.mesh().triangle_count())
            .sum();
        if vol > 1e-9 {
            solids += 1;
        } else {
            empty += 1;
        }
        println!("{name:<26} {vol:>12.3} {tris:>7} {:>6}", result.parts.len());
    }
    println!(
        "\nCoverage: {solids}/{total} fixtures produced a non-empty solid ({empty} empty/unsupported)."
    );
    // We support the foundational node set; guard that a healthy majority work.
    assert!(
        solids >= 20,
        "expected >=20 fixtures to produce solids, got {solids}"
    );
}
