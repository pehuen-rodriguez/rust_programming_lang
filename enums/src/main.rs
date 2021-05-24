#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Some home address {:#?}", home);
    println!("Some loopback address {:#?}", loopback);

    let new_message = Message::Move { x: 12, y: 50 };
    println!("Gotta message: {:#?}", new_message);
    let new_message = Message::Write(String::from("Menssage"));
    println!("Gotta message: {:#?}", new_message);
    let new_message = Message::ChangeColor(12, 255, 255);
    println!("Gotta message: {:#?}", new_message);
    let new_message = Message::Quit;
    println!("Gotta message: {:#?}", new_message);
}
