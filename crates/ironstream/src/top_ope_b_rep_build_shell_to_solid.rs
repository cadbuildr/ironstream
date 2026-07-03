// FILE: top_ope_b_rep_build_shell_to_solid.rs
// occt: TopOpeBRepBuild_ShellToSolid

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildShellToSolid {
    shell_id: i32,
    solid_id: i32,
}

impl TopOpeBRepBuildShellToSolid {
    pub fn new() -> Self {
        TopOpeBRepBuildShellToSolid {
            shell_id: 0,
            solid_id: 0,
        }
    }

    pub fn with_ids(shell_id: i32, solid_id: i32) -> Self {
        TopOpeBRepBuildShellToSolid { shell_id, solid_id }
    }

    pub fn shell_id(&self) -> i32 {
        self.shell_id
    }

    pub fn solid_id(&self) -> i32 {
        self.solid_id
    }

    pub fn set_solid_id(&mut self, solid_id: i32) {
        self.solid_id = solid_id;
    }
}

impl Default for TopOpeBRepBuildShellToSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_to_solid_new() {
        let sts = TopOpeBRepBuildShellToSolid::new();
        assert_eq!(sts.shell_id(), 0);
        assert_eq!(sts.solid_id(), 0);
    }

    #[test]
    fn test_shell_to_solid_with_ids() {
        let sts = TopOpeBRepBuildShellToSolid::with_ids(7, 12);
        assert_eq!(sts.shell_id(), 7);
        assert_eq!(sts.solid_id(), 12);
    }

    #[test]
    fn test_shell_to_solid_set_solid_id() {
        let mut sts = TopOpeBRepBuildShellToSolid::new();
        sts.set_solid_id(30);
        assert_eq!(sts.solid_id(), 30);
    }
}
