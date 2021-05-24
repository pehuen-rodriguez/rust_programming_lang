#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Value in cents: {:?}", value_in_cents(Coin::Penny));
    println!("Value in cents: {:?}", value_in_cents(Coin::Nickel));
    println!("Value in cents: {:?}", value_in_cents(Coin::Dime));
    println!("Value in cents: {:?}", value_in_cents(Coin::Quarter));
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Luck Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
