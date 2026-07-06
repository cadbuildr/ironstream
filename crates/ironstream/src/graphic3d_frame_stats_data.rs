// FILE: graphic3d_frame_stats_data.rs
// occt: Graphic3d_FrameStatsData

// Local model of Graphic3d_FrameStatsCounter (subset of counters, matching
// the OCCT enumeration order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum FrameStatsCounter {
    /// Number of ZLayers
    NbLayers = 0,
    /// Number of rendered ZLayers
    NbLayersNotCulled = 1,
    /// Number of defined structures
    NbStructs = 2,
    /// Number of rendered structures
    NbStructsNotCulled = 3,
}

pub const FRAME_STATS_COUNTER_NB: usize = 27;

// Define a simple timer enum for local use
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum FrameStatsTimer {
    /// Elapsed frame timer
    ElapsedFrame = 0,
    /// CPU frame timer
    CpuFrame = 1,
    /// CPU culling timer
    CpuCulling = 2,
    /// CPU picking timer
    CpuPicking = 3,
    /// CPU dynamics timer
    CpuDynamics = 4,
}

pub const FRAME_STATS_TIMER_NB: usize = 5; // CpuDynamics + 1

/// Data frame definition
#[derive(Debug, Clone)]
pub struct FrameStatsData {
    /// counters array
    counters: Vec<usize>,
    /// timers array
    timers: Vec<f64>,
    /// minimal values of timers
    timers_min: Vec<f64>,
    /// maximum values of timers
    timers_max: Vec<f64>,
    /// FPS meter (frames per seconds, elapsed time)
    fps: f64,
    /// CPU FPS meter (frames per seconds, CPU time)
    fps_cpu: f64,
    /// FPS meter for immediate redraws
    fps_immediate: f64,
    /// CPU FPS meter for immediate redraws
    fps_cpu_immediate: f64,
}

impl FrameStatsData {
    /// Empty constructor
    pub fn new() -> Self {
        FrameStatsData {
            counters: vec![0; FRAME_STATS_COUNTER_NB],
            timers: vec![0.0; FRAME_STATS_TIMER_NB],
            timers_min: vec![0.0; FRAME_STATS_TIMER_NB],
            timers_max: vec![0.0; FRAME_STATS_TIMER_NB],
            fps: 0.0,
            fps_cpu: 0.0,
            fps_immediate: 0.0,
            fps_cpu_immediate: 0.0,
        }
    }

    /// Returns FPS (frames per seconds, elapsed time)
    pub fn frame_rate(&self) -> f64 {
        self.fps
    }

    /// Returns CPU FPS (frames per seconds, CPU time)
    pub fn frame_rate_cpu(&self) -> f64 {
        self.fps_cpu
    }

    /// Returns FPS for immediate redraws
    pub fn immediate_frame_rate(&self) -> f64 {
        self.fps_immediate
    }

    /// Returns CPU FPS for immediate redraws
    pub fn immediate_frame_rate_cpu(&self) -> f64 {
        self.fps_cpu_immediate
    }

    /// Get counter value
    pub fn counter_value(&self, index: FrameStatsCounter) -> usize {
        let idx = index as usize;
        if idx < self.counters.len() {
            self.counters[idx]
        } else {
            0
        }
    }

    /// Get counter value using indexing
    pub fn get_counter(&self, index: FrameStatsCounter) -> usize {
        self.counter_value(index)
    }

    /// Get timer value
    pub fn timer_value(&self, index: FrameStatsTimer) -> f64 {
        let idx = index as usize;
        if idx < self.timers.len() {
            self.timers[idx]
        } else {
            0.0
        }
    }

    /// Get timer value using indexing
    pub fn get_timer(&self, index: FrameStatsTimer) -> f64 {
        self.timer_value(index)
    }

    /// Reset data
    pub fn reset(&mut self) {
        for c in &mut self.counters {
            *c = 0;
        }
        for t in &mut self.timers {
            *t = 0.0;
        }
        for t in &mut self.timers_min {
            *t = 0.0;
        }
        for t in &mut self.timers_max {
            *t = 0.0;
        }
        self.fps = 0.0;
        self.fps_cpu = 0.0;
        self.fps_immediate = 0.0;
        self.fps_cpu_immediate = 0.0;
    }

    /// Fill with maximum values from another FrameStatsData
    pub fn fill_max(&mut self, other: &FrameStatsData) {
        for (i, &c) in other.counters.iter().enumerate() {
            if i < self.counters.len() && c > self.counters[i] {
                self.counters[i] = c;
            }
        }
        for (i, &t) in other.timers.iter().enumerate() {
            if i < self.timers.len() && t > self.timers[i] {
                self.timers[i] = t;
            }
        }
        self.fps = self.fps.max(other.fps);
        self.fps_cpu = self.fps_cpu.max(other.fps_cpu);
        self.fps_immediate = self.fps_immediate.max(other.fps_immediate);
        self.fps_cpu_immediate = self.fps_cpu_immediate.max(other.fps_cpu_immediate);
    }
}

impl Default for FrameStatsData {
    fn default() -> Self {
        Self::new()
    }
}

/// Temporary data frame definition (extends FrameStatsData)
#[derive(Debug, Clone)]
pub struct FrameStatsDataTmp {
    /// Base frame stats data
    base: FrameStatsData,
    /// OSD timers for time measurements
    osd_timers: Vec<f64>,
    /// previous timer values
    timers_prev: Vec<f64>,
}

impl FrameStatsDataTmp {
    /// Empty constructor
    pub fn new() -> Self {
        FrameStatsDataTmp {
            base: FrameStatsData::new(),
            osd_timers: vec![0.0; FRAME_STATS_TIMER_NB],
            timers_prev: vec![0.0; FRAME_STATS_TIMER_NB],
        }
    }

    /// Reset data
    pub fn reset(&mut self) {
        self.base.reset();
        for t in &mut self.osd_timers {
            *t = 0.0;
        }
        for t in &mut self.timers_prev {
            *t = 0.0;
        }
    }

    /// Compute average data considering the amount of rendered frames
    pub fn flush_timers(&mut self, _nb_frames: usize, _is_final: bool) {
        // TODO: Implement timer flushing logic based on OCCT
        // This would require more complex timer management
    }

    /// Returns frame rate for modification
    pub fn change_frame_rate(&mut self) -> &mut f64 {
        &mut self.base.fps
    }

    /// Returns CPU frame rate for modification
    pub fn change_frame_rate_cpu(&mut self) -> &mut f64 {
        &mut self.base.fps_cpu
    }

    /// Returns immediate frame rate for modification
    pub fn change_immediate_frame_rate(&mut self) -> &mut f64 {
        &mut self.base.fps_immediate
    }

    /// Returns immediate CPU frame rate for modification
    pub fn change_immediate_frame_rate_cpu(&mut self) -> &mut f64 {
        &mut self.base.fps_cpu_immediate
    }

    /// Returns counter value for modification
    pub fn change_counter_value(&mut self, index: FrameStatsCounter) -> &mut usize {
        let idx = index as usize;
        if idx >= self.base.counters.len() {
            self.base.counters.resize(idx + 1, 0);
        }
        &mut self.base.counters[idx]
    }

    /// Returns timer value for modification
    pub fn change_timer_value(&mut self, index: FrameStatsTimer) -> &mut f64 {
        let idx = index as usize;
        if idx >= self.base.timers.len() {
            self.base.timers.resize(idx + 1, 0.0);
        }
        &mut self.base.timers[idx]
    }

    /// Get counter value
    pub fn counter_value(&self, index: FrameStatsCounter) -> usize {
        self.base.counter_value(index)
    }

    /// Get timer value
    pub fn timer_value(&self, index: FrameStatsTimer) -> f64 {
        self.base.timer_value(index)
    }

    /// Get frame rate
    pub fn frame_rate(&self) -> f64 {
        self.base.frame_rate()
    }

    /// Get CPU frame rate
    pub fn frame_rate_cpu(&self) -> f64 {
        self.base.frame_rate_cpu()
    }
}

impl Default for FrameStatsDataTmp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_stats_data_creation() {
        let data = FrameStatsData::new();
        assert_eq!(data.frame_rate(), 0.0);
        assert_eq!(data.frame_rate_cpu(), 0.0);
        assert_eq!(data.immediate_frame_rate(), 0.0);
        assert_eq!(data.immediate_frame_rate_cpu(), 0.0);
    }

    #[test]
    fn test_frame_stats_data_counters() {
        let data = FrameStatsData::new();
        assert_eq!(data.counter_value(FrameStatsCounter::NbLayers), 0);
        assert_eq!(data.counter_value(FrameStatsCounter::NbStructs), 0);
    }

    #[test]
    fn test_frame_stats_data_timers() {
        let data = FrameStatsData::new();
        assert_eq!(data.timer_value(FrameStatsTimer::ElapsedFrame), 0.0);
        assert_eq!(data.timer_value(FrameStatsTimer::CpuFrame), 0.0);
    }

    #[test]
    fn test_frame_stats_data_reset() {
        let mut data = FrameStatsData::new();
        data.counters[0] = 42;
        data.timers[0] = 1.5;
        data.fps = 60.0;

        data.reset();

        assert_eq!(data.counters[0], 0);
        assert_eq!(data.timers[0], 0.0);
        assert_eq!(data.frame_rate(), 0.0);
    }

    #[test]
    fn test_frame_stats_data_fill_max() {
        let mut data1 = FrameStatsData::new();
        data1.counters[0] = 10;
        data1.fps = 60.0;

        let mut data2 = FrameStatsData::new();
        data2.counters[0] = 20;
        data2.fps = 30.0;

        data1.fill_max(&data2);

        assert_eq!(data1.counters[0], 20);
        assert_eq!(data1.frame_rate(), 60.0); // max of 60 and 30
    }

    #[test]
    fn test_frame_stats_data_tmp_creation() {
        let data = FrameStatsDataTmp::new();
        assert_eq!(data.frame_rate(), 0.0);
    }

    #[test]
    fn test_frame_stats_data_tmp_change_counter() {
        let mut data = FrameStatsDataTmp::new();
        {
            let counter = data.change_counter_value(FrameStatsCounter::NbStructs);
            *counter = 42;
        }
        assert_eq!(data.counter_value(FrameStatsCounter::NbStructs), 42);
    }

    #[test]
    fn test_frame_stats_data_tmp_change_timer() {
        let mut data = FrameStatsDataTmp::new();
        {
            let timer = data.change_timer_value(FrameStatsTimer::CpuFrame);
            *timer = 1.5;
        }
        assert_eq!(data.timer_value(FrameStatsTimer::CpuFrame), 1.5);
    }

    #[test]
    fn test_frame_stats_data_tmp_reset() {
        let mut data = FrameStatsDataTmp::new();
        *data.change_counter_value(FrameStatsCounter::NbLayers) = 10;
        *data.change_frame_rate() = 60.0;

        data.reset();

        assert_eq!(data.counter_value(FrameStatsCounter::NbLayers), 0);
        assert_eq!(data.frame_rate(), 0.0);
    }
}
