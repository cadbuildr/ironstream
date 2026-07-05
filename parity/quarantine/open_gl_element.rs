// FILE: open_gl_element.rs
// occt: OpenGl_Element

pub trait OpenGlElement {
    fn render(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element() {}
}
