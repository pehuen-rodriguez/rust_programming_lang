struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;


    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Some kind {:#?}", loopback.kind);
    println!("Some address {:#?}", loopback.address);
}
