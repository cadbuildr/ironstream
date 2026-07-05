// FILE: draw_appli.rs
// occt: Draw_Appli

//! Draw application interface stub.

#[derive(Clone, Debug)]
pub struct DrawAppli;

impl DrawAppli {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) {
        // Stub for running the Draw interpreter
    }

    pub fn exit(&self) {
        // Stub for exiting the Draw application
    }
}

impl Default for DrawAppli {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let appli = DrawAppli::new();
        let _ = appli;
    }

    #[test]
    fn test_default() {
        let _appli = DrawAppli::default();
    }
}
