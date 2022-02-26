#[derive(Debug, Clone, Copy)]
pub enum IpVersion {
    v4 = 4,
    v6 = 6,
}

#[derive(Debug, Clone, Copy)]
pub enum IpProtocol {
    // https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers
    ICMP = 1,
    TCP = 6,
    UDP = 17,
}
