struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle{width: 100, height: 150};

    println!(
        "The area of the rectangle equals {}",
        calculate_area(&rect)
    );
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
