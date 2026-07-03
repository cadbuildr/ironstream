# ironstream (kernel crate)

## Summary

The IronStream geometry kernel: a from-scratch, **zero-dependency** Rust
implementation whose public API mirrors OpenCascade's package boundary (`gp`,
`Geom`, `TopoDS`, `BRepBuilderAPI`, `BRepPrimAPI`, `BRepAlgoAPI`, `BRepMesh`).
Internally a tessellating B-rep with a BSP-tree CSG boolean engine.

Original, clean-room code: shares no code with OpenCascade/OCCT or any other CAD
kernel.

## Tags

cad, rust, kernel, geometry, opencascade

## Guidelines

- No third-party dependencies in this crate — keep it pure `std`.
- Every solid must stay a watertight, outward-oriented `TriMesh`.

## Dependencies

- Upstream: none (pure `std`).
