fn main() {
    let v = vec![
        IpAddr::V4(String::from("127.0.0.1")),
        IpAddr::V6(String::from("::1")),
    ];
    for ip in &v {
        show_addr(ip);
    }
}
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn show_addr(addr: &IpAddr) {
    println!("{:?}", addr);
}
