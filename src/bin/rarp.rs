extern crate rnet;

use std::str::FromStr;

use bytes::Buf;
use mac_address::MacAddress;
use rnet::l2::arp::{ArpHdr, Htype, Ptype, ArpPayload};
use rnet::socket::{Socket, Domain, Type, Protocol};
use rnet::l2::ether::{EtherType, EtherHdr};

fn main() {
    println!("Hello world");

    let ether_hdr = EtherHdr::from_str("aa:bb:cc:dd:ee:ff", "11:22:33:44:55:66", EtherType::ARP);
    let arp_hdr = ArpHdr::new_request(Htype::ETHER, Ptype::IPv4);
    let arp_payload = ArpPayload::from_str("aa:bb:cc:dd:ee:ff", "192.168.1.1", "11:22:33:44:55:66", "192.168.1.2");

    let packet = ether_hdr.to_bytes().chain(arp_hdr.to_bytes()).chain(arp_payload.to_bytes());

    println!("{:?}", ether_hdr);
    println!("{:?}", arp_hdr);
    println!("{:?}", arp_payload);
    println!("{:?}", packet);

    match Socket::new(Domain::Packet, Type::Raw, Protocol::Arp) {
        Ok(socket) => {
            socket.send_to(packet);
        },
        Err(e) => println!("{}", e),
    }
}