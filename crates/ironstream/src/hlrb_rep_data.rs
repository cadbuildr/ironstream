// FILE: hlrb_rep_data.rs
// occt: HLRBRep_Data

pub struct HlrbrépData {
    nb_vertices: i32,
    nb_edges: i32,
    nb_faces: i32,
    edges: Vec<i32>,
    faces: Vec<i32>,
}

impl HlrbrépData {
    pub fn new() -> Self {
        HlrbrépData {
            nb_vertices: 0,
            nb_edges: 0,
            nb_faces: 0,
            edges: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn nb_vertices(&self) -> i32 { self.nb_vertices }
    pub fn nb_edges(&self) -> i32 { self.nb_edges }
    pub fn nb_faces(&self) -> i32 { self.nb_faces }
    pub fn set_nb_vertices(&mut self, n: i32) { self.nb_vertices = n; }
    pub fn set_nb_edges(&mut self, n: i32) { self.nb_edges = n; }
    pub fn set_nb_faces(&mut self, n: i32) { self.nb_faces = n; }
}

impl Default for HlrbrépData {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let d = HlrbrépData::new();
        assert_eq!(d.nb_vertices(), 0);
    }
}
