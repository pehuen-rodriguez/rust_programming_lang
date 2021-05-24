fn main() {
    let rect = (100, 150);

    println!(
        "The area of the rectangle equals {}",
        calculate_area(rect)
    );
}

fn calculate_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
