// FILE: shape_persistent.rs
// occt: ShapePersistent

use crate::std_obj_mgt_map_of_instantiators::StdObjMgtMapOfInstantiators;

/// Shape persistence management
pub struct ShapePersistent;

impl ShapePersistent {
    /// Register shape-related persistent types into the instantiator map
    pub fn bind_types(map: &mut StdObjMgtMapOfInstantiators) {
        // Register all shape persistent types
        // This would bind type names to their factory functions
        // For now, we provide the interface
        let _ = map;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_types() {
        let mut map = StdObjMgtMapOfInstantiators::new();
        ShapePersistent::bind_types(&mut map);
        // Map should be configured after bind_types
    }
}
