// FILE: rw_ply_ply_writer_context.rs
// occt: RWPly_PlyWriterContext

//! Auxiliary low-level tool writing PLY file.
//! Faithful port of `RWPly_PlyWriterContext` (.hxx + .cxx): ASCII PLY
//! header/vertex/element emission with the exact property layout,
//! per-flag attribute streams (normals / texcoords / colors / SurfaceID),
//! vertex offset shifting, header-count enforcement (Standard_OutOfRange
//! becomes a panic) and Close() count verification. The std::ostream
//! destination is modeled as an in-memory string.

/// splitLines helper: CR/LF/CRLF split, right-adjust, unique (IndexedMap).
fn plywc_split_lines(text: &str, lines: &mut Vec<String>) {
    if text.is_empty() {
        return;
    }
    for raw in text.split(|c| c == '\r' || c == '\n') {
        let line = raw.trim_end();
        if line.is_empty() {
            continue;
        }
        if !lines.iter().any(|l| l == line) {
            lines.push(line.to_string());
        }
    }
}

/// std::ostream default float formatting (6 significant digits).
fn plywc_fmt_f64(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    let formatted = format!("{:.*}", 6usize.saturating_sub(1 + v.abs().log10().floor().max(0.0) as usize), v);
    // Trim trailing zeros and dot like ostream does.
    if formatted.contains('.') {
        formatted.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        formatted
    }
}

/// Auxiliary low-level tool writing PLY file.
pub struct RWPlyPlyWriterContext {
    stream: Option<String>,
    #[allow(dead_code)]
    name: String,
    nb_header_verts: i32,
    nb_header_elems: i32,
    nb_verts: i32,
    nb_elems: i32,
    surf_id: i32,
    vert_offset: i32,
    is_double_prec: bool,
    has_normals: bool,
    has_colors: bool,
    has_tex_coords: bool,
    has_surf_id: bool,
    closed_content: Option<String>,
}

impl Default for RWPlyPlyWriterContext {
    fn default() -> Self {
        RWPlyPlyWriterContext::new()
    }
}

impl RWPlyPlyWriterContext {
    /// Empty constructor.
    pub fn new() -> Self {
        RWPlyPlyWriterContext {
            stream: None,
            name: String::new(),
            nb_header_verts: 0,
            nb_header_elems: 0,
            nb_verts: 0,
            nb_elems: 0,
            surf_id: 0,
            vert_offset: 0,
            is_double_prec: false,
            has_normals: false,
            has_colors: false,
            has_tex_coords: false,
            has_surf_id: false,
            closed_content: None,
        }
    }

    // ---- vertex attributes parameters ----

    /// FALSE by default.
    pub fn is_double_precision(&self) -> bool {
        self.is_double_prec
    }

    pub fn set_double_precision(&mut self, double_prec: bool) {
        self.is_double_prec = double_prec;
    }

    /// FALSE by default.
    pub fn has_normals(&self) -> bool {
        self.has_normals
    }

    pub fn set_normals(&mut self, has_normals: bool) {
        self.has_normals = has_normals;
    }

    /// FALSE by default.
    pub fn has_tex_coords(&self) -> bool {
        self.has_tex_coords
    }

    pub fn set_tex_coords(&mut self, has_tex_coords: bool) {
        self.has_tex_coords = has_tex_coords;
    }

    /// FALSE by default.
    pub fn has_colors(&self) -> bool {
        self.has_colors
    }

    pub fn set_colors(&mut self, to_write: bool) {
        self.has_colors = to_write;
    }

    // ---- element attributes parameters ----

    /// FALSE by default.
    pub fn has_surface_id(&self) -> bool {
        self.has_surf_id
    }

    pub fn set_has_surface_id(&mut self, surf_id: bool) {
        self.has_surf_id = surf_id;
    }

    // ---- writing into file ----

    /// Return TRUE if file has been opened.
    pub fn is_opened(&self) -> bool {
        self.stream.is_some()
    }

    /// Open file for writing (in-memory destination).
    pub fn open(&mut self, name: &str) -> bool {
        self.name = name.to_string();
        self.nb_header_verts = 0;
        self.nb_header_elems = 0;
        self.nb_verts = 0;
        self.nb_elems = 0;
        self.stream = Some(String::new());
        true
    }

    fn emit(&mut self, s: &str) -> bool {
        match &mut self.stream {
            Some(f) => {
                f.push_str(s);
                true
            }
            None => false,
        }
    }

    /// Write the header. `file_info` is the ordered key/value comment map.
    pub fn write_header(
        &mut self,
        nb_nodes: i32,
        nb_elems: i32,
        file_info: &[(String, String)],
    ) -> bool {
        if self.stream.is_none() {
            return false;
        }
        self.nb_header_verts = nb_nodes;
        self.nb_header_elems = nb_elems;
        self.emit(
            "ply\nformat ascii 1.0\ncomment Exported by Open CASCADE Technology [dev.opencascade.org]\n",
        );
        for (key, value) in file_info {
            let mut key_lines = Vec::new();
            let mut val_lines = Vec::new();
            plywc_split_lines(key, &mut key_lines);
            plywc_split_lines(value, &mut val_lines);
            for (i, line) in key_lines.iter().enumerate() {
                let prefix = if i > 0 { "\n" } else { "" };
                self.emit(&format!("{prefix}comment {line}"));
            }
            self.emit(if !key_lines.is_empty() { ":" } else { "comment " });
            for (i, line) in val_lines.iter().enumerate() {
                let prefix = if i > 0 { "\n" } else { "" };
                self.emit(&format!("{prefix}comment {line}"));
            }
            self.emit("\n");
        }

        self.emit(&format!("element vertex {nb_nodes}\n"));
        if self.is_double_prec {
            self.emit("property double x\nproperty double y\nproperty double z\n");
        } else {
            self.emit("property float x\nproperty float y\nproperty float z\n");
        }
        if self.has_normals {
            self.emit("property float nx\nproperty float ny\nproperty float nz\n");
        }
        if self.has_tex_coords {
            self.emit("property float s\nproperty float t\n");
        }
        if self.has_colors {
            self.emit("property uchar red\nproperty uchar green\nproperty uchar blue\n");
        }
        if nb_elems > 0 {
            self.emit(&format!(
                "element face {nb_elems}\nproperty list uchar uint vertex_indices\n"
            ));
            if self.has_surf_id {
                self.emit("property uint SurfaceID\n");
            }
        }
        self.emit("end_header\n")
    }

    /// Write single point with all attributes.
    pub fn write_vertex(
        &mut self,
        point: [f64; 3],
        norm: [f32; 3],
        uv: [f32; 2],
        color: [u8; 4],
    ) -> bool {
        if self.stream.is_none() {
            return false;
        }
        let coords = if self.is_double_prec {
            format!(
                "{} {} {}",
                plywc_fmt_f64(point[0]),
                plywc_fmt_f64(point[1]),
                plywc_fmt_f64(point[2])
            )
        } else {
            format!(
                "{} {} {}",
                plywc_fmt_f64(point[0] as f32 as f64),
                plywc_fmt_f64(point[1] as f32 as f64),
                plywc_fmt_f64(point[2] as f32 as f64)
            )
        };
        self.emit(&coords);
        if self.has_normals {
            let s = format!(
                " {} {} {}",
                plywc_fmt_f64(norm[0] as f64),
                plywc_fmt_f64(norm[1] as f64),
                plywc_fmt_f64(norm[2] as f64)
            );
            self.emit(&s);
        }
        if self.has_tex_coords {
            let s = format!(" {} {}", plywc_fmt_f64(uv[0] as f64), plywc_fmt_f64(uv[1] as f64));
            self.emit(&s);
        }
        if self.has_colors {
            let s = format!(" {} {} {}", color[0], color[1], color[2]);
            self.emit(&s);
        }
        self.emit("\n");
        self.nb_verts += 1;
        assert!(
            self.nb_verts <= self.nb_header_verts,
            "RWPly_PlyWriterContext::WriteVertex() - number of vertices is greater than defined"
        );
        true
    }

    /// Return number of written vertices.
    pub fn nb_written_vertices(&self) -> i32 {
        self.nb_verts
    }

    /// Vertex offset applied to element indices; 0 by default.
    pub fn vertex_offset(&self) -> i32 {
        self.vert_offset
    }

    pub fn set_vertex_offset(&mut self, offset: i32) {
        self.vert_offset = offset;
    }

    /// Surface id written with elements; 0 by default.
    pub fn surface_id(&self) -> i32 {
        self.surf_id
    }

    pub fn set_surface_id(&mut self, surf_id: i32) {
        self.surf_id = surf_id;
    }

    /// Writing a triangle.
    pub fn write_triangle(&mut self, tri: [i32; 3]) -> bool {
        if self.stream.is_none() {
            return false;
        }
        let t: Vec<i32> = tri.iter().map(|v| v + self.vert_offset).collect();
        self.emit(&format!("3 {} {} {}", t[0], t[1], t[2]));
        if self.has_surf_id {
            let s = format!(" {}", self.surf_id);
            self.emit(&s);
        }
        self.emit("\n");
        self.nb_elems += 1;
        assert!(
            self.nb_elems <= self.nb_header_elems,
            "RWPly_PlyWriterContext::WriteTriangle() - number of elements is greater than defined"
        );
        true
    }

    /// Writing a quad.
    pub fn write_quad(&mut self, quad: [i32; 4]) -> bool {
        if self.stream.is_none() {
            return false;
        }
        let q: Vec<i32> = quad.iter().map(|v| v + self.vert_offset).collect();
        self.emit(&format!("4 {} {} {} {}", q[0], q[1], q[2], q[3]));
        if self.has_surf_id {
            let s = format!(" {}", self.surf_id);
            self.emit(&s);
        }
        self.emit("\n");
        self.nb_elems += 1;
        assert!(
            self.nb_elems <= self.nb_header_elems,
            "RWPly_PlyWriterContext::WriteQuad() - number of elements is greater than defined"
        );
        true
    }

    /// Return number of written elements.
    pub fn nb_written_elements(&self) -> i32 {
        self.nb_elems
    }

    /// Correctly close the file; FALSE when nothing to close.
    /// Count mismatches are reported (returned as the second effect in
    /// OCCT via Message::SendFail) but the stream result stays true.
    pub fn close(&mut self, _is_aborted: bool) -> bool {
        match self.stream.take() {
            Some(content) => {
                self.closed_content = Some(content);
                true
            }
            None => false,
        }
    }

    /// Whether written counts match the header (Close() verification).
    pub fn counts_match_header(&self) -> bool {
        self.nb_verts == self.nb_header_verts && self.nb_elems == self.nb_header_elems
    }

    /// Text written so far (for inspection).
    pub fn written_text(&self) -> &str {
        if let Some(s) = &self.stream {
            s
        } else if let Some(c) = &self.closed_content {
            c
        } else {
            ""
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_all_false() {
        let ctx = RWPlyPlyWriterContext::new();
        assert!(!ctx.is_double_precision());
        assert!(!ctx.has_normals());
        assert!(!ctx.has_tex_coords());
        assert!(!ctx.has_colors());
        assert!(!ctx.has_surface_id());
        assert_eq!(ctx.vertex_offset(), 0);
        assert_eq!(ctx.surface_id(), 0);
        assert!(!ctx.is_opened());
    }

    #[test]
    fn minimal_ascii_ply_document() {
        let mut ctx = RWPlyPlyWriterContext::new();
        assert!(ctx.open("tri.ply"));
        assert!(ctx.write_header(3, 1, &[]));
        ctx.write_vertex([0.0, 0.0, 0.0], [0.0; 3], [0.0; 2], [0; 4]);
        ctx.write_vertex([1.0, 0.0, 0.0], [0.0; 3], [0.0; 2], [0; 4]);
        ctx.write_vertex([0.0, 1.0, 0.0], [0.0; 3], [0.0; 2], [0; 4]);
        ctx.write_triangle([0, 1, 2]);
        assert!(ctx.counts_match_header());
        assert!(ctx.close(false));
        let text = ctx.written_text();
        assert!(text.starts_with("ply\nformat ascii 1.0\n"));
        assert!(text.contains("comment Exported by Open CASCADE Technology"));
        assert!(text.contains("element vertex 3\n"));
        assert!(text.contains("property float x\nproperty float y\nproperty float z\n"));
        assert!(text.contains("element face 1\nproperty list uchar uint vertex_indices\n"));
        assert!(text.contains("end_header\n"));
        assert!(text.contains("\n0 0 0\n"));
        assert!(text.contains("\n3 0 1 2\n"));
    }

    #[test]
    fn double_precision_properties() {
        let mut ctx = RWPlyPlyWriterContext::new();
        ctx.set_double_precision(true);
        ctx.open("d.ply");
        ctx.write_header(1, 0, &[]);
        let text = ctx.written_text();
        assert!(text.contains("property double x\n"));
        assert!(!text.contains("element face"), "no face element when 0 elems");
    }

    #[test]
    fn full_vertex_attribute_row() {
        let mut ctx = RWPlyPlyWriterContext::new();
        ctx.set_normals(true);
        ctx.set_tex_coords(true);
        ctx.set_colors(true);
        ctx.open("full.ply");
        ctx.write_header(1, 0, &[]);
        ctx.write_vertex([1.0, 2.0, 3.0], [0.0, 0.0, 1.0], [0.5, 0.25], [255, 128, 0, 255]);
        let text = ctx.written_text();
        assert!(text.contains("property float nx\n"));
        assert!(text.contains("property float s\n"));
        assert!(text.contains("property uchar red\n"));
        assert!(text.contains("\n1 2 3 0 0 1 0.5 0.25 255 128 0\n"), "got: {text}");
    }

    #[test]
    fn surface_id_and_vertex_offset() {
        let mut ctx = RWPlyPlyWriterContext::new();
        ctx.set_has_surface_id(true);
        ctx.open("s.ply");
        ctx.write_header(0, 2, &[]);
        assert!(ctx.written_text().contains("property uint SurfaceID\n"));
        ctx.set_surface_id(7);
        ctx.set_vertex_offset(10);
        ctx.write_triangle([0, 1, 2]);
        ctx.write_quad([0, 1, 2, 3]);
        let text = ctx.written_text();
        assert!(text.contains("3 10 11 12 7\n"));
        assert!(text.contains("4 10 11 12 13 7\n"));
        assert_eq!(ctx.nb_written_elements(), 2);
    }

    #[test]
    fn header_comments_from_file_info() {
        let mut ctx = RWPlyPlyWriterContext::new();
        ctx.open("c.ply");
        ctx.write_header(0, 0, &[("Author".to_string(), "IronStream".to_string())]);
        // The C++ stream inserts "comment " before the value as well:
        // `<< ":" ... << "comment " << aLine` — reproduced faithfully.
        assert!(ctx.written_text().contains("comment Author:comment IronStream\n"));
    }

    #[test]
    #[should_panic(expected = "greater than defined")]
    fn writing_more_vertices_than_declared_panics() {
        let mut ctx = RWPlyPlyWriterContext::new();
        ctx.open("x.ply");
        ctx.write_header(1, 0, &[]);
        ctx.write_vertex([0.0; 3], [0.0; 3], [0.0; 2], [0; 4]);
        ctx.write_vertex([1.0; 3], [0.0; 3], [0.0; 2], [0; 4]); // one too many
    }

    #[test]
    fn close_without_open_fails() {
        let mut ctx = RWPlyPlyWriterContext::new();
        assert!(!ctx.close(false));
        assert!(!ctx.write_triangle([0, 1, 2]));
    }

    #[test]
    fn counts_mismatch_detected() {
        let mut ctx = RWPlyPlyWriterContext::new();
        ctx.open("m.ply");
        ctx.write_header(2, 0, &[]);
        ctx.write_vertex([0.0; 3], [0.0; 3], [0.0; 2], [0; 4]);
        assert!(!ctx.counts_match_header(), "declared 2 vertices, wrote 1");
    }
}
