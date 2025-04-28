#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("ip kind: {:?}", ip_kind)
}

fn main() {
    // Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.
    // This is useful because now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind.
    // We can then, for instance, define a function that takes any IpAddrKind:
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    route(four);
    route(six);
}
