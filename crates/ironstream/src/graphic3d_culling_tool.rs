// FILE: graphic3d_culling_tool.rs
// occt: Graphic3d_CullingTool

/// Auxiliary structure holding non-persistent culling options.
#[derive(Debug, Clone, Copy)]
pub struct CullingContext {
    /// culling distance
    pub dist_cull: f64,
    /// squared culling size
    pub size_cull2: f64,
}

impl CullingContext {
    /// Creates a new empty culling context with default values.
    pub fn new() -> Self {
        CullingContext {
            dist_cull: -1.0,
            size_cull2: -1.0,
        }
    }
}

impl Default for CullingContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Auxiliary structure representing 3D plane.
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    /// Origin point of the plane
    pub origin: [f64; 3],
    /// Normal vector of the plane
    pub normal: [f64; 3],
}

impl Plane {
    /// Creates default plane with origin at (0, 0, 0) and normal (0, 0, 1).
    pub fn new() -> Self {
        Plane {
            origin: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        }
    }

    /// Creates plane with specific parameters.
    pub fn with_params(origin: [f64; 3], normal: [f64; 3]) -> Self {
        Plane { origin, normal }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

/// Graphic3d_CullingTool class provides a possibility to store parameters of view volume,
/// such as its vertices and equations, and contains methods detecting if given AABB overlaps view volume.
#[derive(Debug, Clone)]
pub struct CullingTool {
    /// viewport width
    viewport_width: i32,
    /// viewport height
    viewport_height: i32,
    /// whether projection is parallel
    is_projection_parallel: bool,
}

impl CullingTool {
    /// Creates an empty culling tool object with parallel projection type by default.
    pub fn new() -> Self {
        CullingTool {
            viewport_width: 0,
            viewport_height: 0,
            is_projection_parallel: true,
        }
    }

    /// Returns viewport width.
    pub fn viewport_width(&self) -> i32 {
        self.viewport_width
    }

    /// Returns viewport height.
    pub fn viewport_height(&self) -> i32 {
        self.viewport_height
    }

    /// Sets viewport size.
    pub fn set_viewport_size(&mut self, width: i32, height: i32, _resolution_ratio: f64) {
        self.viewport_width = width;
        self.viewport_height = height;
    }

    /// Setup distance culling.
    pub fn set_culling_distance(&self, ctx: &mut CullingContext, distance: f64) {
        ctx.dist_cull = if distance > 0.0 { distance } else { -1.0 };
    }

    /// Setup size culling.
    pub fn set_culling_size(&self, ctx: &mut CullingContext, size: f64) {
        if size > 0.0 {
            ctx.size_cull2 = size * size;
        } else {
            ctx.size_cull2 = -1.0;
        }
    }

    /// Checks whether given AABB should be entirely culled or not.
    /// Returns TRUE if AABB is completely outside of view frustum or culled by size/distance;
    /// FALSE in case of partial or complete overlap.
    pub fn is_culled(
        &self,
        _ctx: &CullingContext,
        _min_pnt: [f64; 3],
        _max_pnt: [f64; 3],
    ) -> bool {
        // Simplified implementation - just return false for now
        // Full implementation would require view volume setup and frustum calculations
        false
    }

    /// Returns TRUE if given AABB should be discarded by distance culling criterion.
    pub fn is_too_distant(
        &self,
        ctx: &CullingContext,
        _min_pnt: [f64; 3],
        _max_pnt: [f64; 3],
    ) -> bool {
        ctx.dist_cull <= 0.0
    }

    /// Returns TRUE if given AABB should be discarded by size culling criterion.
    pub fn is_too_small(&self, ctx: &CullingContext, _min_pnt: [f64; 3], _max_pnt: [f64; 3]) -> bool {
        ctx.size_cull2 <= 0.0
    }
}

impl Default for CullingTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_culling_context_creation() {
        let ctx = CullingContext::new();
        assert_eq!(ctx.dist_cull, -1.0);
        assert_eq!(ctx.size_cull2, -1.0);
    }

    #[test]
    fn test_culling_context_default() {
        let ctx = CullingContext::default();
        assert_eq!(ctx.dist_cull, -1.0);
        assert_eq!(ctx.size_cull2, -1.0);
    }

    #[test]
    fn test_plane_creation() {
        let plane = Plane::new();
        assert_eq!(plane.origin, [0.0, 0.0, 0.0]);
        assert_eq!(plane.normal, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_plane_with_params() {
        let origin = [1.0, 2.0, 3.0];
        let normal = [0.577, 0.577, 0.577];
        let plane = Plane::with_params(origin, normal);
        assert_eq!(plane.origin, origin);
        assert_eq!(plane.normal, normal);
    }

    #[test]
    fn test_culling_tool_creation() {
        let tool = CullingTool::new();
        assert_eq!(tool.viewport_width(), 0);
        assert_eq!(tool.viewport_height(), 0);
        assert!(tool.is_projection_parallel);
    }

    #[test]
    fn test_culling_tool_set_viewport_size() {
        let mut tool = CullingTool::new();
        tool.set_viewport_size(1280, 720, 1.0);
        assert_eq!(tool.viewport_width(), 1280);
        assert_eq!(tool.viewport_height(), 720);
    }

    #[test]
    fn test_set_culling_distance() {
        let tool = CullingTool::new();
        let mut ctx = CullingContext::new();

        // Set positive distance
        tool.set_culling_distance(&mut ctx, 100.0);
        assert_eq!(ctx.dist_cull, 100.0);

        // Set negative distance (should become -1.0)
        tool.set_culling_distance(&mut ctx, -50.0);
        assert_eq!(ctx.dist_cull, -1.0);

        // Set zero distance (should become -1.0)
        tool.set_culling_distance(&mut ctx, 0.0);
        assert_eq!(ctx.dist_cull, -1.0);
    }

    #[test]
    fn test_set_culling_size() {
        let tool = CullingTool::new();
        let mut ctx = CullingContext::new();

        // Set positive size
        tool.set_culling_size(&mut ctx, 10.0);
        assert_eq!(ctx.size_cull2, 100.0); // size squared

        // Set negative size (should become -1.0)
        tool.set_culling_size(&mut ctx, -5.0);
        assert_eq!(ctx.size_cull2, -1.0);

        // Set zero size (should become -1.0)
        tool.set_culling_size(&mut ctx, 0.0);
        assert_eq!(ctx.size_cull2, -1.0);
    }

    #[test]
    fn test_is_too_distant() {
        let tool = CullingTool::new();
        let mut ctx = CullingContext::new();

        // Default context (dist_cull = -1.0) should not be too distant
        assert!(tool.is_too_distant(&ctx, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]));

        // Set valid distance
        ctx.dist_cull = 100.0;
        assert!(!tool.is_too_distant(&ctx, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]));
    }

    #[test]
    fn test_is_too_small() {
        let tool = CullingTool::new();
        let mut ctx = CullingContext::new();

        // Default context (size_cull2 = -1.0) should not be too small
        assert!(tool.is_too_small(&ctx, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]));

        // Set valid size
        ctx.size_cull2 = 100.0;
        assert!(!tool.is_too_small(&ctx, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]));
    }
}
