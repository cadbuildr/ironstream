// FILE: iges_graph_pick.rs
// occt: IGESGraph_Pick

pub struct IGESGraphPick {
    nb_property_values: i32,
    pick: i32,
}

impl IGESGraphPick {
    pub fn new() -> Self {
        IGESGraphPick {
            nb_property_values: 0,
            pick: 0,
        }
    }

    pub fn init(&mut self, nb_props: i32, a_pick_status: i32) {
        self.nb_property_values = nb_props;
        self.pick = a_pick_status;
    }

    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    pub fn pick_flag(&self) -> i32 {
        self.pick
    }

    pub fn is_pickable(&self) -> bool {
        self.pick == 0
    }
}

impl Default for IGESGraphPick {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pick = IGESGraphPick::new();
        assert_eq!(pick.nb_property_values(), 0);
        assert_eq!(pick.pick_flag(), 0);
        assert!(pick.is_pickable());
    }

    #[test]
    fn test_init() {
        let mut pick = IGESGraphPick::new();
        pick.init(1, 0);
        assert_eq!(pick.nb_property_values(), 1);
        assert_eq!(pick.pick_flag(), 0);
        assert!(pick.is_pickable());
    }

    #[test]
    fn test_not_pickable() {
        let mut pick = IGESGraphPick::new();
        pick.init(1, 1);
        assert_eq!(pick.pick_flag(), 1);
        assert!(!pick.is_pickable());
    }
}
