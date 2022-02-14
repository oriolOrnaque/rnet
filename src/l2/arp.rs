use std::{net::{Ipv4Addr}, str::FromStr};

use bytes::{BytesMut, Buf, BufMut, Bytes};
use mac_address::{MacAddress};

#[derive(Debug, Clone, Copy)]
pub enum Htype {
    ETHER = 1,
}

impl Htype {
    fn hlen(&self) -> u8 {
        match self {
            &Self::ETHER => 6,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Ptype {
    IPv4 = 0x0800,
}

impl Ptype {
    fn plen(&self) -> u8 {
        match self {
            &Self::IPv4 => 4,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArpOp {
    REQUEST = 1,
    REPLY = 2,
}

#[derive(Debug)]
pub struct ArpHdr {
    htype: Htype,
    ptype: Ptype,
    hlen: u8,
    plen: u8,
    oper: ArpOp,
}

impl ArpHdr {
    pub fn new(htype: Htype, ptype: Ptype, op: ArpOp) -> Self {
        ArpHdr {
            htype: htype,
            ptype: ptype,
            hlen: htype.hlen(),
            plen: ptype.plen(),
            oper: op
        }
    }

    pub fn new_request(htype: Htype, ptype: Ptype) -> Self {
        ArpHdr::new(htype, ptype, ArpOp::REQUEST)
    }

    pub fn new_response(htype: Htype, ptype: Ptype) -> Self {
        ArpHdr::new(htype, ptype, ArpOp::REPLY)
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut bytes = BytesMut::with_capacity(2 + 2 + 1 + 1 + 2);
        bytes.put_u16(self.htype as u16);
        bytes.put_u16(self.ptype as u16);
        bytes.put_u8(self.hlen);
        bytes.put_u8(self.plen);
        bytes.put_u16(self.oper as u16);
        bytes.freeze()
    }
}

#[derive(Debug)]
pub struct ArpPayload {
    sha: MacAddress,
    spa: Ipv4Addr,
    tha: MacAddress,
    tpa: Ipv4Addr,
}

impl ArpPayload {
    pub fn new(sha: MacAddress, spa: Ipv4Addr, tha: MacAddress, tpa: Ipv4Addr) -> Self {
        ArpPayload { sha: sha, spa: spa, tha: tha, tpa: tpa }
    }

    pub fn from_str(sha: &str, spa: &str, tha: &str, tpa: &str) -> Self {
        ArpPayload::new(
            MacAddress::from_str(sha).unwrap(),
            Ipv4Addr::from_str(spa).unwrap(),
            MacAddress::from_str(tha).unwrap(),
            Ipv4Addr::from_str(tpa).unwrap(),
        )
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut bytes = BytesMut::with_capacity(6 + 4 + 6 + 4);
        bytes.put_slice(&self.sha.bytes());
        bytes.put_slice(&self.spa.octets());
        bytes.put_slice(&self.tha.bytes());
        bytes.put_slice(&self.tpa.octets());
        bytes.freeze()
    }
}