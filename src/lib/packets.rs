#![allow(unused)]

use etherparse::{err::packet::BuildWriteError, IpNumber, PacketBuilder};

const SRC_MAC: [u8; 6] = [1, 2, 3, 4, 5, 6];
const DEST_MAC: [u8; 6] = [7, 8, 9, 10, 11, 12];
const SRC_IP: [u8; 4] = [192, 168, 1, 1];
const DEST_IP: [u8; 4] = [192, 168, 1, 2];
const TTL: u8 = 31;
const PROTO: IpNumber = IpNumber(17); // UDP
const SRC_PORT: u16 = 21;
const DEST_PORT: u16 = 1234;

pub fn p1() -> Result<Vec<u8>, BuildWriteError> {
    let builder = PacketBuilder::ethernet2(SRC_MAC, DEST_MAC)
        .ipv4(SRC_IP, DEST_IP, TTL)
        .udp(SRC_PORT, DEST_PORT);

    //payload of the udp packet
    let payload = [1, 2, 3, 4, 5, 6, 7, 8];

    let mut packet = Vec::<u8>::with_capacity(builder.size(payload.len()));

    builder.write(&mut packet, &payload).map(|_ok| packet)
}

#[cfg(test)]
mod tests {
    use etherparse::LinkSlice::Ethernet2;
    use etherparse::{
        EtherType, Ethernet2Slice, IpSlice, Ipv4Slice, NetSlice, PacketHeaders, PayloadSlice,
        SlicedPacket, TransportSlice, UdpHeader, UdpHeaderSlice, UdpSlice,
    };

    use super::*;

    #[test]
    fn get_p1() {
        let packet = p1().unwrap();

        // println!("p1() packet: {:?}", packet);

        let parsed = PacketHeaders::from_ethernet_slice(&packet).unwrap();

        // println!("ethernet: {:?}", parsed.link);
        // println!("ip: {:?}", parsed.net); // contains ip
        // println!("udp: {:?}", parsed.transport);

        if let Some(link) = parsed.link {
            assert_eq!(SRC_MAC, link.source);
            assert_eq!(DEST_MAC, link.destination);
            assert_eq!(EtherType::IPV4, link.ether_type);
        } else {
            assert!(false, "No link layer");
        }

        if let Some(etherparse::NetHeaders::Ipv4(headers, extensions)) = parsed.net {
            assert_eq!(SRC_IP, headers.source);
            assert_eq!(DEST_IP, headers.destination);
            assert_eq!(TTL, headers.time_to_live);
            assert_eq!(PROTO, headers.protocol);
        } else {
            assert!(false, "No IPV4 headers");
        }

        if let Some(etherparse::TransportHeader::Udp(udp)) = parsed.transport {
            assert_eq!(SRC_PORT, udp.source_port);
            assert_eq!(DEST_PORT, udp.destination_port);
        } else {
            assert!(false, "No UDP transport");
        }

        // Now the other way...
        let parsed = SlicedPacket::from_ethernet(&packet).unwrap();

        if let Some(Ethernet2(link)) = parsed.link {
            assert_eq!(SRC_MAC, link.source());
            assert_eq!(DEST_MAC, link.destination());
            assert_eq!(EtherType::IPV4, link.ether_type());
        } else {
            assert!(false, "No link layer");
        }

        if let Some(NetSlice::Ipv4(ipv4)) = parsed.net {
            assert_eq!(SRC_IP, ipv4.header().source());
            assert_eq!(DEST_IP, ipv4.header().destination());
            assert_eq!(TTL, ipv4.header().ttl());
            assert_eq!(PROTO, ipv4.header().protocol());
        } else {
            assert!(false, "No IPV4 headers");
        }

        if let Some(TransportSlice::Udp(udp)) = parsed.transport {
            assert_eq!(SRC_PORT, udp.source_port());
            assert_eq!(DEST_PORT, udp.destination_port());
        } else {
            assert!(false, "No UDP transport");
        }
    }

    #[test]
    fn udp_from_p1() {
        let packet = p1().unwrap();

        println!("p1() packet: {:?}", packet);

        let parsed = SlicedPacket::from_ethernet(&packet).unwrap();

        if let Some(TransportSlice::Udp(udp)) = parsed.transport {
            println!("udp: {:?}", udp);

            let udp = UdpHeaderSlice::from_slice(udp.slice()).unwrap();

            assert_eq!(SRC_PORT, udp.source_port());
            assert_eq!(DEST_PORT, udp.destination_port());
        }
    }

    #[test]
    fn udp_from_struct() {
        let udp = etherparse::UdpHeader {
            source_port: 21,
            destination_port: 1234,
            checksum: 0,
            length: 0,
        };

        let bytes = udp.to_bytes();

        let udp = UdpSlice::from_slice(&bytes);

        println!("udp: {:?}", udp);
    }
}
