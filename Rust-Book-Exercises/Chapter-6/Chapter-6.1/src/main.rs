enum IpAddrKindEnum { V4, V6 }
struct IpAddrStruct { kind: IpAddrKindEnum, address: String }
enum IpAddrEnumVariant { V4(String), V6(String) }
enum IpAddrEnumTypeVariant { V4(u8, u8, u8, u8), V6(String) }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) { println!("Calling..."); }
}

enum Option<T> { None, Some(T) }

fn main() {
    let _ip_type = IpAddrKindEnum::V4;

    let _home = IpAddrStruct { kind: IpAddrKindEnum::V4, address: String::from("127.0.0.1") };
    let _loopback = IpAddrStruct { kind: IpAddrKindEnum::V6, address: String::from("::1") };

    let _home2 = IpAddrEnumVariant::V4(String::from("127.0.0.1"));
    let _loopback2 = IpAddrEnumVariant::V6(String::from("::1"));

    let _home3 = IpAddrEnumTypeVariant::V4(127, 0, 0, 1);
    let _loopback3 = IpAddrEnumTypeVariant::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = Option::None;
}