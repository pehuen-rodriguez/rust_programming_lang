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

impl Message {
    fn print_me(&self) {
        println!("Gotta message: {:#?}", &self);
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Some home address {:#?}", home);
    println!("Some loopback address {:#?}", loopback);

    let new_message = Message::Move { x: 12, y: 50 };
    new_message.print_me();
    let new_message = Message::Write(String::from("Menssage"));
    new_message.print_me();
    let new_message = Message::ChangeColor(12, 255, 255);
    new_message.print_me();
    let new_message = Message::Quit;
    new_message.print_me();

    let five = Some(5);
    println!("five? {:?}", five);
    let a_string = Some("string");
    println!("a_string? {:?}", a_string);
    let absent_number: Option<i32> = None;
    println!("a_string? {:?}", absent_number);
}
