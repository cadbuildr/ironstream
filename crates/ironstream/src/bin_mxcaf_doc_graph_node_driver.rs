// FILE: bin_mxcaf_doc_graph_node_driver.rs
// occt: BinMXCAFDoc_GraphNodeDriver
//
// Faithful port of OCCT BinMXCAFDoc_GraphNodeDriver
// (BinMXCAFDoc_GraphNodeDriver.cxx). Payload layout per the .cxx:
//   { <Integer fatherId> }*  <Integer -1>
//   { <Integer childId>  }*  <Integer -1>
//   <GUID graphId>
// Father/child ids are 1-based indices produced by the write relocation
// table (NCollection_IndexedMap::Add); on read they are resolved or created
// through the read relocation table (BinObjMgt_RRelocationTable Bind/Find).
//
// The OCAF plumbing is modelled locally:
//   - `GnodePersistentStream`  ~ BinObjMgt_Persistent (Integer + GUID subset,
//     big-endian file byte order, 4-byte word alignment),
//   - `GnodeArena`             ~ the pool of XCAFDoc_GraphNode attributes,
//   - `GnodeWRelocTable`       ~ NCollection_IndexedMap<handle> (write side),
//   - `GnodeRRelocTable`       ~ BinObjMgt_RRelocationTable (read side).

use std::collections::HashMap;

/// Local model of Standard_GUID as stored by BinObjMgt_Persistent::PutGUID:
/// Data1 (u32), Data2 (u16), Data3 (u16), Data4 (8 bytes), each scalar in
/// file byte order (big-endian).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GnodeGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl GnodeGuid {
    pub const fn nil() -> Self {
        GnodeGuid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        }
    }
}

/// Local model of one XCAFDoc_GraphNode attribute: father/child links are
/// arena indices (handles), plus the graph GUID.
#[derive(Debug, Clone, PartialEq)]
pub struct GnodeAttribute {
    fathers: Vec<usize>,
    children: Vec<usize>,
    graph_id: GnodeGuid,
}

impl GnodeAttribute {
    pub fn new_empty() -> Self {
        GnodeAttribute {
            fathers: Vec::new(),
            children: Vec::new(),
            graph_id: GnodeGuid::nil(),
        }
    }

    pub fn nb_fathers(&self) -> usize {
        self.fathers.len()
    }

    pub fn nb_children(&self) -> usize {
        self.children.len()
    }

    /// 1-based like XCAFDoc_GraphNode::GetFather(i).
    pub fn get_father(&self, i: usize) -> usize {
        self.fathers[i - 1]
    }

    /// 1-based like XCAFDoc_GraphNode::GetChild(i).
    pub fn get_child(&self, i: usize) -> usize {
        self.children[i - 1]
    }

    /// Mirrors XCAFDoc_GraphNode::SetFather(node).
    pub fn set_father(&mut self, node: usize) {
        self.fathers.push(node);
    }

    /// Mirrors XCAFDoc_GraphNode::SetChild(node).
    pub fn set_child(&mut self, node: usize) {
        self.children.push(node);
    }

    /// Mirrors XCAFDoc_GraphNode::SetGraphID(guid).
    pub fn set_graph_id(&mut self, guid: GnodeGuid) {
        self.graph_id = guid;
    }

    /// Mirrors XCAFDoc_GraphNode::ID().
    pub fn id(&self) -> GnodeGuid {
        self.graph_id
    }
}

/// Pool of graph-node attributes; indices play the role of handles.
#[derive(Debug, Default)]
pub struct GnodeArena {
    pub nodes: Vec<GnodeAttribute>,
}

impl GnodeArena {
    pub fn new() -> Self {
        GnodeArena { nodes: Vec::new() }
    }

    /// Mirrors `NewEmpty()` allocating a fresh attribute; returns its handle.
    pub fn new_empty(&mut self) -> usize {
        self.nodes.push(GnodeAttribute::new_empty());
        self.nodes.len() - 1
    }
}

/// Write relocation table ~ NCollection_IndexedMap<handle<Standard_Transient>>:
/// Add() returns the existing 1-based index or appends a new one.
#[derive(Debug, Default)]
pub struct GnodeWRelocTable {
    index_of: HashMap<usize, i32>,
    order: Vec<usize>,
}

impl GnodeWRelocTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// NCollection_IndexedMap::Add — 1-based index.
    pub fn add(&mut self, handle: usize) -> i32 {
        if let Some(&id) = self.index_of.get(&handle) {
            return id;
        }
        self.order.push(handle);
        let id = self.order.len() as i32;
        self.index_of.insert(handle, id);
        id
    }

    pub fn extent(&self) -> usize {
        self.order.len()
    }
}

/// Read relocation table ~ BinObjMgt_RRelocationTable (IsBound/Find/Bind).
#[derive(Debug, Default)]
pub struct GnodeRRelocTable {
    bound: HashMap<i32, usize>,
}

impl GnodeRRelocTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_bound(&self, id: i32) -> bool {
        self.bound.contains_key(&id)
    }

    pub fn find(&self, id: i32) -> usize {
        self.bound[&id]
    }

    pub fn bind(&mut self, id: i32, handle: usize) {
        self.bound.insert(id, handle);
    }
}

/// Local stand-in for BinObjMgt_Persistent (Integer + GUID subset).
pub struct GnodePersistentStream {
    data: Vec<u8>,
    pos: usize,
    err: bool,
}

impl GnodePersistentStream {
    pub fn new() -> Self {
        GnodePersistentStream {
            data: Vec::new(),
            pos: 0,
            err: false,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        GnodePersistentStream {
            data: bytes.to_vec(),
            pos: 0,
            err: false,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn is_error(&self) -> bool {
        self.err
    }

    fn align_put(&mut self, n: usize) {
        while self.data.len() % n != 0 {
            self.data.push(0);
        }
    }

    fn align_get(&mut self, n: usize) {
        while self.pos % n != 0 {
            self.pos += 1;
        }
    }

    /// BinObjMgt_Persistent::PutInteger.
    pub fn put_integer(&mut self, v: i32) {
        self.align_put(4);
        self.data.extend_from_slice(&v.to_be_bytes());
    }

    /// BinObjMgt_Persistent::GetInteger.
    pub fn get_integer(&mut self) -> Option<i32> {
        self.align_get(4);
        if self.pos + 4 > self.data.len() {
            self.err = true;
            return None;
        }
        let mut b = [0u8; 4];
        b.copy_from_slice(&self.data[self.pos..self.pos + 4]);
        self.pos += 4;
        Some(i32::from_be_bytes(b))
    }

    /// BinObjMgt_Persistent::PutGUID — word-aligned, 16 bytes total.
    pub fn put_guid(&mut self, g: &GnodeGuid) {
        self.align_put(4);
        self.data.extend_from_slice(&g.data1.to_be_bytes());
        self.data.extend_from_slice(&g.data2.to_be_bytes());
        self.data.extend_from_slice(&g.data3.to_be_bytes());
        self.data.extend_from_slice(&g.data4);
    }

    /// BinObjMgt_Persistent::GetGUID.
    pub fn get_guid(&mut self) -> Option<GnodeGuid> {
        self.align_get(4);
        if self.pos + 16 > self.data.len() {
            self.err = true;
            return None;
        }
        let b = &self.data[self.pos..self.pos + 16];
        let g = GnodeGuid {
            data1: u32::from_be_bytes([b[0], b[1], b[2], b[3]]),
            data2: u16::from_be_bytes([b[4], b[5]]),
            data3: u16::from_be_bytes([b[6], b[7]]),
            data4: [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]],
        };
        self.pos += 16;
        Some(g)
    }
}

impl Default for GnodePersistentStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of BinMXCAFDoc_GraphNodeDriver.
pub struct BinMXCAFDocGraphNodeDriver {
    name: String,
}

impl BinMXCAFDocGraphNodeDriver {
    pub fn new() -> Self {
        BinMXCAFDocGraphNodeDriver {
            name: "XCAFDoc_GraphNode".to_string(),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.name
    }

    /// Mirrors Paste(read). `target` is the handle of the attribute being
    /// filled inside `arena`; unresolved ids allocate fresh empty nodes bound
    /// into the read relocation table (exactly the .cxx logic).
    pub fn paste_read(
        &self,
        source: &mut GnodePersistentStream,
        arena: &mut GnodeArena,
        target: usize,
        reloc: &mut GnodeRRelocTable,
    ) -> bool {
        // Read Fathers
        let mut an_id = match source.get_integer() {
            Some(v) => v,
            None => return false,
        };
        while an_id != -1 {
            let node = if reloc.is_bound(an_id) {
                reloc.find(an_id)
            } else {
                let fresh = arena.new_empty();
                reloc.bind(an_id, fresh);
                fresh
            };
            arena.nodes[target].set_father(node);
            an_id = match source.get_integer() {
                Some(v) => v,
                None => return false,
            };
        }

        // Read Children
        an_id = match source.get_integer() {
            Some(v) => v,
            None => return false,
        };
        while an_id != -1 {
            let node = if reloc.is_bound(an_id) {
                reloc.find(an_id)
            } else {
                let fresh = arena.new_empty();
                reloc.bind(an_id, fresh);
                fresh
            };
            arena.nodes[target].set_child(node);
            an_id = match source.get_integer() {
                Some(v) => v,
                None => return false,
            };
        }

        // Graph id
        let guid = match source.get_guid() {
            Some(g) => g,
            None => return false,
        };
        arena.nodes[target].set_graph_id(guid);
        true
    }

    /// Mirrors Paste(write): father ids, -1, child ids, -1, GUID; ids come
    /// from the write relocation table.
    pub fn paste_write(
        &self,
        arena: &GnodeArena,
        source: usize,
        target: &mut GnodePersistentStream,
        reloc: &mut GnodeWRelocTable,
    ) {
        let node = &arena.nodes[source];

        // Write fathers
        for i in 1..=node.nb_fathers() {
            let id = reloc.add(node.get_father(i));
            target.put_integer(id);
        }
        target.put_integer(-1);

        // Write children
        for i in 1..=node.nb_children() {
            let id = reloc.add(node.get_child(i));
            target.put_integer(id);
        }
        target.put_integer(-1);

        // Graph id
        target.put_guid(&node.id());
    }
}

impl Default for BinMXCAFDocGraphNodeDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_guid() -> GnodeGuid {
        // XCAFDoc::AssemblyGUID-like value.
        GnodeGuid {
            data1: 0x5b896af8,
            data2: 0x3adf,
            data3: 0x11d6,
            data4: [0xa9, 0xcc, 0x00, 0xc0, 0x4f, 0x10, 0xdb, 0xaf],
        }
    }

    #[test]
    fn roundtrip_fathers_children_guid() {
        let driver = BinMXCAFDocGraphNodeDriver::new();

        // Source arena: node 0 with fathers {1, 2} and child {3}.
        let mut src_arena = GnodeArena::new();
        let n0 = src_arena.new_empty();
        let f1 = src_arena.new_empty();
        let f2 = src_arena.new_empty();
        let c1 = src_arena.new_empty();
        src_arena.nodes[n0].set_father(f1);
        src_arena.nodes[n0].set_father(f2);
        src_arena.nodes[n0].set_child(c1);
        src_arena.nodes[n0].set_graph_id(sample_guid());

        let mut wtable = GnodeWRelocTable::new();
        let mut stream = GnodePersistentStream::new();
        driver.paste_write(&src_arena, n0, &mut stream, &mut wtable);
        assert_eq!(wtable.extent(), 3); // f1, f2, c1

        // Read back into a fresh arena.
        let mut dst_arena = GnodeArena::new();
        let t0 = dst_arena.new_empty();
        let mut rtable = GnodeRRelocTable::new();
        let mut back = GnodePersistentStream::from_bytes(stream.bytes());
        assert!(driver.paste_read(&mut back, &mut dst_arena, t0, &mut rtable));

        let out = &dst_arena.nodes[t0];
        assert_eq!(out.nb_fathers(), 2);
        assert_eq!(out.nb_children(), 1);
        assert_eq!(out.id(), sample_guid());
        // Distinct persistent ids must map to distinct reconstructed nodes.
        assert_ne!(out.get_father(1), out.get_father(2));
        assert_ne!(out.get_father(1), out.get_child(1));
    }

    #[test]
    fn shared_ids_relocate_to_same_node() {
        // One node is both father and child: the write table must emit the
        // same id twice, and the read table must resolve both to one node.
        let driver = BinMXCAFDocGraphNodeDriver::new();

        let mut src_arena = GnodeArena::new();
        let n0 = src_arena.new_empty();
        let shared = src_arena.new_empty();
        src_arena.nodes[n0].set_father(shared);
        src_arena.nodes[n0].set_child(shared);
        src_arena.nodes[n0].set_graph_id(sample_guid());

        let mut wtable = GnodeWRelocTable::new();
        let mut stream = GnodePersistentStream::new();
        driver.paste_write(&src_arena, n0, &mut stream, &mut wtable);
        assert_eq!(wtable.extent(), 1);

        let mut dst_arena = GnodeArena::new();
        let t0 = dst_arena.new_empty();
        let mut rtable = GnodeRRelocTable::new();
        let mut back = GnodePersistentStream::from_bytes(stream.bytes());
        assert!(driver.paste_read(&mut back, &mut dst_arena, t0, &mut rtable));

        let out = &dst_arena.nodes[t0];
        assert_eq!(out.get_father(1), out.get_child(1));
        // Arena holds exactly target + one shared node.
        assert_eq!(dst_arena.nodes.len(), 2);
    }

    #[test]
    fn empty_node_roundtrip() {
        let driver = BinMXCAFDocGraphNodeDriver::new();
        let mut src_arena = GnodeArena::new();
        let n0 = src_arena.new_empty();
        src_arena.nodes[n0].set_graph_id(sample_guid());

        let mut wtable = GnodeWRelocTable::new();
        let mut stream = GnodePersistentStream::new();
        driver.paste_write(&src_arena, n0, &mut stream, &mut wtable);
        // -1, -1, GUID => 4 + 4 + 16 bytes
        assert_eq!(stream.bytes().len(), 24);

        let mut dst_arena = GnodeArena::new();
        let t0 = dst_arena.new_empty();
        let mut rtable = GnodeRRelocTable::new();
        let mut back = GnodePersistentStream::from_bytes(stream.bytes());
        assert!(driver.paste_read(&mut back, &mut dst_arena, t0, &mut rtable));
        assert_eq!(dst_arena.nodes[t0].nb_fathers(), 0);
        assert_eq!(dst_arena.nodes[t0].nb_children(), 0);
        assert_eq!(dst_arena.nodes[t0].id(), sample_guid());
    }

    #[test]
    fn truncated_stream_fails() {
        let driver = BinMXCAFDocGraphNodeDriver::new();
        let mut src_arena = GnodeArena::new();
        let n0 = src_arena.new_empty();
        src_arena.nodes[n0].set_graph_id(sample_guid());

        let mut wtable = GnodeWRelocTable::new();
        let mut stream = GnodePersistentStream::new();
        driver.paste_write(&src_arena, n0, &mut stream, &mut wtable);

        // Cut inside the GUID.
        let cut = &stream.bytes()[..stream.bytes().len() - 4];
        let mut dst_arena = GnodeArena::new();
        let t0 = dst_arena.new_empty();
        let mut rtable = GnodeRRelocTable::new();
        let mut back = GnodePersistentStream::from_bytes(cut);
        assert!(!driver.paste_read(&mut back, &mut dst_arena, t0, &mut rtable));
    }

    #[test]
    fn driver_metadata() {
        assert_eq!(
            BinMXCAFDocGraphNodeDriver::new().type_name(),
            "XCAFDoc_GraphNode"
        );
    }
}
