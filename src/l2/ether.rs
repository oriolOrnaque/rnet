use std::str::FromStr;

use bytes::{BytesMut, Buf, BufMut, Bytes};
use mac_address::{MacAddress};

#[derive(Debug, Clone, Copy)]
pub enum EtherType {
    IPv4 = 0x0800,
    IPv6 = 0x86DD,
    ARP = 0x0806,
}

#[derive(Debug)]
pub struct EtherHdr {
    dest: MacAddress,
    src: MacAddress,
    ethertype: EtherType,
}

impl EtherHdr {
    pub fn from_str(dest: &str, src: &str, ethertype: EtherType) -> Self {
        EtherHdr {
            dest: MacAddress::from_str(dest).unwrap(),
            src: MacAddress::from_str(src).unwrap(),
            ethertype: ethertype,
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut bytes = BytesMut::with_capacity(6 + 6 + 2);
        bytes.put_slice(&self.dest.bytes());
        bytes.put_slice(&self.src.bytes());
        bytes.put_u16(self.ethertype as u16);
        bytes.freeze()
    }
}