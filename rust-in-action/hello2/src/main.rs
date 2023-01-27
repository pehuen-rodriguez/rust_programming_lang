use std::ops::{Add};

fn main() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }

    let a: i32 = 42;
    let b = &a;
    println!("{}", a.add(*b));
}
