fn main() {
    let number = 3;

    if number > 5 {
        println!("Number was bigger than 5");
    } else {
        println!("Number was bigger than 5");
    }
    
    let always_true = true;
    let five = if always_true {
        5
    } else {
        6
    };

    println!("So its five: {}", five);
}
