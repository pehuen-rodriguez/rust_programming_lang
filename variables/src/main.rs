fn main() {
    const MAX_POINTS: u32 = 100_000;

    let mut x = 5;
    println!("Te value of x is: {}", x);

    x = 6;
    println!("Te value of x is: {}", x);
    println!("Te value of MAX_POINTS is: {}", MAX_POINTS);

    let shadow = 5;
    let shadow = shadow + 10;

    println!("Te value of the sadowed thing is: {}", shadow);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces changed the value type: {}", spaces);
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Explicit type example: {}", guess);

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (12_000, 14.4, 1);
    let (x, y, z) = tup;

    println!(
        "Several values: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        sum, difference, product, quotient, remainder, remainder, heart_eyed_cat, x, y, z
    );
    println!("And also: {}", tup.2);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("third position is {}", months[2]);
    another_function();
}

fn another_function() {
    println!("from another function");
}
