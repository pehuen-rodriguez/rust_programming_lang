fn main() {
    let diez = 10;
    let once = onceize(diez);

    println!(
        "I have one {}. You have two {}",
        diez,
     once,
    )
}

fn onceize(diez: i32) -> i32 {
    diez + 1
}
