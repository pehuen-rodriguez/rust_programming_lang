use std::io;

fn main() {
    println!("Convert Farenheit to Celcius and viceversa.");

    loop {
        println!("What units will your input be?");
        println!("Please enter \"C\" for Celcius and \"F\" for Farenheit");

        let mut units: String = String::new();

        io::stdin()
            .read_line(&mut units)
            .expect("Failed to read line");

        let units = match units.trim() {
            "C" => "C",
            "F" => "F",
            _ => {
                println!("Should have been \"C\" for Celcius or \"F\" for Farenheit");
                continue;
            }
        };
    }
}
