#[derive(Debug)]
enum ip_address_kind {
    v4,
    v6,
}
#[derive(Debug)]
struct ipAddress {
    kind: ip_address_kind,
    address: String,
}

impl ipAddress {
    fn new(kind: ip_address_kind, address: String) -> Self {
        Self { kind, address }
    }
}

fn main() {
    let new_ip = ipAddress::new(ip_address_kind::v4, String::from("192.168.100.1"));
    print!("the ip is {:?}", new_ip);
}
