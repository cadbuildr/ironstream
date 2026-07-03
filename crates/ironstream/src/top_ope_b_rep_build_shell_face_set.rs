// FILE: top_ope_b_rep_build_shell_face_set.rs
// occt: TopOpeBRepBuild_ShellFaceSet

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildShellFaceSet {
    faces: Vec<i32>,
    current_index: usize,
}

impl TopOpeBRepBuildShellFaceSet {
    pub fn new() -> Self {
        TopOpeBRepBuildShellFaceSet {
            faces: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_face(&mut self, face_id: i32) {
        self.faces.push(face_id);
    }

    pub fn init_faces(&mut self) {
        self.current_index = 0;
    }

    pub fn more_faces(&self) -> bool {
        self.current_index < self.faces.len()
    }

    pub fn next_face(&mut self) {
        self.current_index += 1;
    }

    pub fn face(&self) -> Option<i32> {
        if self.current_index < self.faces.len() {
            Some(self.faces[self.current_index])
        } else {
            None
        }
    }

    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }
}

impl Default for TopOpeBRepBuildShellFaceSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_face_set_new() {
        let sfs = TopOpeBRepBuildShellFaceSet::new();
        assert_eq!(sfs.num_faces(), 0);
    }

    #[test]
    fn test_shell_face_set_add_face() {
        let mut sfs = TopOpeBRepBuildShellFaceSet::new();
        sfs.add_face(1);
        sfs.add_face(2);
        sfs.add_face(3);
        assert_eq!(sfs.num_faces(), 3);
    }

    #[test]
    fn test_shell_face_set_iteration() {
        let mut sfs = TopOpeBRepBuildShellFaceSet::new();
        sfs.add_face(10);
        sfs.add_face(20);

        sfs.init_faces();
        assert!(sfs.more_faces());
        assert_eq!(sfs.face(), Some(10));

        sfs.next_face();
        assert!(sfs.more_faces());
        assert_eq!(sfs.face(), Some(20));

        sfs.next_face();
        assert!(!sfs.more_faces());
    }

    #[test]
    fn test_shell_face_set_default() {
        let sfs = TopOpeBRepBuildShellFaceSet::default();
        assert_eq!(sfs.num_faces(), 0);
    }
}
