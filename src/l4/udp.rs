use bytes::{BytesMut, Bytes, BufMut, Buf};

#[derive(Debug)]
pub struct Udp {
    src: u16,
    dest: u16,
    length: u16,
    checksum: u16,
}

impl Udp {
    pub fn new(src: u16, dest: u16) -> Self {
        Self {
            src: src,
            dest: dest,
            length: 8,
            checksum: 0,
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut bytes = BytesMut::with_capacity(2 * 4);
        bytes.put_u16(self.src);
        bytes.put_u16(self.dest);
        bytes.put_u16(self.length);
        bytes.put_u16(self.checksum);
        bytes.freeze()
    }
}