# IronStream ⇄ OpenCascade parity map

Goal: a faithful, exact-geometry reimplementation of OpenCascade's class
structure in Rust. This document enumerates the OCCT packages/classes and tracks
the IronStream module that mirrors each. Status legend:

- ✅ implemented (exact geometry, real algorithm)
- 🟡 partial (subset of methods / cases)
- ⛔ not yet started

This is the working checklist; it is updated as modules land. Honesty rule: a
node/operation is only marked ✅ when it is implemented with real geometry and
covered by a test — never by a stub or a swallowed error.

Class-inventory snapshot (`parity/parity.py`, OCCT @ 4f95ecaa3b): **3638 / 7084
classes mirrored (51.4%)** — FoundationClasses 92.0%, ModelingAlgorithms 92.5%,
ModelingData 94.8%, Visualization 72.6%, ApplicationFramework 49.0%,
DataExchange 15.0%, Deprecated 4.6%, Draw 0.0%. Modules that fail their ported
tests are quarantined under `parity/quarantine/` (see `parity/quarantine.txt`),
not counted, and queued for honest fixes.

## Foundation maths

| OCCT package / class | IronStream | Status |
|---|---|---|
| `gp_XYZ`, `gp_Pnt`, `gp_Vec`, `gp_Dir` | `gp::{Pnt,Vec3,Dir}` | ✅ |
| `gp_XY`, `gp_Pnt2d`, `gp_Vec2d`, `gp_Dir2d` | `gp2d::{Pnt2d,Vec2d,Dir2d}` | ✅ |
| `gp_Ax1`, `gp_Ax2`, `gp_Ax3` | `gp::{Ax1,Ax2,Ax3}` | ✅ |
| `gp_Ax2d`, `gp_Ax22d` | `gp2d::Ax2d` | ✅ |
| `gp_Trsf`, `gp_GTrsf` | `gp::Trsf` | ✅ |
| `gp_Trsf2d` | `gp2d::Trsf2d` | ✅ |
| `gp_Quaternion` | `gp::Quaternion` | ✅ |
| `gp_Lin`, `gp_Circ`, `gp_Elips`, `gp_Parab`, `gp_Hypr` | `gp::{Lin,Circ,Elips}` | 🟡 (no parab/hypr) |
| `gp_Pln`, `gp_Cylinder`, `gp_Cone`, `gp_Sphere`, `gp_Torus` | `gp::{Pln,Cylinder,Cone,Sphere,Torus}` | ✅ |
| `math_*` (Gauss, BFGS, Newton, matrix) | `mathx` | 🟡 |

## Geometry — curves & surfaces

| OCCT package / class | IronStream | Status |
|---|---|---|
| `Geom2d_Curve` (Line/Circle/Ellipse/BSpline/Bezier/Trimmed/Offset) | `geom2d::Curve2d` | 🟡 (line/circle/ellipse/bspline/trimmed) |
| `Geom_Curve` (Line/Circle/Ellipse/BSpline/Bezier/Trimmed) | `geom::Curve` | 🟡 |
| `Geom_Surface` (Plane/Cyl/Cone/Sphere/Torus/BSpline/Revolution/Extrusion) | `geom::Surface` | 🟡 |
| `Geom_BSplineCurve` / `Geom_BSplineSurface` | `bsplines::{BSplineCurve,BSplineSurface}` | ✅ (de Boor eval) |
| `Geom_BezierCurve` / `Geom_BezierSurface` | `bsplines::bezier_*` | 🟡 |
| `BSplCLib`, `BSplSLib` (knots, de Boor) | `bsplines` | ✅ |
| `GeomAPI_Interpolate` / `GeomAPI_PointsToBSpline` | `bsplines::interpolate` | 🟡 |
| `GeomAPI_ProjectPointOnCurve/Surf` | `geom::project_*` | 🟡 |
| `Geom2dAPI_InterCurveCurve`, `GeomAPI_IntSS`, `IntTools` | `intersect` | 🟡 (analytic pairs) |

## Topology — TopoDS / BRep

| OCCT package / class | IronStream | Status |
|---|---|---|
| `TopoDS_Vertex/Edge/Wire/Face/Shell/Solid/Compound` | `topods::*` | 🟡 (analytic-carrying rewrite in progress) |
| `BRep_Tool` (curve/surface/pcurve/range/point accessors) | `topods::brep_tool` | 🟡 |
| `TopExp`, `TopExp_Explorer`, `TopTools` maps | `topods::explore` | 🟡 |
| `BRepBuilderAPI_MakeVertex/Edge/Wire/Face/Shell/Solid` | `brep_builder_api` | 🟡 |
| `BRepBuilderAPI_Transform`, `_Copy`, `_Sewing` | `brep_builder_api` | 🟡 |
| `ShapeFix`, `ShapeAnalysis`, `ShapeUpgrade` | `shape_fix` | ⛔ |

## Modeling — primitives, sweeps, booleans, features

| OCCT package / class | IronStream | Status |
|---|---|---|
| `BRepPrimAPI_MakeBox/Cylinder/Cone/Sphere/Torus/Wedge` | `brep_prim_api` | 🟡 → exact rewrite |
| `BRepPrimAPI_MakePrism` (extrude) | `brep_prim_api::make_prism` | 🟡 → exact rewrite |
| `BRepPrimAPI_MakeRevol` (lathe) | `brep_prim_api::make_revol` | 🟡 → exact rewrite |
| `BRepOffsetAPI_ThruSections` (loft) | `brep_offset_api::thru_sections` | ⛔ |
| `BRepOffsetAPI_MakePipe` / `MakePipeShell` (sweep) | `brep_offset_api::make_pipe` | ⛔ |
| `BRepOffsetAPI_MakeThickSolid` (shell/hollow) | `brep_offset_api::make_thick_solid` | ⛔ |
| `BRepOffsetAPI_MakeOffset` / `Draft` | `brep_offset_api` | ⛔ |
| `BRepFilletAPI_MakeFillet` (edge fillet) | `brep_fillet_api::make_fillet` | ⛔ |
| `BRepFilletAPI_MakeChamfer` | `brep_fillet_api::make_chamfer` | ⛔ |
| `BRepAlgoAPI_Fuse/Cut/Common/Section` (BOPAlgo) | `brep_algo_api` | 🟡 (mesh CSG → exact B-Rep BOP in progress) |
| `BRepFeat_MakePrism/MakeDPrism` (pockets/pads) | `brep_feat` | ⛔ |

## Meshing & I/O

| OCCT package / class | IronStream | Status |
|---|---|---|
| `BRepMesh_IncrementalMesh` (face tessellation) | `brep_mesh` | 🟡 → surface-sampling rewrite |
| `Poly_Triangulation`, `Poly_Polygon3D` | `mesh::TriMesh` | ✅ |
| `StlAPI_Writer/Reader` | `mesh_io::stl` | ✅ |
| `STEPControl_Writer/Reader` (AP203/AP214) | `step_io` | 🟡 (faceted write) |
| `IGESControl`, `BRepTools` (BREP dump) | `brep_io` | ⛔ |
| `GProp_GProps` (volume/CoG/inertia) | `gprop` | 🟡 (volume/area) |

## Honesty note on "exact NURBS booleans"

OCCT's general boolean (`BOPAlgo`/`IntTools`) computes exact surface–surface
intersection curves for arbitrary NURBS and is one of the largest, most
battle-hardened parts of the kernel. IronStream's path to it:

1. Exact analytic geometry foundation (this is what the current increment
   builds): real `Geom` curves/surfaces with closed-form evaluation, real
   B-spline de Boor evaluation, and real topology that carries them.
2. Exact analytic intersections for the analytic surface pairs that dominate
   mechanical CAD (plane/plane, plane/cylinder, plane/sphere, cylinder/cylinder,
   …) driving a real B-Rep boolean (face splitting + classification + sewing).
3. General NURBS–NURBS intersection (marching / subdivision) — the frontier.

This is tracked above and not hidden behind stubs. The remaining ⛔/🟡 rows are
the honest statement of what is left.
