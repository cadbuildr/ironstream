// FILE: top_ope_b_rep_ds_shape_surface.rs
// occt: TopOpeBRepDS_ShapeSurface

/// Shape: Simplified shape.
#[derive(Clone, Debug)]
pub struct Shape {
    id: usize,
}

impl Shape {
    pub fn new(id: usize) -> Self {
        Shape { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Surface: Simplified surface.
#[derive(Clone, Debug)]
pub struct Surface {
    id: usize,
}

impl Surface {
    pub fn new(id: usize) -> Self {
        Surface { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// ShapeSurface: Pair of shape and surface.
#[derive(Clone, Debug)]
pub struct ShapeSurface {
    shape: Shape,
    surface: Surface,
}

impl ShapeSurface {
    pub fn new(shape: Shape, surface: Surface) -> Self {
        ShapeSurface { shape, surface }
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }

    pub fn set_surface(&mut self, surface: Surface) {
        self.surface = surface;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_new() {
        let shape = Shape::new(42);
        assert_eq!(shape.id(), 42);
    }

    #[test]
    fn test_surface_new() {
        let surf = Surface::new(99);
        assert_eq!(surf.id(), 99);
    }

    #[test]
    fn test_shape_surface_new() {
        let shape = Shape::new(10);
        let surface = Surface::new(20);
        let pair = ShapeSurface::new(shape, surface);

        assert_eq!(pair.shape().id(), 10);
        assert_eq!(pair.surface().id(), 20);
    }

    #[test]
    fn test_shape_surface_setters() {
        let shape = Shape::new(10);
        let surface = Surface::new(20);
        let mut pair = ShapeSurface::new(shape, surface);

        pair.set_shape(Shape::new(11));
        pair.set_surface(Surface::new(21));

        assert_eq!(pair.shape().id(), 11);
        assert_eq!(pair.surface().id(), 21);
    }
}
