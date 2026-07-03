// FILE: topo_solid.rs
// occt: TopoDS_Shell, TopoDS_Solid, TopoDS_CompSolid, TopoDS_Compound

/// TopoDS_Shell: a set of faces connected along their edges.
/// occt: TopoDS_Shell
#[derive(Clone, Debug, PartialEq)]
pub struct TopoShell {
    pub shape_id: u32,
    pub face_ids: Vec<u32>,
    pub is_closed: bool,
    pub is_null: bool,
}

impl Default for TopoShell {
    fn default() -> Self { Self { shape_id: 0, face_ids: Vec::new(), is_closed: false, is_null: true } }
}

impl TopoShell {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, face_ids: Vec::new(), is_closed: false, is_null: false }
    }

    pub fn is_null(&self) -> bool { self.is_null }
    pub fn is_closed(&self) -> bool { self.is_closed }
    pub fn shape_id(&self) -> u32 { self.shape_id }
    pub fn nb_faces(&self) -> usize { self.face_ids.len() }

    pub fn add_face(&mut self, face_id: u32) { self.face_ids.push(face_id); }
    pub fn set_closed(&mut self, v: bool) { self.is_closed = v; }
}

/// TopoDS_Solid: a closed shell that bounds a volume.
/// occt: TopoDS_Solid
#[derive(Clone, Debug, PartialEq)]
pub struct TopoSolid {
    pub shape_id: u32,
    pub shell_ids: Vec<u32>,
    pub volume_hint: f64,   // cached volume (0 = unknown)
    pub is_null: bool,
}

impl Default for TopoSolid {
    fn default() -> Self { Self { shape_id: 0, shell_ids: Vec::new(), volume_hint: 0.0, is_null: true } }
}

impl TopoSolid {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, shell_ids: Vec::new(), volume_hint: 0.0, is_null: false }
    }

    pub fn is_null(&self) -> bool { self.is_null }
    pub fn shape_id(&self) -> u32 { self.shape_id }
    pub fn nb_shells(&self) -> usize { self.shell_ids.len() }
    pub fn volume_hint(&self) -> f64 { self.volume_hint }

    pub fn add_shell(&mut self, shell_id: u32) { self.shell_ids.push(shell_id); }
    pub fn set_volume_hint(&mut self, v: f64) { self.volume_hint = v; }
}

/// TopoDS_CompSolid: a set of solids connected along faces.
/// occt: TopoDS_CompSolid
#[derive(Clone, Debug, PartialEq)]
pub struct TopoCompSolid {
    pub shape_id: u32,
    pub solid_ids: Vec<u32>,
    pub is_null: bool,
}

impl Default for TopoCompSolid {
    fn default() -> Self { Self { shape_id: 0, solid_ids: Vec::new(), is_null: true } }
}

impl TopoCompSolid {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, solid_ids: Vec::new(), is_null: false }
    }

    pub fn is_null(&self) -> bool { self.is_null }
    pub fn shape_id(&self) -> u32 { self.shape_id }
    pub fn nb_solids(&self) -> usize { self.solid_ids.len() }
    pub fn add_solid(&mut self, solid_id: u32) { self.solid_ids.push(solid_id); }
}

/// TopoDS_Compound: a heterogeneous collection of shapes.
/// occt: TopoDS_Compound
#[derive(Clone, Debug, PartialEq)]
pub struct TopoCompound {
    pub shape_id: u32,
    pub child_ids: Vec<u32>,
    pub is_null: bool,
}

impl Default for TopoCompound {
    fn default() -> Self { Self { shape_id: 0, child_ids: Vec::new(), is_null: true } }
}

impl TopoCompound {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, child_ids: Vec::new(), is_null: false }
    }

    pub fn is_null(&self) -> bool { self.is_null }
    pub fn shape_id(&self) -> u32 { self.shape_id }
    pub fn nb_children(&self) -> usize { self.child_ids.len() }

    pub fn add(&mut self, shape_id: u32) {
        if shape_id > 0 && !self.child_ids.contains(&shape_id) {
            self.child_ids.push(shape_id);
        }
    }

    pub fn remove(&mut self, shape_id: u32) {
        self.child_ids.retain(|&id| id != shape_id);
    }

    pub fn is_empty(&self) -> bool { self.child_ids.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_add_face() {
        let mut s = TopoShell::new(1);
        assert!(!s.is_null());
        s.add_face(10);
        s.add_face(20);
        assert_eq!(s.nb_faces(), 2);
        s.set_closed(true);
        assert!(s.is_closed());
    }

    #[test]
    fn solid_add_shell() {
        let mut s = TopoSolid::new(1);
        s.add_shell(5);
        s.set_volume_hint(1234.5);
        assert_eq!(s.nb_shells(), 1);
        assert!((s.volume_hint() - 1234.5).abs() < 1e-6);
    }

    #[test]
    fn comp_solid_add_solid() {
        let mut cs = TopoCompSolid::new(1);
        cs.add_solid(10);
        cs.add_solid(20);
        assert_eq!(cs.nb_solids(), 2);
    }

    #[test]
    fn compound_add_remove() {
        let mut c = TopoCompound::new(1);
        c.add(10);
        c.add(20);
        c.add(10); // duplicate ignored
        assert_eq!(c.nb_children(), 2);
        c.remove(10);
        assert_eq!(c.nb_children(), 1);
    }

    #[test]
    fn compound_defaults_null() {
        let c = TopoCompound::default();
        assert!(c.is_null());
        assert!(c.is_empty());
    }
}
