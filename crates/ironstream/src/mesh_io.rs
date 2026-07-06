//! Mesh I/O — binary STL and a faceted STEP (ISO-10303-21) writer.
//!
//! Binary STL is the primary, fully-supported export path. The STEP writer
//! emits a geometry-focused
//! `FACETED_BREP` so parts can round-trip into other CAD tools; it is a
//! faceted (tessellated) representation, not an analytic B-rep.

use crate::mesh::TriMesh;

/// Serialize a mesh to binary STL bytes.
// occt: StlAPI_Writer
pub fn write_binary_stl(mesh: &TriMesh) -> Vec<u8> {
    let m = mesh.welded(1e-7);
    let mut out = Vec::with_capacity(84 + m.tris.len() * 50);
    // 80-byte header.
    let header = b"IronStream binary STL";
    let mut head = [0u8; 80];
    head[..header.len()].copy_from_slice(header);
    out.extend_from_slice(&head);
    out.extend_from_slice(&(m.tris.len() as u32).to_le_bytes());
    for t in &m.tris {
        let a = m.verts[t[0]];
        let b = m.verts[t[1]];
        let c = m.verts[t[2]];
        let n = (b - a).cross(c - a).normalized();
        for comp in [n.x, n.y, n.z] {
            out.extend_from_slice(&(comp as f32).to_le_bytes());
        }
        for v in [a, b, c] {
            for comp in [v.x, v.y, v.z] {
                out.extend_from_slice(&(comp as f32).to_le_bytes());
            }
        }
        out.extend_from_slice(&0u16.to_le_bytes()); // attribute byte count
    }
    out
}

/// Serialize a mesh to ASCII STL (handy for debugging / diffing).
pub fn write_ascii_stl(mesh: &TriMesh, name: &str) -> String {
    let m = mesh.welded(1e-7);
    let mut s = String::new();
    s.push_str(&format!("solid {name}\n"));
    for t in &m.tris {
        let a = m.verts[t[0]];
        let b = m.verts[t[1]];
        let c = m.verts[t[2]];
        let n = (b - a).cross(c - a).normalized();
        s.push_str(&format!("  facet normal {} {} {}\n", n.x, n.y, n.z));
        s.push_str("    outer loop\n");
        for v in [a, b, c] {
            s.push_str(&format!("      vertex {} {} {}\n", v.x, v.y, v.z));
        }
        s.push_str("    endloop\n  endfacet\n");
    }
    s.push_str(&format!("endsolid {name}\n"));
    s
}

/// Serialize a mesh to a faceted STEP (`FACETED_BREP`) string.
// occt: STEPControl_Writer
pub fn write_step(mesh: &TriMesh, name: &str) -> String {
    let m = mesh.welded(1e-7);
    let mut e = StepEmitter::new();

    // Geometric/representation context boilerplate.
    let app_ctx = e.add("APPLICATION_CONTEXT('automotive design')");
    let _app_proto = e.add(&format!(
        "APPLICATION_PROTOCOL_DEFINITION('international standard','config_control_design',2010,#{app_ctx})"
    ));
    let dim = e.add("(NAMED_UNIT(*)SI_UNIT($,.METRE.)LENGTH_UNIT())");
    let angle = e.add("(NAMED_UNIT(*)PLANE_ANGLE_UNIT()SI_UNIT($,.RADIAN.))");
    let solid_angle = e.add("(NAMED_UNIT(*)SI_UNIT($,.STERADIAN.)SOLID_ANGLE_UNIT())");
    let uncert = e.add(&format!(
        "UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-06),#{dim},'distance','')"
    ));
    let ctx = e.add(&format!(
        "(GEOMETRIC_REPRESENTATION_CONTEXT(3)GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{uncert}))GLOBAL_UNIT_ASSIGNED_CONTEXT((#{dim},#{angle},#{solid_angle}))REPRESENTATION_CONTEXT('Context','3D'))"
    ));

    // One CARTESIAN_POINT per welded vertex.
    let point_ids: Vec<usize> = m
        .verts
        .iter()
        .map(|v| {
            e.add(&format!(
                "CARTESIAN_POINT('',({:.6},{:.6},{:.6}))",
                v.x, v.y, v.z
            ))
        })
        .collect();

    // One planar FACE per triangle, bounded by a POLY_LOOP.
    let mut face_ids = Vec::with_capacity(m.tris.len());
    for t in &m.tris {
        let loop_id = e.add(&format!(
            "POLY_LOOP('',(#{},#{},#{}))",
            point_ids[t[0]], point_ids[t[1]], point_ids[t[2]]
        ));
        let bound = e.add(&format!("FACE_OUTER_BOUND('',#{loop_id},.T.)"));
        let face = e.add(&format!("FACE_OUTER_BOUND_FACE('',(#{bound}))"));
        face_ids.push(face);
    }
    let shell_refs = face_ids
        .iter()
        .map(|id| format!("#{id}"))
        .collect::<Vec<_>>()
        .join(",");
    let shell = e.add(&format!("CLOSED_SHELL('',({shell_refs}))"));
    let brep = e.add(&format!("FACETED_BREP('{name}',#{shell})"));
    let _rep = e.add(&format!(
        "ADVANCED_BREP_SHAPE_REPRESENTATION('{name}',(#{brep}),#{ctx})"
    ));

    e.finish(name)
}

/// Tiny incremental ISO-10303-21 entity emitter.
pub(crate) struct StepEmitter {
    lines: Vec<String>,
    next: usize,
}

impl StepEmitter {
    pub(crate) fn new() -> Self {
        Self {
            lines: Vec::new(),
            next: 1,
        }
    }

    pub(crate) fn add(&mut self, body: &str) -> usize {
        let id = self.next;
        self.next += 1;
        self.lines.push(format!("#{id} = {body};"));
        id
    }

    pub(crate) fn finish(self, name: &str) -> String {
        let mut s = String::new();
        s.push_str("ISO-10303-21;\nHEADER;\n");
        s.push_str("FILE_DESCRIPTION(('IronStream faceted solid'),'2;1');\n");
        s.push_str(&format!(
            "FILE_NAME('{name}.step','',(''),(''),'IronStream','IronStream','');\n"
        ));
        s.push_str("FILE_SCHEMA(('CONFIG_CONTROL_DESIGN'));\nENDSEC;\nDATA;\n");
        for l in &self.lines {
            s.push_str(l);
            s.push('\n');
        }
        s.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep_prim_api::make_box;
    use crate::gp::Pnt;

    #[test]
    fn stl_roundtrip_size() {
        let s = make_box(Pnt::origin(), 1.0, 1.0, 1.0);
        let bytes = write_binary_stl(s.mesh());
        let tri_count = u32::from_le_bytes([bytes[80], bytes[81], bytes[82], bytes[83]]) as usize;
        assert_eq!(bytes.len(), 84 + tri_count * 50);
        assert_eq!(tri_count, 12);
    }

    #[test]
    fn step_emits_header_and_data() {
        let s = make_box(Pnt::origin(), 1.0, 1.0, 1.0);
        let step = write_step(s.mesh(), "box");
        assert!(step.starts_with("ISO-10303-21;"));
        assert!(step.contains("FACETED_BREP"));
        assert!(step.trim_end().ends_with("END-ISO-10303-21;"));
    }
}
