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
}
