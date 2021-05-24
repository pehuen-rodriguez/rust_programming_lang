fn main() {
    let width = 100;
    let height = 150;

    println!(
        "The area of the rectangle equals {}",
        calculate_area(width, height)
    );
}

fn calculate_area(width: i32, height: i32) -> i32 {
    width * height
}
