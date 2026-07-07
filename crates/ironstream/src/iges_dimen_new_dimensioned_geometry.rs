// FILE: iges_dimen_new_dimensioned_geometry.rs
// occt: IGESDimen_NewDimensionedGeometry

/// Defines NewDimensionedGeometry, Type <402> Form <21>
/// in package IGESDimen
pub struct IgesDimen_NewDimensionedGeometry {
    dimensions: Vec<IgesData_IgesEntity>,
    geometry: Vec<IgesData_IgesEntity>,
}

impl IgesDimen_NewDimensionedGeometry {
    pub fn new() -> Self {
        IgesDimen_NewDimensionedGeometry {
            dimensions: Vec::new(),
            geometry: Vec::new(),
        }
    }

    pub fn init(&mut self, dims: Vec<IgesData_IgesEntity>, geom: Vec<IgesData_IgesEntity>) {
        self.dimensions = dims;
        self.geometry = geom;
    }

    pub fn nb_dimensions(&self) -> usize {
        self.dimensions.len()
    }

    pub fn dimension(&self, index: usize) -> Option<&IgesData_IgesEntity> {
        if index == 0 || index > self.dimensions.len() {
            return None;
        }
        Some(&self.dimensions[index - 1])
    }

    pub fn nb_geometry_entities(&self) -> usize {
        self.geometry.len()
    }

    pub fn geometry_entity(&self, index: usize) -> Option<&IgesData_IgesEntity> {
        if index == 0 || index > self.geometry.len() {
            return None;
        }
        Some(&self.geometry[index - 1])
    }
}

impl Default for IgesDimen_NewDimensionedGeometry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct IgesData_IgesEntity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dimensioned_geometry_creation() {
        let geom = IgesDimen_NewDimensionedGeometry::new();
        assert_eq!(geom.nb_dimensions(), 0);
    }
}
