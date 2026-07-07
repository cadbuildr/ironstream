// FILE: if_select_packet_list.rs
// occt: IFSelect_PacketList

#[derive(Clone, Debug)]
pub struct IfSelectPacketList {
    packets: Vec<Vec<usize>>,
}

impl IfSelectPacketList {
    pub fn new() -> Self {
        IfSelectPacketList {
            packets: vec![],
        }
    }

    pub fn add_packet(&mut self) {
        self.packets.push(vec![]);
    }

    pub fn add_to_packet(&mut self, entity: usize) {
        if let Some(packet) = self.packets.last_mut() {
            packet.push(entity);
        }
    }

    pub fn packet_count(&self) -> usize {
        self.packets.len()
    }
}

impl Default for IfSelectPacketList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let pl = IfSelectPacketList::new();
        assert_eq!(pl.packet_count(), 0);
    }

    #[test]
    fn test_add_packet() {
        let mut pl = IfSelectPacketList::new();
        pl.add_packet();
        assert_eq!(pl.packet_count(), 1);
    }

    #[test]
    fn test_add_to_packet() {
        let mut pl = IfSelectPacketList::new();
        pl.add_packet();
        pl.add_to_packet(1);
        assert_eq!(pl.packet_count(), 1);
    }
}
