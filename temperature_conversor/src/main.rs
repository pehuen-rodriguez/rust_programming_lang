use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {
    println!("Welcome! You can convert Farenheit to Celcius and viceversa.");
    let one_second = time::Duration::from_secs(1);
    thread::sleep(one_second);

    loop {
        println!("What units will your input be? Please enter \"C\" for Celcius and \"F\" for Farenheit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut units: String = String::new();

        io::stdin()
            .read_line(&mut units)
            .expect("Failed to read line");

        let units = match units.to_uppercase().trim() {
            "C" => "C",
            "F" => "F",
            "Q!" => break,
            _ => {
                println!("Should have been \"C\" for Celcius or \"F\" for Farenheit");
                println!("");
                continue;
            }
        };

        println!("Cool! Now give us the temp and we'll provide the conversion back 😊");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut temp_value: String = String::new();

        io::stdin()
            .read_line(&mut temp_value)
            .expect("Failed to read line");

        let temp_value: f32 = match temp_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Gimmme a number 😻!");
                continue;
            }
        };

        match units {
            "F" => println!("Ok, it's {}º Celcius", (temp_value - 32.0) / 1.8),
            "C" => println!("Ok, it's {}º Farenheit", (temp_value * 1.8) + 32.0),
            _ => {
                println!("OMG! There was an error");
                continue;
            }
        };

        thread::sleep(one_second);
        
        println!("");
        println!("Awesome! Keep converting or enter q! to quit");

        thread::sleep(one_second);
    }
}
