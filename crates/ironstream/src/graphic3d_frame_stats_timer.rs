// FILE: graphic3d_frame_stats_timer.rs
// occt: Graphic3d_FrameStatsTimer

/// Timers for collecting frame performance statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FrameStatsTimer {
    ElapsedFrame = 0,
    CpuFrame = 1,
    CpuCulling = 2,
    CpuPicking = 3,
    CpuDynamics = 4,
}

pub const FRAME_STATS_TIMER_NB: usize = 5;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_stats_timer_values() {
        assert_eq!(FrameStatsTimer::ElapsedFrame as u32, 0);
        assert_eq!(FrameStatsTimer::CpuFrame as u32, 1);
        assert_eq!(FrameStatsTimer::CpuCulling as u32, 2);
        assert_eq!(FrameStatsTimer::CpuPicking as u32, 3);
        assert_eq!(FrameStatsTimer::CpuDynamics as u32, 4);
    }

    #[test]
    fn test_frame_stats_timer_nb() {
        assert_eq!(FRAME_STATS_TIMER_NB, 5);
        assert_eq!(FrameStatsTimer::CpuDynamics as u32 + 1, FRAME_STATS_TIMER_NB as u32);
    }

    #[test]
    fn test_frame_stats_timer_clone() {
        let timer = FrameStatsTimer::CpuFrame;
        let cloned = timer;
        assert_eq!(timer, cloned);
    }
}
