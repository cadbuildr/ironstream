// FILE: ais_manipulator_owner.rs
// occt: AIS_ManipulatorOwner

#[derive(Clone, Debug)]
pub struct ManipulatorOwner {
    owner_id: u32,
    manipulator_id: u32,
}

impl ManipulatorOwner {
    pub fn new(owner_id: u32, manipulator_id: u32) -> Self {
        Self { owner_id, manipulator_id }
    }
    pub fn owner_id(&self) -> u32 { self.owner_id }
    pub fn manipulator_id(&self) -> u32 { self.manipulator_id }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let o = ManipulatorOwner::new(1, 2);
        assert_eq!(o.owner_id(), 1);
    }
}
