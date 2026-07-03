// FILE: ais_conn.rs
// occt: AIS_ConnectedInteractive, AIS_MultipleConnectedInteractive

/// Synchronisation status of a connected object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectStatus {
    Connected,
    NotConnected,
    TransformOutOfSync,
    Error,
}

impl Default for ConnectStatus {
    fn default() -> Self { Self::NotConnected }
}

/// A 4×4 homogeneous transform matrix (row-major).
#[derive(Clone, Debug)]
pub struct ConnTransform {
    pub matrix: [[f64; 4]; 4],
}

impl Default for ConnTransform {
    fn default() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl ConnTransform {
    pub fn identity() -> Self { Self::default() }

    pub fn translation(tx: f64, ty: f64, tz: f64) -> Self {
        let mut m = Self::default();
        m.matrix[0][3] = tx;
        m.matrix[1][3] = ty;
        m.matrix[2][3] = tz;
        m
    }

    pub fn is_identity(&self) -> bool {
        let id = Self::identity().matrix;
        self.matrix == id
    }

    pub fn translation_part(&self) -> [f64; 3] {
        [self.matrix[0][3], self.matrix[1][3], self.matrix[2][3]]
    }

    pub fn apply_to_point(&self, p: [f64; 3]) -> [f64; 3] {
        let m = &self.matrix;
        [
            m[0][0]*p[0] + m[0][1]*p[1] + m[0][2]*p[2] + m[0][3],
            m[1][0]*p[0] + m[1][1]*p[1] + m[1][2]*p[2] + m[1][3],
            m[2][0]*p[0] + m[2][1]*p[1] + m[2][2]*p[2] + m[2][3],
        ]
    }

    pub fn composed_with(&self, other: &ConnTransform) -> ConnTransform {
        let a = &self.matrix;
        let b = &other.matrix;
        let mut r = [[0.0f64; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    r[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        ConnTransform { matrix: r }
    }
}

// occt: AIS_ConnectedInteractive — displays a referenced object at a transformed position
#[derive(Clone, Debug)]
pub struct AisConnectedInteractive {
    pub id: u32,
    pub referenced_id: Option<u32>,
    pub transform: ConnTransform,
    pub status: ConnectStatus,
    pub is_displayed: bool,
    pub z_layer: i32,
}

impl AisConnectedInteractive {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            referenced_id: None,
            transform: ConnTransform::identity(),
            status: ConnectStatus::NotConnected,
            is_displayed: false,
            z_layer: 0,
        }
    }

    pub fn connect(&mut self, referenced_id: u32) {
        self.referenced_id = Some(referenced_id);
        self.status = ConnectStatus::Connected;
    }

    pub fn disconnect(&mut self) {
        self.referenced_id = None;
        self.status = ConnectStatus::NotConnected;
    }

    pub fn set_transform(&mut self, transform: ConnTransform) {
        self.transform = transform;
        if self.status == ConnectStatus::Connected {
            self.status = ConnectStatus::Connected;
        }
    }

    pub fn sync_transform(&mut self) {
        if self.status == ConnectStatus::TransformOutOfSync {
            self.status = ConnectStatus::Connected;
        }
    }

    pub fn display(&mut self) { self.is_displayed = true; }
    pub fn erase(&mut self) { self.is_displayed = false; }
    pub fn is_connected(&self) -> bool { self.status == ConnectStatus::Connected }
    pub fn has_owner(&self) -> bool { self.referenced_id.is_some() }
}

/// A single connection entry in a multiple-connected object.
#[derive(Clone, Debug)]
pub struct ConnectionEntry {
    pub referenced_id: u32,
    pub transform: ConnTransform,
    pub connection_id: u32,
}

// occt: AIS_MultipleConnectedInteractive — aggregates several connected references
#[derive(Clone, Debug, Default)]
pub struct AisMultipleConnected {
    pub id: u32,
    pub connections: Vec<ConnectionEntry>,
    pub is_displayed: bool,
    pub z_layer: i32,
    next_conn_id: u32,
}

impl AisMultipleConnected {
    pub fn new(id: u32) -> Self { Self { id, next_conn_id: 1, ..Default::default() } }

    pub fn add_connection(&mut self, referenced_id: u32, transform: ConnTransform) -> u32 {
        let cid = self.next_conn_id;
        self.next_conn_id += 1;
        self.connections.push(ConnectionEntry { referenced_id, transform, connection_id: cid });
        cid
    }

    pub fn remove_connection(&mut self, connection_id: u32) -> bool {
        let before = self.connections.len();
        self.connections.retain(|c| c.connection_id != connection_id);
        self.connections.len() < before
    }

    pub fn connection(&self, connection_id: u32) -> Option<&ConnectionEntry> {
        self.connections.iter().find(|c| c.connection_id == connection_id)
    }

    pub fn nb_connections(&self) -> usize { self.connections.len() }
    pub fn is_empty(&self) -> bool { self.connections.is_empty() }
    pub fn display(&mut self) { self.is_displayed = true; }
    pub fn erase(&mut self) { self.is_displayed = false; }

    pub fn referenced_ids(&self) -> Vec<u32> {
        self.connections.iter().map(|c| c.referenced_id).collect()
    }

    pub fn update_transform(&mut self, connection_id: u32, transform: ConnTransform) -> bool {
        if let Some(c) = self.connections.iter_mut().find(|c| c.connection_id == connection_id) {
            c.transform = transform;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conn_transform_identity() {
        let t = ConnTransform::identity();
        assert!(t.is_identity());
        let p = t.apply_to_point([1.0, 2.0, 3.0]);
        assert_eq!(p, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn conn_transform_translation() {
        let t = ConnTransform::translation(5.0, 0.0, 0.0);
        let p = t.apply_to_point([1.0, 0.0, 0.0]);
        assert!((p[0] - 6.0).abs() < 1e-10);
    }

    #[test]
    fn conn_transform_compose() {
        let t1 = ConnTransform::translation(1.0, 0.0, 0.0);
        let t2 = ConnTransform::translation(0.0, 2.0, 0.0);
        let t3 = t1.composed_with(&t2);
        let p = t3.apply_to_point([0.0, 0.0, 0.0]);
        assert!((p[0] - 1.0).abs() < 1e-10);
        assert!((p[1] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn ais_connected_interactive() {
        let mut obj = AisConnectedInteractive::new(1);
        assert!(!obj.is_connected());
        obj.connect(42);
        assert!(obj.is_connected());
        obj.set_transform(ConnTransform::translation(1.0, 2.0, 3.0));
        obj.display();
        assert!(obj.is_displayed);
        obj.disconnect();
        assert!(!obj.is_connected());
    }

    #[test]
    fn ais_multiple_connected() {
        let mut mc = AisMultipleConnected::new(10);
        let cid1 = mc.add_connection(1, ConnTransform::identity());
        let cid2 = mc.add_connection(2, ConnTransform::translation(5.0, 0.0, 0.0));
        assert_eq!(mc.nb_connections(), 2);
        mc.remove_connection(cid1);
        assert_eq!(mc.nb_connections(), 1);
        assert!(mc.connection(cid2).is_some());
    }

    #[test]
    fn ais_multiple_update_transform() {
        let mut mc = AisMultipleConnected::new(1);
        let cid = mc.add_connection(5, ConnTransform::identity());
        let ok = mc.update_transform(cid, ConnTransform::translation(1.0, 2.0, 3.0));
        assert!(ok);
        let c = mc.connection(cid).unwrap();
        assert!(!c.transform.is_identity());
    }
}
