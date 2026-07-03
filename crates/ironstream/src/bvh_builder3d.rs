// FILE: bvh_builder3d.rs
// occt: BVH_Builder3d

/// Type alias representing a specialized BVH builder for 3D space with f64 precision.
///
/// In the original OCCT C++, BVH_Builder3d is defined as a typedef:
/// ```ignore
/// typedef BVH_Builder<double, 3> BVH_Builder3d;
/// ```
///
/// This is a specialization of the generic BVH_Builder template to:
/// - Numeric type: f64 (double precision floating point)
/// - Dimension: 3 (3D vectors)
///
/// The BVH_Builder is an abstract base class that implements various strategies
/// for constructing BVH (Bounding Volume Hierarchy) trees. Concrete implementations
/// include linear builders, binned builders, and other specialized variants.
///
/// This typedef serves as a convenience for users working with 3D geometry,
/// eliminating the need to specify the template parameters each time.
///
/// # Example
/// ```ignore
/// // In C++:
/// Handle(BVH_Builder3d) builder = new BVH_LinearBuilder<double, 3>(leafSize, maxDepth);
///
/// // In Rust, users would work with concrete builder implementations
/// // that implement the builder pattern for 3D double-precision geometry.
/// ```
///
/// Note: This is primarily a documentation type in Rust. Actual builder
/// implementations should be provided by concrete builder types in the crate.
#[derive(Clone, Copy, Debug, Default)]
pub struct BVH_Builder3d {
    _phantom: core::marker::PhantomData<(f64, [f64; 3])>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bvh_builder3d_exists() {
        // Verify that BVH_Builder3d type exists and can be instantiated
        let _builder = BVH_Builder3d {
            _phantom: core::marker::PhantomData,
        };
    }

    #[test]
    fn test_bvh_builder3d_is_send_sync() {
        // Verify that BVH_Builder3d is Send + Sync for multi-threaded use
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<BVH_Builder3d>();
    }

    #[test]
    fn test_bvh_builder3d_can_be_cloned() {
        // BVH_Builder3d should be lightweight and cheaply cloneable
        #[derive(Clone)]
        struct _AssertClone {
            _b: BVH_Builder3d,
        }
    }
}
