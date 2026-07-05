// FILE: rw_obj_obj_writer_context.rs
// occt: RWObj_ObjWriterContext

//! Auxiliary low-level tool writing OBJ file.
//! Faithful port of `RWObj_ObjWriterContext` (.hxx + .cxx). The FILE*
//! destination is modeled as an in-memory byte buffer so the emitted OBJ
//! text is directly testable; all statement formats ("v %f %f %f",
//! "f %d/%d/%d ...", index shifting via FlushFace, header comments and
//! mtllib emission) follow the C++ implementation exactly.

/// splitLines from RWObj_ObjWriterContext.cxx: splits on CR/LF/CRLF,
/// right-adjusts each line, skips empty lines, keeps unique lines in
/// insertion order (NCollection_IndexedMap semantics).
fn owc_split_lines(text: &str, lines: &mut Vec<String>) {
    if text.is_empty() {
        return;
    }
    for raw in text.split(|c| c == '\r' || c == '\n') {
        let line = raw.trim_end(); // RightAdjust
        if line.is_empty() {
            continue;
        }
        if !lines.iter().any(|l| l == line) {
            lines.push(line.to_string());
        }
    }
}

/// Auxiliary low-level tool writing OBJ file.
pub struct RWObjObjWriterContext {
    /// Public counter of written faces (`NbFaces` field).
    pub nb_faces: i32,
    /// In-memory stand-in for the FILE* destination; None = closed.
    file: Option<String>,
    #[allow(dead_code)]
    name: String,
    active_material: String,
    elem_pos_first: [i32; 4],
    elem_norm_first: [i32; 4],
    elem_uv_first: [i32; 4],
    has_normals: bool,
    has_tex_coords: bool,
    /// Text kept after Close() so tests can inspect the document.
    closed_content: Option<String>,
}

impl RWObjObjWriterContext {
    /// Main constructor (opens the in-memory destination).
    pub fn new(name: &str) -> Self {
        RWObjObjWriterContext {
            nb_faces: 0,
            file: Some(String::new()),
            name: name.to_string(),
            active_material: String::new(),
            elem_pos_first: [1, 1, 1, 1],
            elem_norm_first: [1, 1, 1, 1],
            elem_uv_first: [1, 1, 1, 1],
            has_normals: false,
            has_tex_coords: false,
            closed_content: None,
        }
    }

    /// Return true if file has been opened.
    pub fn is_opened(&self) -> bool {
        self.file.is_some()
    }

    /// Correctly close the file.
    pub fn close(&mut self) -> bool {
        match self.file.take() {
            Some(content) => {
                self.closed_content = Some(content);
                true
            }
            None => false,
        }
    }

    /// Content written so far (post-Close inspection helper).
    pub fn written_text(&self) -> &str {
        if let Some(f) = &self.file {
            f
        } else if let Some(c) = &self.closed_content {
            c
        } else {
            ""
        }
    }

    /// Return true if normals are defined.
    pub fn has_normals(&self) -> bool {
        self.has_normals
    }

    /// Set if normals are defined.
    pub fn set_normals(&mut self, has_normals: bool) {
        self.has_normals = has_normals;
    }

    /// Return true if texture coordinates are defined.
    pub fn has_tex_coords(&self) -> bool {
        self.has_tex_coords
    }

    /// Set if texture coordinates are defined.
    pub fn set_tex_coords(&mut self, has_tex_coords: bool) {
        self.has_tex_coords = has_tex_coords;
    }

    fn emit(&mut self, s: &str) -> bool {
        match &mut self.file {
            Some(f) => {
                f.push_str(s);
                true
            }
            None => false,
        }
    }

    /// Write the header. `file_info` is the ordered key/value metadata map.
    pub fn write_header(
        &mut self,
        nb_nodes: i32,
        nb_elems: i32,
        mat_lib: &str,
        file_info: &[(String, String)],
    ) -> bool {
        let mut is_ok = self.emit(&format!(
            "# Exported by Open CASCADE Technology [dev.opencascade.org]\n#  Vertices: {}\n#     Faces: {}\n",
            nb_nodes, nb_elems
        ));
        for (key, value) in file_info {
            let mut key_lines = Vec::new();
            let mut val_lines = Vec::new();
            owc_split_lines(key, &mut key_lines);
            owc_split_lines(value, &mut val_lines);
            for (i, line) in key_lines.iter().enumerate() {
                let s = if i > 0 { format!("\n# {line}") } else { format!("# {line}") };
                is_ok = is_ok && self.emit(&s);
            }
            is_ok = is_ok && self.emit(if !key_lines.is_empty() { ":" } else { "# " });
            for (i, line) in val_lines.iter().enumerate() {
                let s = if i > 0 { format!("\n# {line}") } else { format!(" {line}") };
                is_ok = is_ok && self.emit(&s);
            }
            is_ok = is_ok && self.emit("\n");
        }
        if !mat_lib.is_empty() {
            is_ok = is_ok && self.emit(&format!("mtllib {mat_lib}\n"));
        }
        is_ok
    }

    /// Return active material or empty string if not set.
    pub fn active_material(&self) -> &str {
        &self.active_material
    }

    /// Set active material and write the `usemtl` statement.
    pub fn write_active_material(&mut self, material: &str) -> bool {
        self.active_material = material.to_string();
        if !material.is_empty() {
            self.emit(&format!("usemtl {material}\n"))
        } else {
            self.emit("usemtl\n")
        }
    }

    /// Writing a triangle (0-based node indices, shifted by the running
    /// first-element offsets).
    pub fn write_triangle(&mut self, tri: [i32; 3]) -> bool {
        let p: Vec<i32> = (0..3).map(|i| tri[i] + self.elem_pos_first[i]).collect();
        if self.has_normals {
            let n: Vec<i32> = (0..3).map(|i| tri[i] + self.elem_norm_first[i]).collect();
            if self.has_tex_coords {
                let t: Vec<i32> = (0..3).map(|i| tri[i] + self.elem_uv_first[i]).collect();
                return self.emit(&format!(
                    "f {}/{}/{} {}/{}/{} {}/{}/{}\n",
                    p[0], t[0], n[0], p[1], t[1], n[1], p[2], t[2], n[2]
                ));
            }
            return self.emit(&format!(
                "f {}//{} {}//{} {}//{}\n",
                p[0], n[0], p[1], n[1], p[2], n[2]
            ));
        }
        if self.has_tex_coords {
            let t: Vec<i32> = (0..3).map(|i| tri[i] + self.elem_uv_first[i]).collect();
            self.emit(&format!(
                "f {}/{} {}/{} {}/{}\n",
                p[0], t[0], p[1], t[1], p[2], t[2]
            ))
        } else {
            self.emit(&format!("f {} {} {}\n", p[0], p[1], p[2]))
        }
    }

    /// Writing a quad.
    pub fn write_quad(&mut self, quad: [i32; 4]) -> bool {
        let p: Vec<i32> = (0..4).map(|i| quad[i] + self.elem_pos_first[i]).collect();
        if self.has_normals {
            let n: Vec<i32> = (0..4).map(|i| quad[i] + self.elem_norm_first[i]).collect();
            if self.has_tex_coords {
                let t: Vec<i32> = (0..4).map(|i| quad[i] + self.elem_uv_first[i]).collect();
                return self.emit(&format!(
                    "f {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{}\n",
                    p[0], t[0], n[0], p[1], t[1], n[1], p[2], t[2], n[2], p[3], t[3], n[3]
                ));
            }
            return self.emit(&format!(
                "f {}//{} {}//{} {}//{} {}//{}\n",
                p[0], n[0], p[1], n[1], p[2], n[2], p[3], n[3]
            ));
        }
        if self.has_tex_coords {
            let t: Vec<i32> = (0..4).map(|i| quad[i] + self.elem_uv_first[i]).collect();
            self.emit(&format!(
                "f {}/{} {}/{} {}/{} {}/{}\n",
                p[0], t[0], p[1], t[1], p[2], t[2], p[3], t[3]
            ))
        } else {
            self.emit(&format!("f {} {} {} {}\n", p[0], p[1], p[2], p[3]))
        }
    }

    /// Writing a vertex ("v %f %f %f").
    pub fn write_vertex(&mut self, v: [f32; 3]) -> bool {
        self.emit(&format!("v {:.6} {:.6} {:.6}\n", v[0], v[1], v[2]))
    }

    /// Writing a normal ("vn %f %f %f").
    pub fn write_normal(&mut self, v: [f32; 3]) -> bool {
        self.emit(&format!("vn {:.6} {:.6} {:.6}\n", v[0], v[1], v[2]))
    }

    /// Writing a texture coordinate ("vt %f %f").
    pub fn write_tex_coord(&mut self, v: [f32; 2]) -> bool {
        self.emit(&format!("vt {:.6} {:.6}\n", v[0], v[1]))
    }

    /// Writing a group name.
    pub fn write_group(&mut self, value: &str) -> bool {
        if !value.is_empty() {
            self.emit(&format!("g {value}\n"))
        } else {
            self.emit("g\n")
        }
    }

    /// Increment indices shift after finishing a face set of nb_nodes nodes.
    pub fn flush_face(&mut self, nb_nodes: i32) {
        for i in 0..4 {
            self.elem_pos_first[i] += nb_nodes;
        }
        if self.has_normals {
            for i in 0..4 {
                self.elem_norm_first[i] += nb_nodes;
            }
        }
        if self.has_tex_coords {
            for i in 0..4 {
                self.elem_uv_first[i] += nb_nodes;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_and_mtllib() {
        let mut ctx = RWObjObjWriterContext::new("out.obj");
        assert!(ctx.is_opened());
        let info = vec![("Author".to_string(), "IronStream".to_string())];
        assert!(ctx.write_header(3, 1, "scene.mtl", &info));
        let text = ctx.written_text();
        assert!(text.starts_with("# Exported by Open CASCADE Technology [dev.opencascade.org]\n"));
        assert!(text.contains("#  Vertices: 3\n"));
        assert!(text.contains("#     Faces: 1\n"));
        assert!(text.contains("# Author: IronStream\n"));
        assert!(text.contains("mtllib scene.mtl\n"));
    }

    #[test]
    fn simple_triangle_document() {
        let mut ctx = RWObjObjWriterContext::new("tri.obj");
        ctx.write_vertex([0.0, 0.0, 0.0]);
        ctx.write_vertex([1.0, 0.0, 0.0]);
        ctx.write_vertex([0.0, 1.0, 0.0]);
        // 0-based triangle indices; 1-based in the file via the +1 shift.
        ctx.write_triangle([0, 1, 2]);
        ctx.nb_faces += 1;
        assert!(ctx.close());
        assert!(!ctx.is_opened());
        let text = ctx.written_text();
        assert!(text.contains("v 0.000000 0.000000 0.000000\n"));
        assert!(text.contains("f 1 2 3\n"));
        assert_eq!(ctx.nb_faces, 1);
    }

    #[test]
    fn triangle_with_normals_and_uv_format() {
        let mut ctx = RWObjObjWriterContext::new("full.obj");
        ctx.set_normals(true);
        ctx.set_tex_coords(true);
        ctx.write_triangle([0, 1, 2]);
        assert!(ctx.written_text().contains("f 1/1/1 2/2/2 3/3/3\n"));
    }

    #[test]
    fn normals_only_uses_double_slash() {
        let mut ctx = RWObjObjWriterContext::new("n.obj");
        ctx.set_normals(true);
        ctx.write_triangle([0, 1, 2]);
        assert!(ctx.written_text().contains("f 1//1 2//2 3//3\n"));
    }

    #[test]
    fn flush_face_shifts_indices_of_next_mesh() {
        let mut ctx = RWObjObjWriterContext::new("multi.obj");
        ctx.write_triangle([0, 1, 2]);
        ctx.flush_face(3); // first mesh had 3 nodes
        ctx.write_triangle([0, 1, 2]);
        let text = ctx.written_text();
        assert!(text.contains("f 1 2 3\n"));
        assert!(text.contains("f 4 5 6\n"), "second mesh starts at index 4: {text}");
    }

    #[test]
    fn quad_and_group_and_material() {
        let mut ctx = RWObjObjWriterContext::new("q.obj");
        ctx.write_group("body");
        ctx.write_active_material("steel");
        assert_eq!(ctx.active_material(), "steel");
        ctx.write_quad([0, 1, 2, 3]);
        let text = ctx.written_text();
        assert!(text.contains("g body\n"));
        assert!(text.contains("usemtl steel\n"));
        assert!(text.contains("f 1 2 3 4\n"));
        // Empty group/material forms.
        let mut ctx2 = RWObjObjWriterContext::new("e.obj");
        ctx2.write_group("");
        ctx2.write_active_material("");
        assert!(ctx2.written_text().contains("g\n"));
        assert!(ctx2.written_text().contains("usemtl\n"));
    }

    #[test]
    fn writes_fail_after_close() {
        let mut ctx = RWObjObjWriterContext::new("closed.obj");
        assert!(ctx.close());
        assert!(!ctx.close(), "double close fails");
        assert!(!ctx.write_vertex([0.0, 0.0, 0.0]));
        assert!(!ctx.write_group("x"));
    }

    #[test]
    fn multiline_file_info_is_commented_per_line() {
        let mut ctx = RWObjObjWriterContext::new("info.obj");
        let info = vec![("Note".to_string(), "line1\r\nline2".to_string())];
        ctx.write_header(0, 0, "", &info);
        let text = ctx.written_text();
        assert!(text.contains("# Note: line1\n# line2\n"), "got: {text}");
        assert!(!text.contains("mtllib"));
    }
}
