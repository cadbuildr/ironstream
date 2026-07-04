// FILE: iges_dimen_dimensioned_geometry.rs
// occt: IGESDimen_DimensionedGeometry

/// Defines IGES Dimensioned Geometry, Type <402> Form <13>
/// in package IGESDimen
pub struct IgesDimen_DimensionedGeometry {
    nb_dimensions: i32,
    dimension: Option<Box<IgesData_IgesEntity>>,
    geometry_entities: Vec<IgesData_IgesEntity>,
}

impl IgesDimen_DimensionedGeometry {
    /// Create a new DimensionedGeometry entity
    pub fn new() -> Self {
        IgesDimen_DimensionedGeometry {
            nb_dimensions: 0,
            dimension: None,
            geometry_entities: Vec::new(),
        }
    }

    pub fn init(&mut self, nb_dims: i32, a_dimension: IgesData_IgesEntity, entities: Vec<IgesData_IgesEntity>) {
        self.nb_dimensions = nb_dims;
        self.dimension = Some(Box::new(a_dimension));
        self.geometry_entities = entities;
    }

    /// Returns the number of dimensions
    pub fn nb_dimensions(&self) -> i32 {
        self.nb_dimensions
    }

    /// Returns the number of associated geometry entities
    pub fn nb_geometry_entities(&self) -> i32 {
        self.geometry_entities.len() as i32
    }

    /// Returns the Dimension entity
    pub fn dimension_entity(&self) -> Option<&IgesData_IgesEntity> {
        self.dimension.as_ref().map(|d| d.as_ref())
    }

    /// Returns the num'th Geometry entity
    /// raises exception if index <= 0 or index > nb_geometry_entities()
    pub fn geometry_entity(&self, index: usize) -> Option<&IgesData_IgesEntity> {
        if index == 0 || index > self.geometry_entities.len() {
            return None;
        }
        Some(&self.geometry_entities[index - 1])
    }
}

impl Default for IgesDimen_DimensionedGeometry {
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
    fn test_dimensioned_geometry_creation() {
        let geom = IgesDimen_DimensionedGeometry::new();
        assert_eq!(geom.nb_dimensions(), 0);
        assert_eq!(geom.nb_geometry_entities(), 0);
    }

    #[test]
    fn test_dimensioned_geometry_init() {
        let mut geom = IgesDimen_DimensionedGeometry::new();
        let dim = IgesData_IgesEntity;
        let ents = vec![IgesData_IgesEntity, IgesData_IgesEntity];

        geom.init(1, dim, ents);

        assert_eq!(geom.nb_dimensions(), 1);
        assert_eq!(geom.nb_geometry_entities(), 2);
        assert!(geom.geometry_entity(1).is_some());
    }
}
