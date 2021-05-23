fn main() {
    let mut index = 0;

    while index != 3 {
        index += 1;
        println!("{} Hello, world!", index);
    }

    let silly_array = [10, 20, 30];
    index = 0;

    while index < 3 {
        println!("Val in array was {}", silly_array[index]);
        index += 1;
    }

    for element in silly_array.iter() {
        println!("Val was with for: {}", element);
    }

    for count in (1..4).rev() {
        println!("Count in reverse: {}", count);
    }
}
