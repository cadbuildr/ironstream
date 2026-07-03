// FILE: ais_drag_action.rs
// occt: AIS_DragAction

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragAction {
    Start = 0,
    Confirmed = 1,
    Update = 2,
    Stop = 3,
    Abort = 4,
}

impl DragAction {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(DragAction::Start),
            1 => Some(DragAction::Confirmed),
            2 => Some(DragAction::Update),
            3 => Some(DragAction::Stop),
            4 => Some(DragAction::Abort),
            _ => None,
        }
    }

    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(DragAction::Start as u32, 0);
    }
}
