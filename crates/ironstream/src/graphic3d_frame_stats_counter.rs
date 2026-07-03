// FILE: graphic3d_frame_stats_counter.rs
// occt: Graphic3d_FrameStatsCounter

/// Stats counter enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum FrameStatsCounter {
    // overall scene counters
    /// number of ZLayers
    NbLayers = 0,
    /// number of defined OpenGl_Structure
    NbStructs = 1,
    /// estimated GPU memory used for geometry
    EstimatedBytesGeom = 2,
    /// estimated GPU memory used for FBOs
    EstimatedBytesFbos = 3,
    /// estimated GPU memory used for textures
    EstimatedBytesTextures = 4,

    // rendered counters
    /// number of not culled ZLayers
    NbLayersNotCulled = 5,
    /// number of not culled OpenGl_Structure
    NbStructsNotCulled = 6,
    /// number of not culled OpenGl_Group
    NbGroupsNotCulled = 7,
    /// number of not culled OpenGl_Element
    NbElemsNotCulled = 8,
    /// number of not culled OpenGl_PrimitiveArray drawing triangles
    NbElemsFillNotCulled = 9,
    /// number of not culled OpenGl_PrimitiveArray drawing lines
    NbElemsLineNotCulled = 10,
    /// number of not culled OpenGl_PrimitiveArray drawing points
    NbElemsPointNotCulled = 11,
    /// number of not culled OpenGl_Text
    NbElemsTextNotCulled = 12,
    /// number of not culled (as structure) triangles
    NbTrianglesNotCulled = 13,
    /// number of not culled (as structure) line segments
    NbLinesNotCulled = 14,
    /// number of not culled (as structure) points
    NbPointsNotCulled = 15,

    // immediate layer rendered counters
    /// number of ZLayers in immediate layer
    NbLayersImmediate = 16,
    /// number of OpenGl_Structure in immediate layer
    NbStructsImmediate = 17,
    /// number of OpenGl_Group in immediate layer
    NbGroupsImmediate = 18,
    /// number of OpenGl_Element in immediate layer
    NbElemsImmediate = 19,
    /// number of OpenGl_PrimitiveArray drawing triangles in immediate layer
    NbElemsFillImmediate = 20,
    /// number of OpenGl_PrimitiveArray drawing lines in immediate layer
    NbElemsLineImmediate = 21,
    /// number of OpenGl_PrimitiveArray drawing points in immediate layer
    NbElemsPointImmediate = 22,
    /// number of OpenGl_Text in immediate layer
    NbElemsTextImmediate = 23,
    /// number of triangles in immediate layer
    NbTrianglesImmediate = 24,
    /// number of line segments in immediate layer
    NbLinesImmediate = 25,
    /// number of points in immediate layer
    NbPointsImmediate = 26,
}

pub const FRAME_STATS_COUNTER_NB: usize = 27; // NbPointsImmediate + 1

pub const FRAME_STATS_COUNTER_SCENE_LOWER: usize = 0; // NbLayers
pub const FRAME_STATS_COUNTER_SCENE_UPPER: usize = 4; // EstimatedBytesTextures

pub const FRAME_STATS_COUNTER_RENDERED_LOWER: usize = 5; // NbLayersNotCulled
pub const FRAME_STATS_COUNTER_RENDERED_UPPER: usize = 15; // NbPointsNotCulled

pub const FRAME_STATS_COUNTER_IMMEDIATE_LOWER: usize = 16; // NbLayersImmediate
pub const FRAME_STATS_COUNTER_IMMEDIATE_UPPER: usize = 26; // NbPointsImmediate

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_stats_counter_values() {
        assert_eq!(FrameStatsCounter::NbLayers as usize, 0);
        assert_eq!(FrameStatsCounter::NbStructs as usize, 1);
        assert_eq!(FrameStatsCounter::EstimatedBytesGeom as usize, 2);
        assert_eq!(FrameStatsCounter::EstimatedBytesFbos as usize, 3);
        assert_eq!(FrameStatsCounter::EstimatedBytesTextures as usize, 4);
        assert_eq!(FrameStatsCounter::NbLayersNotCulled as usize, 5);
        assert_eq!(FrameStatsCounter::NbStructsNotCulled as usize, 6);
        assert_eq!(FrameStatsCounter::NbPointsImmediate as usize, 26);
    }

    #[test]
    fn test_frame_stats_counter_nb() {
        assert_eq!(FRAME_STATS_COUNTER_NB, 27);
    }

    #[test]
    fn test_frame_stats_counter_ranges() {
        assert_eq!(FRAME_STATS_COUNTER_SCENE_LOWER, 0);
        assert_eq!(FRAME_STATS_COUNTER_SCENE_UPPER, 4);

        assert_eq!(FRAME_STATS_COUNTER_RENDERED_LOWER, 5);
        assert_eq!(FRAME_STATS_COUNTER_RENDERED_UPPER, 15);

        assert_eq!(FRAME_STATS_COUNTER_IMMEDIATE_LOWER, 16);
        assert_eq!(FRAME_STATS_COUNTER_IMMEDIATE_UPPER, 26);
    }

    #[test]
    fn test_frame_stats_counter_copy() {
        let counter = FrameStatsCounter::NbStructsNotCulled;
        let counter_copy = counter;
        assert_eq!(counter, counter_copy);
    }
}
