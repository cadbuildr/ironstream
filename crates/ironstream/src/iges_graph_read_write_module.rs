// FILE: iges_graph_read_write_module.rs
// occt: IGESGraph_ReadWriteModule

pub struct IGESGraphReadWriteModule;

impl IGESGraphReadWriteModule {
    pub fn new() -> Self {
        IGESGraphReadWriteModule
    }

    pub fn case_iges(&self, typenum: i32, formnum: i32) -> i32 {
        match typenum {
            304 => {
                if formnum == 1 {
                    9
                } else if formnum == 2 {
                    7
                } else {
                    0
                }
            }
            310 => 13,
            312 => 12,
            314 => 1,
            406 => {
                match formnum {
                    1 => 2,
                    13 => 10,
                    16 => 3,
                    17 => 4,
                    18 => 6,
                    19 => 8,
                    20 => 5,
                    21 => 11,
                    22 => 14,
                    _ => 0,
                }
            }
            _ => 0,
        }
    }

    pub fn read_own_params(&self, _cn: i32) {
        // Dispatches to appropriate Tool class for reading parameters
        // Concrete implementation would interact with reader data
    }

    pub fn write_own_params(&self, _cn: i32) {
        // Dispatches to appropriate Tool class for writing parameters
        // Concrete implementation would interact with writer
    }
}

impl Default for IGESGraphReadWriteModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = IGESGraphReadWriteModule::new();
        assert_eq!(module.case_iges(314, 0), 1);
    }

    #[test]
    fn test_case_iges_304_form_1() {
        let module = IGESGraphReadWriteModule::new();
        assert_eq!(module.case_iges(304, 1), 9);
    }

    #[test]
    fn test_case_iges_304_form_2() {
        let module = IGESGraphReadWriteModule::new();
        assert_eq!(module.case_iges(304, 2), 7);
    }

    #[test]
    fn test_case_iges_406_form_21() {
        let module = IGESGraphReadWriteModule::new();
        assert_eq!(module.case_iges(406, 21), 11);
    }

    #[test]
    fn test_case_iges_406_form_22() {
        let module = IGESGraphReadWriteModule::new();
        assert_eq!(module.case_iges(406, 22), 14);
    }

    #[test]
    fn test_case_iges_unknown() {
        let module = IGESGraphReadWriteModule::new();
        assert_eq!(module.case_iges(999, 0), 0);
    }
}
