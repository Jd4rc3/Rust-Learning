fn main() {
    let home = IpAddrKind::V4(Ipv4Addr);
    let loopback = IpAddrKind::V6(Ipv6Addr);
}

enum IpAddrKind {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

struct Ipv4Addr {}

struct Ipv6Addr {}