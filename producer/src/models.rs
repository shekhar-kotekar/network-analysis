use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::{FromPacket, Packet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PacketInfo {
    source: String,
    destination: String,
    packet_type: String,
    packet_size: usize,
    from_packet: String,
    data: Vec<u8>,
}

impl PacketInfo {
    pub fn from(packet: EthernetPacket) -> Self {
        PacketInfo {
            source: format!("{:?}", packet.get_source()),
            destination: format!("{:?}", packet.get_destination()),
            packet_type: format!("{:?}", packet.get_ethertype()),
            packet_size: packet.packet().len(),
            from_packet: format!("{:?}", packet.from_packet().source),
            data: packet.payload().to_vec(),
        }
    }
}
