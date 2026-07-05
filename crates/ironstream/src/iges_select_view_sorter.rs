// FILE: iges_select_view_sorter.rs
// occt: IGESSelect_ViewSorter

//! Sorts IGES Entities by their attached views and drawings.
//!
//! This type splits a set of entities according to different views they are attached to,
//! then creates packets according to single views (optionally with drawings) or according
//! to drawings and their referenced views.

use std::collections::{HashMap, BTreeMap};

/// Drawing entity type number in IGES
const IGES_DRAWING_TYPE: i32 = 404;

/// Represents a transient IGES entity
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IGESEntity {
    id: usize,
    type_number: i32,
    is_view_kind: bool,
}

impl IGESEntity {
    pub fn new(id: usize, type_number: i32, is_view_kind: bool) -> Self {
        IGESEntity {
            id,
            type_number,
            is_view_kind,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn type_number(&self) -> i32 {
        self.type_number
    }

    pub fn is_view_kind(&self) -> bool {
        self.is_view_kind
    }

    pub fn is_single(&self) -> bool {
        self.is_view_kind && self.type_number != IGES_DRAWING_TYPE
    }
}

/// IGES Model containing entities
pub struct IGESModel {
    entities: Vec<IGESEntity>,
}

impl IGESModel {
    pub fn new(entities: Vec<IGESEntity>) -> Self {
        IGESModel { entities }
    }

    pub fn nb_entities(&self) -> usize {
        self.entities.len()
    }

    pub fn entity(&self, index: usize) -> Option<&IGESEntity> {
        if index > 0 && index <= self.entities.len() {
            Some(&self.entities[index - 1])
        } else {
            None
        }
    }
}

/// Represents a set of entities grouped by view/drawing
pub struct PacketList {
    packets: Vec<Vec<usize>>,
}

impl PacketList {
    pub fn new() -> Self {
        PacketList {
            packets: Vec::new(),
        }
    }

    pub fn add_packet(&mut self) {
        self.packets.push(Vec::new());
    }

    pub fn add_entity(&mut self, entity_id: usize) {
        if !self.packets.is_empty() {
            self.packets.last_mut().unwrap().push(entity_id);
        }
    }

    pub fn packets(&self) -> &[Vec<usize>] {
        &self.packets
    }
}

/// Represents a graph of entity relationships
pub struct InterfaceGraph {
    sharings: HashMap<usize, Vec<usize>>,
}

impl InterfaceGraph {
    pub fn new() -> Self {
        InterfaceGraph {
            sharings: HashMap::new(),
        }
    }

    pub fn add_sharing(&mut self, entity_id: usize, shared_by: usize) {
        self.sharings
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(shared_by);
    }

    pub fn sharings(&self, entity_id: usize) -> Vec<usize> {
        self.sharings.get(&entity_id).cloned().unwrap_or_default()
    }
}

/// Sorter for IGES views and drawings
pub struct IGESSelectViewSorter {
    model: Option<IGESModel>,
    entity_map: BTreeMap<usize, usize>,
    items_map: BTreeMap<usize, IGESEntity>,
    finals_map: BTreeMap<usize, IGESEntity>,
    entity_indices: Vec<usize>,
    final_indices: Vec<usize>,
}

impl IGESSelectViewSorter {
    /// Creates an empty ViewSorter
    pub fn new() -> Self {
        IGESSelectViewSorter {
            model: None,
            entity_map: BTreeMap::new(),
            items_map: BTreeMap::new(),
            finals_map: BTreeMap::new(),
            entity_indices: Vec::new(),
            final_indices: Vec::new(),
        }
    }

    /// Sets the IGES model
    pub fn set_model(&mut self, model: IGESModel) {
        self.model = Some(model);
    }

    /// Clears all recorded data
    pub fn clear(&mut self) {
        self.entity_map.clear();
        self.items_map.clear();
        self.finals_map.clear();
        self.entity_indices.clear();
        self.final_indices.clear();
    }

    /// Adds an entity to the sorter
    pub fn add_entity(&mut self, entity: IGESEntity) -> bool {
        if self.entity_map.contains_key(&entity.id()) {
            return false;
        }

        self.entity_map.insert(entity.id(), entity.id());

        // Determine the view this entity is attached to
        let view_index = if entity.type_number() == IGES_DRAWING_TYPE {
            // Drawing entities are their own view
            let idx = self.items_map.len() + 1;
            self.items_map.insert(idx, entity.clone());
            idx
        } else if entity.is_view_kind() {
            // View kind entities are their own view
            let idx = self.items_map.len() + 1;
            self.items_map.insert(idx, entity.clone());
            idx
        } else {
            // Other entities: for now, treat as having no specific view
            0
        };

        self.entity_indices.push(view_index);
        self.final_indices.push(0);

        true
    }

    /// Returns the count of recorded entities
    pub fn nb_entities(&self) -> usize {
        self.entity_map.len()
    }

    /// Sorts entities by single views, optionally including drawings
    pub fn sort_single_views(&mut self, also_frames: bool) {
        self.finals_map.clear();

        for (i, &item_index) in self.entity_indices.iter().enumerate() {
            let mut final_index = 0;

            if item_index > 0 {
                if let Some(item) = self.items_map.get(&item_index) {
                    let mut ok = false;

                    if also_frames && item.type_number() == IGES_DRAWING_TYPE {
                        ok = true;
                    }

                    if !ok && item.is_single() {
                        ok = true;
                    }

                    if ok {
                        let idx = self.finals_map.len() + 1;
                        if !self
                            .finals_map
                            .iter()
                            .any(|(_, e)| e.id() == item.id())
                        {
                            self.finals_map.insert(idx, item.clone());
                            final_index = idx;
                        } else {
                            // Find existing index
                            for (k, e) in &self.finals_map {
                                if e.id() == item.id() {
                                    final_index = *k;
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if i < self.final_indices.len() {
                self.final_indices[i] = final_index;
            }
        }
    }

    /// Sorts entities by drawings and their referenced views
    pub fn sort_drawings(&mut self, _graph: &InterfaceGraph) {
        self.finals_map.clear();

        for (i, &item_index) in self.entity_indices.iter().enumerate() {
            let mut final_index = 0;

            if item_index > 0 {
                if let Some(item) = self.items_map.get(&item_index) {
                    let drawing = if item.type_number() == IGES_DRAWING_TYPE {
                        Some(item.clone())
                    } else {
                        // In real implementation, would use graph to find containing drawing
                        None
                    };

                    if let Some(draw) = drawing {
                        let idx = self.finals_map.len() + 1;
                        if !self.finals_map.iter().any(|(_, e)| e.id() == draw.id()) {
                            self.finals_map.insert(idx, draw);
                            final_index = idx;
                        } else {
                            for (k, e) in &self.finals_map {
                                if e.id() == draw.id() {
                                    final_index = *k;
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if i < self.final_indices.len() {
                self.final_indices[i] = final_index;
            }
        }
    }

    /// Returns the count of sets
    pub fn nb_sets(&self, final_sets: bool) -> usize {
        if final_sets {
            self.finals_map.len()
        } else {
            self.items_map.len()
        }
    }

    /// Returns the entity attached to a set
    pub fn set_item(&self, num: usize, final_sets: bool) -> Option<IGESEntity> {
        if final_sets {
            self.finals_map.get(&num).cloned()
        } else {
            self.items_map.get(&num).cloned()
        }
    }

    /// Returns a packet list of sets and their contents
    pub fn sets(&self, final_sets: bool) -> PacketList {
        let mut list = PacketList::new();

        let nb = if final_sets {
            self.final_indices.len()
        } else {
            self.entity_indices.len()
        };

        let nbs = self.nb_sets(final_sets);

        for num in 1..=nbs {
            list.add_packet();

            for i in 0..nb {
                let idx = if final_sets {
                    self.final_indices.get(i).copied().unwrap_or(0)
                } else {
                    self.entity_indices.get(i).copied().unwrap_or(0)
                };

                if idx == num {
                    if let Some(ent_id) = self
                        .entity_map
                        .iter()
                        .nth(i)
                        .map(|(id, _)| *id)
                    {
                        list.add_entity(ent_id);
                    }
                }
            }
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iges_entity_creation() {
        let entity = IGESEntity::new(1, 200, false);
        assert_eq!(entity.id(), 1);
        assert_eq!(entity.type_number(), 200);
        assert!(!entity.is_view_kind());
    }

    #[test]
    fn test_iges_entity_drawing() {
        let entity = IGESEntity::new(1, IGES_DRAWING_TYPE, true);
        assert_eq!(entity.type_number(), IGES_DRAWING_TYPE);
    }

    #[test]
    fn test_iges_entity_is_single() {
        let view = IGESEntity::new(1, 200, true);
        assert!(view.is_single());

        let drawing = IGESEntity::new(2, IGES_DRAWING_TYPE, true);
        assert!(!drawing.is_single());
    }

    #[test]
    fn test_view_sorter_creation() {
        let sorter = IGESSelectViewSorter::new();
        assert_eq!(sorter.nb_entities(), 0);
    }

    #[test]
    fn test_add_entity() {
        let mut sorter = IGESSelectViewSorter::new();
        let entity = IGESEntity::new(1, 200, true);

        let result = sorter.add_entity(entity);
        assert!(result);
        assert_eq!(sorter.nb_entities(), 1);
    }

    #[test]
    fn test_add_duplicate_entity() {
        let mut sorter = IGESSelectViewSorter::new();
        let entity = IGESEntity::new(1, 200, true);

        sorter.add_entity(entity.clone());
        let result = sorter.add_entity(entity);

        assert!(!result);
        assert_eq!(sorter.nb_entities(), 1);
    }

    #[test]
    fn test_clear() {
        let mut sorter = IGESSelectViewSorter::new();
        sorter.add_entity(IGESEntity::new(1, 200, true));

        sorter.clear();
        assert_eq!(sorter.nb_entities(), 0);
    }

    #[test]
    fn test_sort_single_views() {
        let mut sorter = IGESSelectViewSorter::new();
        let entity = IGESEntity::new(1, 200, true);
        sorter.add_entity(entity);

        sorter.sort_single_views(false);
        assert_eq!(sorter.nb_sets(true), 1);
    }

    #[test]
    fn test_sort_single_views_with_frames() {
        let mut sorter = IGESSelectViewSorter::new();
        let drawing = IGESEntity::new(1, IGES_DRAWING_TYPE, true);
        sorter.add_entity(drawing);

        sorter.sort_single_views(true);
        assert_eq!(sorter.nb_sets(true), 1);
    }

    #[test]
    fn test_sort_single_views_without_frames() {
        let mut sorter = IGESSelectViewSorter::new();
        let drawing = IGESEntity::new(1, IGES_DRAWING_TYPE, true);
        sorter.add_entity(drawing);

        sorter.sort_single_views(false);
        assert_eq!(sorter.nb_sets(true), 0);
    }

    #[test]
    fn test_set_item() {
        let mut sorter = IGESSelectViewSorter::new();
        let entity = IGESEntity::new(42, 200, true);
        sorter.add_entity(entity);
        sorter.sort_single_views(false);

        let item = sorter.set_item(1, true);
        assert!(item.is_some());
        assert_eq!(item.unwrap().id(), 42);
    }

    #[test]
    fn test_interface_graph() {
        let mut graph = InterfaceGraph::new();
        graph.add_sharing(1, 10);
        graph.add_sharing(1, 11);

        let sharings = graph.sharings(1);
        assert_eq!(sharings.len(), 2);
        assert!(sharings.contains(&10));
    }

    #[test]
    fn test_packet_list() {
        let mut list = PacketList::new();
        list.add_packet();
        list.add_entity(1);
        list.add_entity(2);
        list.add_packet();
        list.add_entity(3);

        assert_eq!(list.packets().len(), 2);
        assert_eq!(list.packets()[0].len(), 2);
        assert_eq!(list.packets()[1].len(), 1);
    }

    #[test]
    fn test_nb_sets_initial() {
        let sorter = IGESSelectViewSorter::new();
        assert_eq!(sorter.nb_sets(false), 0);
        assert_eq!(sorter.nb_sets(true), 0);
    }

    #[test]
    fn test_multiple_entities_same_view() {
        let mut sorter = IGESSelectViewSorter::new();
        let view1 = IGESEntity::new(1, 200, true);
        let entity1 = IGESEntity::new(2, 300, false);

        sorter.add_entity(view1);
        sorter.add_entity(entity1);

        sorter.sort_single_views(false);
        assert_eq!(sorter.nb_sets(true), 1);
    }
}
