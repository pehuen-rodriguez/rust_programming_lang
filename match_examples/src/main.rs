#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

#[derive(Debug)]
enum UsStates {
    Arkansas,
    Alaska,
}

fn main() {
    println!("Value in cents: {:?}", value_in_cents(Coin::Penny));
    println!("Value in cents: {:?}", value_in_cents(Coin::Nickel));
    println!("Value in cents: {:?}", value_in_cents(Coin::Dime));
    println!(
        "Value in cents: {:?}",
        value_in_cents(Coin::Quarter(UsStates::Alaska))
    );
    println!(
        "Value in cents: {:?}",
        value_in_cents(Coin::Quarter(UsStates::Arkansas))
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Summed one {:?}", six);
    println!("Summed nottin {:?}", none);

    let some_u8_value: u8 = 0u8;
    match some_u8_value {
        1 => println!("one!"),
        5 => println!("five!"),
        _ => (),
    }

    let some_u8_value: Option<u8> = Some(3);
    if let Some(3) = some_u8_value {
        println!("Yes it was three");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Luck Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Oh its from {:?}", state);
            25
        }
    }
}
