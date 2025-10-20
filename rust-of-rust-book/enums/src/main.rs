enum IpAddrKind {
    v4,
    v6,
}

enum IpAddrWithData {
    V4(u8, u8, u8, u8),
    V6(String),
}

// A Message enum whose variants each store different amounts and types of values
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn route(ip_kind: IpAddrKind) {}
fn main() {
    println!("ripping enums!");

    let home = IpAddrWithData::V4(127, 0, 0, 1);
    let loopback = IpAddrWithData::V6(String::from("::1"));
    
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    route(IpAddrKind::v4);
    route(IpAddrKind::v4);

}
