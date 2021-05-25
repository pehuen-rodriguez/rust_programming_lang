pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules {
                println!("I was nested!");
            }
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
    let green = TrafficLights::Green;
    let green = Green;
    let cat = Cat;
}

enum TrafficLights {
    Green,
    Yellow,
    Red,
}

enum Pets {
    Cat,
    Dog,
    Fish,
}

use TrafficLights::{Red, Green, Yellow};
use Pets::*;
