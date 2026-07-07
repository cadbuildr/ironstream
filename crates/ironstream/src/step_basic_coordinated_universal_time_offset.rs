// FILE: step_basic_coordinated_universal_time_offset.rs
// occt: StepBasic_CoordinatedUniversalTimeOffset

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepBasic_AheadOrBehind {
    Ahead,
    Behind,
}

pub struct StepBasic_CoordinatedUniversalTimeOffset {
    hour_offset: i32,
    minute_offset: i32,
    sense: StepBasic_AheadOrBehind,
    has_minute_offset: bool,
}

impl StepBasic_CoordinatedUniversalTimeOffset {
    pub fn new() -> Self {
        StepBasic_CoordinatedUniversalTimeOffset {
            hour_offset: 0,
            minute_offset: 0,
            sense: StepBasic_AheadOrBehind::Ahead,
            has_minute_offset: false,
        }
    }

    pub fn init(
        &mut self,
        hour_offset: i32,
        has_minute_offset: bool,
        minute_offset: i32,
        sense: StepBasic_AheadOrBehind,
    ) {
        self.hour_offset = hour_offset;
        self.has_minute_offset = has_minute_offset;
        if has_minute_offset {
            self.minute_offset = minute_offset;
        }
        self.sense = sense;
    }

    pub fn set_hour_offset(&mut self, hour_offset: i32) {
        self.hour_offset = hour_offset;
    }

    pub fn hour_offset(&self) -> i32 {
        self.hour_offset
    }

    pub fn set_minute_offset(&mut self, minute_offset: i32) {
        self.minute_offset = minute_offset;
        self.has_minute_offset = true;
    }

    pub fn unset_minute_offset(&mut self) {
        self.has_minute_offset = false;
    }

    pub fn minute_offset(&self) -> i32 {
        self.minute_offset
    }

    pub fn has_minute_offset(&self) -> bool {
        self.has_minute_offset
    }

    pub fn set_sense(&mut self, sense: StepBasic_AheadOrBehind) {
        self.sense = sense;
    }

    pub fn sense(&self) -> StepBasic_AheadOrBehind {
        self.sense
    }
}

impl Default for StepBasic_CoordinatedUniversalTimeOffset {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let offset = StepBasic_CoordinatedUniversalTimeOffset::new();
        assert_eq!(offset.hour_offset(), 0);
        assert!(!offset.has_minute_offset());
    }

    #[test]
    fn test_init() {
        let mut offset = StepBasic_CoordinatedUniversalTimeOffset::new();
        offset.init(5, true, 30, StepBasic_AheadOrBehind::Ahead);
        assert_eq!(offset.hour_offset(), 5);
        assert!(offset.has_minute_offset());
        assert_eq!(offset.minute_offset(), 30);
    }
}
