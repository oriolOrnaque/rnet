use libc::{socket, close, AF_PACKET, SOCK_RAW, SOCK_DGRAM, c_int, ETH_P_ALL, ETH_P_ARP, ETH_P_IP};

#[derive(Debug, Clone, Copy)]
pub enum Domain {
    Packet = AF_PACKET as isize,
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Raw = SOCK_RAW as isize,
    Dgram = SOCK_DGRAM as isize,
}

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    Ether = ETH_P_ALL as isize,
    Arp = ETH_P_ARP as isize,
    IPv4 = ETH_P_IP as isize,
}

#[derive(Debug)]
pub struct Socket {
    domain: Domain,
    ty: Type,
    protocol: Protocol,
    fd: i32,
}

impl Socket {
    pub fn new(domain: Domain, ty: Type, protocol: Protocol) -> std::io::Result<Self> {
        unsafe {
            match socket(domain as c_int, ty as c_int, (protocol as c_int).to_be()) {
                -1 => Err(std::io::Error::last_os_error()),
                fd => Ok(
                    Socket {
                        domain: domain,
                        ty: ty,
                        protocol: protocol,
                        fd: fd,
                    }
                ),
            }
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe {
            close(self.fd)
        };
    }
}