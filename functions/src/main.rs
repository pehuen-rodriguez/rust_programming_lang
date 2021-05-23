fn main() {
    println!("Hello, world!");
    another_function(5, 9);
}

fn another_function(x: i32, y: i32) {
    println!("Another function with {}", x * y);
    println!("Five function with a sigle expression returns: {}", five());
}

fn five() -> i32 {
    5
}