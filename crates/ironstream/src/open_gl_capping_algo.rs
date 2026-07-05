// FILE: open_gl_capping_algo.rs
// occt: OpenGl_CappingAlgo

pub struct OpenGlCappingAlgo;

impl OpenGlCappingAlgo {
    pub fn render_capping() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capping() {
        OpenGlCappingAlgo::render_capping();
    }
}
