use std::net::Ipv4Addr;

use bytes::{Bytes, Buf, BufMut, BytesMut};

use super::ip::{IpVersion, IpProtocol};


#[derive(Debug)]
pub struct IPv4 {
    version: IpVersion,
    ihl: u8,
    dscp: u8,
    ecn: u8,
    total_length: u16,
    id: u16,
    flags: u8,
    fragment_offset: u16,
    ttl: u8,
    protocol: IpProtocol,
    checksum: u16,
    src: Ipv4Addr,
    dest: Ipv4Addr,
}

impl IPv4 {
    pub fn new(src: Ipv4Addr, dest: Ipv4Addr, protocol: IpProtocol) -> Self {
        Self {
            version: IpVersion::v4,
            ihl: 5,
            dscp: 0,
            ecn: 0,
            total_length: 20,
            id: 1,
            flags: 0,
            fragment_offset: 0,
            ttl: 64,
            protocol: protocol,
            checksum: 0,
            src: src,
            dest: dest,
        }
    }

    fn checksum(bytes: &Bytes, verify: bool) -> u16 {
        let mut bytes = bytes.clone();

        let version_ihl_dscp_ecn = bytes.get_u16();
        let total_length = bytes.get_u16();
        let id = bytes.get_u16();
        let flags_fragment_offset = bytes.get_u16();
        let ttl_protocol = bytes.get_u16();
        let _checksum = bytes.get_u16();
        let src0 = bytes.get_u16();
        let src1 = bytes.get_u16();
        let dest0 = bytes.get_u16();
        let dest1 = bytes.get_u16();

        let mut sum = (version_ihl_dscp_ecn as u32 + total_length as u32 + id as u32
            + flags_fragment_offset as u32 + ttl_protocol as u32 + src0 as u32 + src1 as u32
            + dest0 as u32 + dest1 as u32 ) as u32;
        if verify {
            sum += _checksum as u32;
        }
        let mut carry_count = ((sum & 0xffff0000) >> 16) as u16;

        while carry_count > 0 {
            sum = (sum as u16 + carry_count) as u32;
            carry_count = ((sum & 0xffff0000) >> 16) as u16;
        }

        (sum & 0x0000ffff) as u16
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut bytes = BytesMut::with_capacity(2 * 4);

        let version_ihl = (((self.version as u8) & 0x0f) << 4) + (self.ihl & 0x0f);
        let dscp_ecn = ((self.dscp & 0x3f) << 2) + (self.ecn & 0x04);
        let flags_fragment_offset = (((self.flags as u16) & 0x0003) << 13) + (self.fragment_offset & 0x1fff);

        bytes.put_u8(version_ihl);
        bytes.put_u8(dscp_ecn);
        bytes.put_u16(self.total_length);
        bytes.put_u16(self.id);
        bytes.put_u16(flags_fragment_offset);
        bytes.put_u8(self.ttl);
        bytes.put_u8(self.protocol as u8);
        bytes.put_u16(self.checksum);
        bytes.put_slice(&self.src.octets());
        bytes.put_slice(&self.dest.octets());

        bytes.freeze()
    }
}

#[cfg(test)]
mod test_ipv4 {
    use std::str::FromStr;

    use bytes::BytesMut;
    use super::*;

    #[test]
    fn qwe() {
        let asd = IPv4::new(Ipv4Addr::from_str("127.0.0.1").unwrap(),
        Ipv4Addr::from_str("127.0.0.2").unwrap(), IpProtocol::UDP);

        println!("{:x?}", asd.to_bytes());
    }

    #[test]
    fn compute_checksum() {
        let mut bytes = BytesMut::with_capacity(20);
        bytes.put_u16(0x4500);
        bytes.put_u16(0x0073);
        bytes.put_u16(0x0000);
        bytes.put_u16(0x4000);
        bytes.put_u16(0x4011);
        bytes.put_u16(0x0000);
        bytes.put_u16(0xc0a8);
        bytes.put_u16(0x0001);
        bytes.put_u16(0xc0a8);
        bytes.put_u16(0x00c7);

        assert_eq!(0x479e, IPv4::checksum(&bytes.freeze(), false));
    }

    #[test]
    fn verify_checksum() {
        let mut bytes = BytesMut::with_capacity(20);
        bytes.put_u16(0x4500);
        bytes.put_u16(0x0073);
        bytes.put_u16(0x0000);
        bytes.put_u16(0x4000);
        bytes.put_u16(0x4011);
        bytes.put_u16(0xb861);
        bytes.put_u16(0xc0a8);
        bytes.put_u16(0x0001);
        bytes.put_u16(0xc0a8);
        bytes.put_u16(0x00c7);

        assert_eq!(0xffff, IPv4::checksum(&bytes.freeze(), true));
    }
}
