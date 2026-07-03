// FILE: b_rep_sweep_prism.rs
// occt: BRepSweep_Prism

use std::f64;

/// Provides natural constructors to build BRepSweep translated swept Primitives.
/// A prism is a shape obtained by translating a base shape along a vector or direction.
pub struct BRepSweepPrism {
    // Core state: wraps a BRepSweep_Translation internally.
    // For now, we store the vector and metadata needed for Shape generation.
    vec: (f64, f64, f64),
    copy: bool,
    canonize: bool,
    is_infinite: bool,
}

impl BRepSweepPrism {
    /// Builds the prism of base S and vector V.
    /// If Copy is true, S is copied.
    /// If Canonize is true then generated surfaces are attempted to be canonized in simple types.
    pub fn new_with_vec(
        _s: &TopoDS_Shape,
        v: &GpVec,
        copy: bool,
        canonize: bool,
    ) -> Self {
        let mag = v.magnitude();
        if mag.abs() < PRECISION_CONFUSION {
            panic!("BRepSweepPrism: vector magnitude is near zero");
        }

        BRepSweepPrism {
            vec: (v.x(), v.y(), v.z()),
            copy,
            canonize,
            is_infinite: false,
        }
    }

    /// Builds a semi-infinite or an infinite prism of base S.
    /// If Copy is true S is copied.
    /// If Inf is true the prism is infinite, if Inf is false the prism is semi-infinite in the direction D.
    /// If Canonize is true then generated surfaces are attempted to be canonized in simple types.
    pub fn new_with_dir(
        _s: &TopoDS_Shape,
        d: &GpDir,
        inf: bool,
        copy: bool,
        canonize: bool,
    ) -> Self {
        BRepSweepPrism {
            vec: (d.x(), d.y(), d.z()),
            copy,
            canonize,
            is_infinite: inf,
        }
    }

    /// Returns the TopoDS Shape attached to the prism.
    pub fn shape(&self) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the TopoDS Shape generated with aGenS (subShape of the generating shape).
    pub fn shape_from_gen(&self, _a_gen_s: &TopoDS_Shape) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the TopoDS Shape of the bottom of the prism.
    pub fn first_shape(&self) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the TopoDS Shape of the bottom of the prism generated with aGenS.
    pub fn first_shape_from_gen(&self, _a_gen_s: &TopoDS_Shape) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the TopoDS Shape of the top of the prism.
    pub fn last_shape(&self) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the TopoDS Shape of the top of the prism generated with aGenS.
    pub fn last_shape_from_gen(&self, _a_gen_s: &TopoDS_Shape) -> TopoDS_Shape {
        TopoDS_Shape::new()
    }

    /// Returns the Vector of the Prism.
    /// If it is an infinite prism the Vec is unitary.
    pub fn vec(&self) -> GpVec {
        GpVec::new(self.vec.0, self.vec.1, self.vec.2)
    }

    /// Returns true if aGenS is used in resulting shape.
    pub fn is_used(&self, _a_gen_s: &TopoDS_Shape) -> bool {
        false
    }

    /// Returns true if the shape generated from theS is used in result shape.
    pub fn gen_is_used(&self, _the_s: &TopoDS_Shape) -> bool {
        false
    }
}

// Stub structures for type safety
/// Minimal TopoDS_Shape stub
#[derive(Clone)]
pub struct TopoDS_Shape {
    // Placeholder
}

impl TopoDS_Shape {
    pub fn new() -> Self {
        TopoDS_Shape {}
    }
}

/// Minimal GpVec stub
#[derive(Clone, Copy, Debug)]
pub struct GpVec {
    x: f64,
    y: f64,
    z: f64,
}

impl GpVec {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GpVec { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

/// Minimal GpDir stub
#[derive(Clone, Copy, Debug)]
pub struct GpDir {
    x: f64,
    y: f64,
    z: f64,
}

impl GpDir {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let mag = (x * x + y * y + z * z).sqrt();
        if mag.abs() < PRECISION_CONFUSION {
            panic!("GpDir: direction magnitude is near zero");
        }
        GpDir {
            x: x / mag,
            y: y / mag,
            z: z / mag,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

const PRECISION_CONFUSION: f64 = 1e-7;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prism_with_vec() {
        let vec = GpVec::new(1.0, 0.0, 0.0);
        let shape = TopoDS_Shape::new();
        let prism = BRepSweepPrism::new_with_vec(&shape, &vec, false, true);

        let retrieved_vec = prism.vec();
        assert!((retrieved_vec.x() - 1.0).abs() < 1e-10);
        assert!((retrieved_vec.y() - 0.0).abs() < 1e-10);
        assert!((retrieved_vec.z() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_prism_with_vec_copy_canonize() {
        let vec = GpVec::new(2.0, 3.0, 4.0);
        let shape = TopoDS_Shape::new();
        let prism = BRepSweepPrism::new_with_vec(&shape, &vec, true, false);

        assert_eq!(prism.copy, true);
        assert_eq!(prism.canonize, false);
        assert!((prism.vec().magnitude() - vec.magnitude()).abs() < 1e-10);
    }

    #[test]
    fn test_prism_with_dir() {
        let dir = GpDir::new(1.0, 1.0, 1.0);
        let shape = TopoDS_Shape::new();
        let prism = BRepSweepPrism::new_with_dir(&shape, &dir, true, false, true);

        assert_eq!(prism.is_infinite, true);
        let retrieved_vec = prism.vec();
        let mag = retrieved_vec.magnitude();
        assert!((mag - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_shape_methods() {
        let vec = GpVec::new(1.0, 2.0, 3.0);
        let shape = TopoDS_Shape::new();
        let prism = BRepSweepPrism::new_with_vec(&shape, &vec, false, true);

        let _ = prism.shape();
        let _ = prism.first_shape();
        let _ = prism.last_shape();
        let _ = prism.is_used(&shape);
        let _ = prism.gen_is_used(&shape);
    }

    #[test]
    #[should_panic(expected = "vector magnitude is near zero")]
    fn test_prism_zero_vector() {
        let vec = GpVec::new(0.0, 0.0, 0.0);
        let shape = TopoDS_Shape::new();
        let _ = BRepSweepPrism::new_with_vec(&shape, &vec, false, true);
    }
}
