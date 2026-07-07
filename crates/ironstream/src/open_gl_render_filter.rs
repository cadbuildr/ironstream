// FILE: open_gl_render_filter.rs
// occt: OpenGl_RenderFilter

/// Filter for selective rendering.
#[derive(Debug, Clone)]
pub struct OpenGlRenderFilter;

impl OpenGlRenderFilter {
    pub fn new() -> Self {
        OpenGlRenderFilter
    }
}

impl Default for OpenGlRenderFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_filter() {
        let _filter = OpenGlRenderFilter::new();
    }
}
